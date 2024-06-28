use crate::pair::Pair;
use api_store::ApiData;
use api_store::ApiStore;
use chain_data::ChainData;
use chain_data::TimestampBased;

use exchange::Candle;
use exchange::Exchange;
use remote_exchanges::coinbase::Coinbase;
use remote_exchanges::okx::auth::OkxAuth;
use remote_exchanges::okx::Okx;

use remote_exchanges::ExchangeErrors;
use remote_exchanges::UpdateExchange;
use remote_exchanges::UserData;
use request_store::request::Request;
use request_store::request::Response;
use request_store::ExchangeRequestInfo;
use request_store::RequestStore;
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

mod api_client;
mod api_store;
mod chain_data;
mod exchange;
mod memory;
mod pair;
mod remote_exchanges;
mod request_store;
mod storable_wrapper;

#[ic_cdk::query]
fn __get_candid_interface_tmp_hack() -> String {
    include_str!("../trendlens_backend.did").to_string()
}

/// considering that stop > start
fn get_range_to_fetch(stop: u64, current: u64) -> Option<std::ops::Range<u64>> {
    if stop <= current {
        return None;
    }

    Some(current..stop)
}

#[ic_cdk::init]
fn init() {
    ic_cdk::println!("Initializing TrendLense backend canister");
    Okx::default().init();
}

#[ic_cdk::query]
fn get_last_timestamp(exchange: Exchange) -> u64 {
    let exchange = match exchange {
        Exchange::Okx => Okx::default(),
        Exchange::Coinbase => unimplemented!(),
    };

    let stored_exchange_data = exchange.get_mut_chain_data();
    stored_exchange_data.candles.last_timestamp().unwrap_or(0)
}

#[ic_cdk::update]
fn register_api_key(exchange_api: ApiData) {
    let principal = ic_cdk::caller();

    ApiStore::register_key(&principal, exchange_api);
}

#[ic_cdk::update]
fn remove_api_key(api_key: String) {
    let principal = ic_cdk::caller();

    ApiStore::remove_key(&principal, &api_key)
}

#[ic_cdk::query]
fn get_api_keys() -> Vec<ApiData> {
    let principal = ic_cdk::caller();

    ApiStore::get_all_keys(&principal).unwrap_or_default()
}

#[ic_cdk::update]
fn initialize_request(request: ExchangeRequestInfo) -> u8 {
    let identity = ic_cdk::caller();

    RequestStore::add_request(&identity, request)
}

#[ic_cdk::query]
fn get_request(index: u8) -> Option<ExchangeRequestInfo> {
    let identity = ic_cdk::caller();

    RequestStore::get_request(&identity, index)
}

#[ic_cdk::update]
async fn run_request(
    index: u8,
    signature: String,
    timestamp: i64,
) -> Result<Response, ExchangeErrors> {
    let identity = ic_cdk::caller();
    let exchange_request = RequestStore::get_request(&identity, index).expect("missing request");
    let api_info = ApiStore::get_by_api(&identity, &exchange_request.api_key.as_str())
        .expect("api info not found");

    let exchange: Box<dyn UserData> = match exchange_request.exchange {
        Exchange::Okx => {
            let time = OffsetDateTime::from_unix_timestamp(timestamp).expect("invalid timestamp");
            let formatted_timestamp = time.format(&Rfc3339).expect("formatting failed");

            Box::new(Okx::with_auth(OkxAuth {
                api_key: exchange_request.api_key,
                passphrase: api_info.passphrase.unwrap(),
                timestamp: formatted_timestamp,
                signature,
            }))
        }
        Exchange::Coinbase => Box::new(Coinbase::default()),
    };

    let response = match exchange_request.request {
        Request::Empty => {
            panic!()
        }
        Request::Instruments(instruments) => exchange.get_instruments(instruments).await?,
    };

    Ok(response)
}

// TODO: split this function into smaller ones
// TODO: handle errors, return to caller
#[ic_cdk::update]
async fn pull_candles(
    pair: Pair,
    exchange: Exchange,
    start_timestamp: u64,
    end_timestamp: u64,
) -> Vec<Candle> {
    if start_timestamp >= end_timestamp {
        return vec![];
    }

    let exchange: Box<dyn UpdateExchange> = match exchange {
        Exchange::Okx => Box::new(Okx::default()),
        Exchange::Coinbase => Box::new(Coinbase::default()),
    };

    let mut stored_exchange_data = exchange.get_mut_chain_data();

    // first call is where the storing candles starts
    let last_candle_timestamp = stored_exchange_data
        .candles
        .last_timestamp()
        .unwrap_or(start_timestamp);

    ic_cdk::println!("Last candle timestamp: {}", last_candle_timestamp);

    let range_to_fetch = get_range_to_fetch(end_timestamp, last_candle_timestamp);

    ic_cdk::println!("Range to fetch: {:?}", range_to_fetch);

    // TODO: handle errors, return to caller
    let fetched_candles = match range_to_fetch {
        Some(ref range) => exchange
            .fetch_candles(pair, range.clone(), 1)
            .await
            .unwrap_or_default(),
        None => {
            vec![]
        }
    };

    ic_cdk::println!("Fetched candles: {:?}", fetched_candles.len());

    let range_to_get = if range_to_fetch.is_some() {
        if start_timestamp <= last_candle_timestamp {
            Some(start_timestamp..last_candle_timestamp)
        } else {
            None
        }
    } else {
        Some(start_timestamp..end_timestamp)
    };

    ic_cdk::println!("Range to get: {:?}", range_to_get);

    let stored_candles = if let Some(range) = range_to_get {
        stored_exchange_data.candles.get_between(range)
    } else {
        vec![]
    };

    stored_exchange_data
        .candles
        .insert_many(fetched_candles.clone());

    exchange.set_chain_data(stored_exchange_data);

    fetched_candles
        .into_iter()
        .chain(stored_candles.into_iter())
        .collect::<Vec<_>>()
}

ic_cdk::export_candid!();

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_timestamp_to_fetch() {
        let stop = 10;
        let current = 5;

        assert_eq!(get_range_to_fetch(stop, current), Some(5..10));
    }

    #[test]
    fn test_first_timestamp_to_fetch_no_fetch() {
        let stop = 10;
        let current = 15;

        assert_eq!(get_range_to_fetch(stop, current), None);
    }
}
