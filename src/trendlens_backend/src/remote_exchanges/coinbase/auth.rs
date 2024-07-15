use super::Coinbase;
use crate::remote_exchanges::Authorize;
use ic_cdk::api::management_canister::http_request::HttpHeader;

pub struct CoinbaseAuth {
    pub api_key: String,
    pub passphrase: String,
    pub signature: String,
    pub timestamp: String,
}

impl Authorize for CoinbaseAuth {
    fn get_auth_headers(&self) -> Vec<HttpHeader> {
        vec![
            HttpHeader {
                name: "CB-ACCESS-KEY".to_string(),
                value: self.api_key.clone(),
            },
            HttpHeader {
                name: "CB-ACCESS-SIGN".to_string(),
                value: self.signature.clone(),
            },
            HttpHeader {
                name: "CB-ACCESS-TIMESTAMP".to_string(),
                value: self.timestamp.clone(),
            },
            HttpHeader {
                name: "CB-ACCESS-PASSPHRASE".to_string(),
                value: self.passphrase.clone(),
            },
            HttpHeader {
                name: "x-simulated-trading".to_string(),
                value: "1".to_string(),
            },
        ]
    }
}

impl Coinbase {
    pub fn with_auth(auth: CoinbaseAuth) -> Self {
        Self {
            auth: Some(auth),
            ..Default::default()
        }
    }
}
