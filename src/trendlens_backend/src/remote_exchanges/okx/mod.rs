use crate::api_client::ApiClient;
use crate::chain_data::ChainData;
use crate::exchange::Exchange;
use crate::Pair;
use auth::OkxAuth;

pub mod api;
pub mod auth;
pub mod open;
pub mod response;
pub mod user;

#[derive(Default)]
pub struct Okx {
    auth: Option<OkxAuth>,
    api_client: ApiClient,
}


impl Okx {
    /// gets interval string from u32 in minutes
    fn interval_string(interval: u32) -> String {
        match interval {
            0..=1 => "1m",
            2..=3 => "3m",
            4..=u32::MAX => "5m",
        }
        .to_string()
    }

    /// gets index name from global pair enum
    fn index_name(pair: Pair) -> Option<String> {
        match pair {
            Pair::BtcUsd => Some("BTC-USD".to_string()),
            Pair::EthUsd => Some("ETH-USD".to_string()),
        }
    }
}

impl ChainData for Okx {
    fn key(&self) -> Exchange {
        Exchange::Okx
    }
}

#[cfg(test)]
mod test_okx_helpers {
    use super::*;
    use crate::Pair;

    #[test]
    fn test_okx_interval_string() {
        assert_eq!(Okx::interval_string(0), "1m");
        assert_eq!(Okx::interval_string(1), "1m");
        assert_eq!(Okx::interval_string(2), "3m");
        assert_eq!(Okx::interval_string(3), "3m");
        assert_eq!(Okx::interval_string(4), "5m");
        assert_eq!(Okx::interval_string(5), "5m");
        assert_eq!(Okx::interval_string(50), "5m");
        assert_eq!(Okx::interval_string(u32::MAX), "5m");
    }

    #[test]
    fn test_okx_index_name() {
        assert_eq!(Okx::index_name(Pair::BtcUsd), Some("BTC-USD".to_string()));
        assert_eq!(Okx::index_name(Pair::EthUsd), Some("ETH-USD".to_string()));
    }
}
