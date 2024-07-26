use std::time::Duration;

use crate::{
    remote_exchanges::{
        okx::api, response::ApiResponseWrapper, ApiRequest, Authorize, ExchangeErrors,
        PathFormatter,
    },
    CALLBACK_RESPONSES, PROXY_CANISTER_ID,
};
use candid::{CandidType, Nat};
use ic_cdk::api::{
    call::RejectionCode,
    management_canister::http_request::{CanisterHttpRequestArgument, HttpHeader, HttpMethod},
};
use ic_cdk_timers::set_timer;
use ic_stable_structures::Storable;
use proxy_canister_types::{
    HttpHeader as ProxyHttpHeader, HttpMethod as ProxyHttpMethod, HttpRequest,
    HttpRequestEndpointArgs, HttpRequestEndpointResult, HttpResult,
};
use thiserror::Error;

async fn sleep(duration: Duration) {
    let (sender, receiver) = oneshot::channel();

    set_timer(duration, move || {
        let _ = sender.send(());
    });

    let _ = receiver.await;
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
    fn headers(&self, host: &'static str) -> Vec<HttpHeader> {
        vec![
            HttpHeader {
                name: "Host".to_string(),
                value: format!("{host}:443"),
            },
            HttpHeader {
                name: "User-Agent".to_string(),
                value: "trend_lense_backend".to_string(),
            },
            HttpHeader {
                name: "Content-Type".to_string(),
                value: "application/json".to_string(),
            },
            HttpHeader {
                name: "Idempotency-Key".to_string(),
                value: "jaca".to_string(),
            },
        ]
    }

    fn required_cycles_for_request(arg: &CanisterHttpRequestArgument) -> u128 {
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

        ic_cdk::println!("qs {}", qs);

        let path = if R::PATH_PARAMS {
            request.get_path(R::URI)
        } else {
            R::URI.to_string()
        };

        ic_cdk::println!("path {}", path);

        let api_url = format!("https://{}/{}{}", R::HOST, path, qs);

        ic_cdk::println!("{}", api_url);

        let auth_headers = if let Some(a) = auth {
            a.get_auth_headers()
        } else {
            vec![]
        };

        ic_cdk::println!("{:?}", auth_headers);

        let body = R::BODY.then(|| request.to_body());

        ic_cdk::println!("{:?}", body);

        let headers = [auth_headers, self.headers(R::HOST)]
            .concat()
            .into_iter()
            .map(|h| ProxyHttpHeader {
                name: h.name,
                value: h.value,
            })
            .collect::<Vec<_>>();

        let request = HttpRequest {
            url: api_url,
            method: match R::METHOD {
                HttpMethod::GET => ProxyHttpMethod::GET,
                HttpMethod::POST => ProxyHttpMethod::POST,
                HttpMethod::HEAD => ProxyHttpMethod::HEAD,
            },
            headers,
            body: body.and_then(|b| Some(b.to_bytes().to_vec())),
        };

        let proxy_canister_id = PROXY_CANISTER_ID.with(|id| id.borrow().clone());
        let res: Result<(HttpRequestEndpointResult,), _> = ic_cdk::call(
            proxy_canister_id,
            "http_request",
            (HttpRequestEndpointArgs {
                request,
                timeout_ms: Some(10_000),
                callback_method_name: Some("http_response_callback".to_string()),
            },),
        )
        .await;

        let http_request_code = match res {
            Ok((http_res,)) => http_res.map_err(|_p| ExchangeErrors::Proxy)?,
            Err(_) => return Err(ExchangeErrors::Proxy),
        };

        let result = loop {
            match CALLBACK_RESPONSES
                .with(|responses| responses.borrow_mut().remove(&http_request_code))
            {
                Some(r) => {
                    break r;
                }

                None => {
                    sleep(Duration::from_secs(1)).await;
                    continue;
                }
            }
        };

        ic_cdk::println!("{:?}", result);

        match result {
            HttpResult::Success(response) => {
                ic_cdk::println!(
                    "{:?}",
                    String::from_utf8(response.body.clone()).expect("conversion failed")
                );

                if response.status != Nat::from(200u32) {
                    return Err(ApiClientErrors::Http {
                        status: response.status,
                        body: String::from_utf8(response.body).expect("conversion failed"),
                    }
                    .into());
                }

                let body: String = String::from_utf8(response.body).expect("conversion failed");
                let deserialized_response: W = serde_json::from_str(&body).map_err(|e| {
                    ExchangeErrors::DeserializationFailed {
                        message: e.to_string(),
                    }
                })?;

                deserialized_response.extract_response()
            }
            HttpResult::Failure(_) => return Err(ExchangeErrors::Proxy),
        }
    }
}
