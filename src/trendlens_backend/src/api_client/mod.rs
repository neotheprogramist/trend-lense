use crate::remote_exchanges::{okx::response::ApiResponse, ApiRequest, Authorizable};
use candid::Nat;
use ic_cdk::api::{
    call::RejectionCode,
    management_canister::http_request::{http_request, CanisterHttpRequestArgument, HttpHeader},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiClientErrors {
    #[error("http call failed with status {status}")]
    Http { status: Nat },
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
                value: host.to_string(),
            },
            HttpHeader {
                name: "User-Agent".to_string(),
                value: "trend_lense_backend".to_string(),
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

    pub async fn call<R, A>(&self, request: R, auth: Option<&A>) -> Result<R::Response, ApiClientErrors>
    where
        R: ApiRequest,
        A: Authorizable,
    {
        let api_url = format!(
            "https://{}/{}{}",
            R::HOST,
            R::URI,
            format!("?{}", request.to_query_string())
        );

        ic_cdk::println!("{}", api_url);

        let auth_headers = if let Some(a) = auth {
            a.get_auth_headers()
        } else {
            vec![]
        };


        let request = CanisterHttpRequestArgument {
            url: api_url,
            method: R::METHOD,
            headers: [auth_headers, self.headers(R::HOST)].concat(),
            body: None,
            ..Default::default()
        };

        let required_cycles = Self::required_cycles_for_request(&request);

        match http_request(request, required_cycles).await {
            Ok((response,)) => {
                if response.status != Nat::from(200u32) {
                    return Err(ApiClientErrors::Http {
                        status: response.status,
                    });
                }

                let body = String::from_utf8(response.body).unwrap();
                let deserialized_response: ApiResponse<R::Response> =
                    serde_json::from_str(&body).unwrap();
                // TODO: handle errors and messages

                return Ok(deserialized_response.data);
            }
            Err(err) => {
                return Err(ApiClientErrors::Reject {
                    message: err.1,
                    code: err.0,
                })
            }
        }
    }
}
