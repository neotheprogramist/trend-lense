use crate::remote_exchanges::{
    response::ApiResponseWrapper, ApiRequest, Authorize, ExchangeErrors, PathFormatter,
};
use candid::{CandidType, Nat, Principal};
use ic_cdk::api::{
    call::RejectionCode,
    management_canister::http_request::{CanisterHttpRequestArgument, HttpHeader, HttpResponse},
};
use ic_stable_structures::Storable;
use std::{cell::RefCell, str::FromStr};
use thiserror::Error;

thread_local! {
    static PROXY_CANISTER_ID: RefCell<Principal> = RefCell::new(Principal::from_str("ahdfa-wyaaa-aaaal-ajpba-cai").unwrap());

    static IDEMPOTENCY_COUNTER: RefCell<u64> = RefCell::new(0);
}

#[derive(Error, Debug, CandidType)]
pub enum ApiClientErrors {
    #[error("http call failed with status {status} and body {body}")]
    Http { status: Nat, body: String },
    #[error("request rejected with code {code:?} and message {message}")]
    Reject {
        message: String,
        code: RejectionCode,
    },
}

#[derive(Default)]
pub struct ApiClient;

impl ApiClient {
    fn get_contract_headers(_host: &'static str) -> Vec<HttpHeader> {
        vec![
            HttpHeader {
                name: "User-Agent".to_string(),
                value: "trend_lense_backend".to_string(),
            },
            HttpHeader {
                name: "Content-Type".to_string(),
                value: "application/json".to_string(),
            },
            HttpHeader {
                name: "idempotency-key".to_string(),
                value: IDEMPOTENCY_COUNTER.with_borrow_mut(|c| {
                    let counter = *c;
                    *c += 1;
                    counter.to_string()
                }),
            },
        ]
    }

    fn get_required_cycles(arg: &CanisterHttpRequestArgument) -> u128 {
        let max_response_bytes = match arg.max_response_bytes {
            Some(ref n) => *n as u128,
            None => 2 * 1024 * 1024u128, // default 2MiB
        };
        let arg_raw = candid::utils::encode_args((arg,)).expect("Failed to encode arguments.");
        // The fee is for a 13-node subnet to demonstrate a typical usage.
        (3_000_000u128
            + 60_000u128 * 13
            + (arg_raw.len() as u128 + "http_request".len() as u128) * 400
            + max_response_bytes * 800)
            * 13
    }

    fn get_headers<A, R>(auth: Option<&A>) -> Vec<HttpHeader>
    where
        A: Authorize,
        R: ApiRequest,
    {
        let auth_headers = if let Some(a) = auth {
            a.get_auth_headers()
        } else {
            vec![]
        };

        let joined_headers = [auth_headers, Self::get_contract_headers(R::HOST)].concat();
        ic_cdk::println!("HEADERS: {:?}", joined_headers);

        joined_headers
    }

    pub async fn call<W, R, A>(
        &self,
        request: R,
        auth: Option<&A>,
    ) -> Result<R::Response, ExchangeErrors>
    where
        R: ApiRequest,
        A: Authorize,
        W: ApiResponseWrapper<R::Response>,
    {
        let qs = if R::BODY {
            "".to_string()
        } else {
            let qs = request.to_query_string();

            if qs == "" {
                "".to_string()
            } else {
                format!("?{}", qs)
            }
        };
        ic_cdk::println!("query: {}", qs);

        let path = if R::PATH_PARAMS {
            request.get_path(R::URI)
        } else {
            R::URI.to_string()
        };
        ic_cdk::println!("path: {}", path);

        let api_url = format!("https://{}/{}{}", R::HOST, path, qs);
        ic_cdk::println!("{}", api_url);

        let body = R::BODY.then(|| request.to_body());
        ic_cdk::println!("{:?}", body);

        let request = CanisterHttpRequestArgument {
            url: api_url,
            method: R::METHOD,
            headers: Self::get_headers::<_, R>(auth),
            body: body.and_then(|b| Some(b.to_bytes().to_vec())),
            ..Default::default()
        };

        let required_cycles = Self::get_required_cycles(&request);
        let (response,): (HttpResponse,) = ic_cdk::api::call::call_with_payment128(
            PROXY_CANISTER_ID.with_borrow(|id| id.clone()),
            "proxy_http_request",
            (request,),
            required_cycles,
        )
        .await
        .map_err(|(code, message)| ApiClientErrors::Reject { code, message })?;

        ic_cdk::println!(
            "{:?}",
            String::from_utf8(response.body.clone()).expect("conversion failed")
        );

        let body: String = String::from_utf8(response.body).expect("conversion failed");

        if response.status != Nat::from(200u32) {
            return Err(ApiClientErrors::Http {
                status: response.status,
                body,
            }
            .into());
        }

        let deserialized_response: W =
            serde_json::from_str(&body).map_err(|e| ExchangeErrors::DeserializationFailed {
                message: e.to_string(),
            })?;

        deserialized_response.extract_response()
    }
}
