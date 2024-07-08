use crate::pair::Pair;
use api_store::{ApiData, ApiStore};
use chain_data::TimestampBased;
use exchange::{Candle, Exchange, ExchangeImpl};
use remote_exchanges::{
    coinbase::Coinbase,
    okx::{auth::OkxAuth, Okx},
    ExchangeErrors, UserData,
};
use request_store::{
    request::{Request, Response},
    ExchangeRequestInfo, RequestStore,
};

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

// TODO: rename or get rid off
#[ic_cdk::query]
fn get_last_timestamp(exchange: Exchange, pair: Pair) -> Option<u64> {
    let exchange_impl = ExchangeImpl::new(exchange);

    exchange_impl.get_data(pair)?.candles.last_timestamp()
}

#[ic_cdk::update]
fn register_api_key(exchange_api: ApiData) -> bool {
    let principal = ic_cdk::caller();

    ApiStore::register_key(&principal, exchange_api);

    // There should be test if key is a valid key

    true
}

#[ic_cdk::update]
fn remove_api_key(api_key: String) -> Option<ApiData> {
    let principal = ic_cdk::caller();

    ApiStore::remove_key(&principal, &api_key)
}

#[ic_cdk::query]
fn get_request_signature_string(index: u8) -> String {
    let identity = ic_cdk::caller();

    let request_info = RequestStore::get_request(&identity, index).expect("missing request");

    let exchange_impl = ExchangeImpl::new(request_info.exchange);
    exchange_impl.get_signature_string(request_info.request)
}

#[ic_cdk::update]
fn initialize_request(request: ExchangeRequestInfo) -> u8 {
    let identity = ic_cdk::caller();

    RequestStore::add_request(&identity, request)
}

#[ic_cdk::update]
fn delete_request(index: u8) {
    let identity = ic_cdk::caller();

    RequestStore::delete_request(&identity, index);
}

#[ic_cdk::query]
fn get_request(index: u8) -> Option<ExchangeRequestInfo> {
    let identity = ic_cdk::caller();

    RequestStore::get_request(&identity, index)
}

#[ic_cdk::query]
fn get_requests() -> Vec<Option<ExchangeRequestInfo>> {
    let identity = ic_cdk::caller();

    let last_index = RequestStore::next_index(&identity);

    (0..last_index)
        .into_iter()
        .map(|i| RequestStore::get_request(&identity, i))
        .collect::<Vec<_>>()
}

#[ic_cdk::query]
fn get_pairs(exchange: Exchange) -> Vec<Pair> {
    let exchange_impl = ExchangeImpl::new(exchange);

    exchange_impl.get_pairs()
}

#[ic_cdk::update]
async fn run_request(
    index: u8,
    signature: String,
    timestamp_utc: String,
) -> Result<Response, ExchangeErrors> {
    let identity = ic_cdk::caller();
    let exchange_request = RequestStore::get_request(&identity, index).expect("missing request");

    ic_cdk::println!("{:?}", exchange_request);

    let api_info =
        ApiStore::get_by_api(&identity, &exchange_request.api_key).expect("api info not found");

    let exchange: Box<dyn UserData> = match exchange_request.exchange {
        Exchange::Okx => Box::new(Okx::with_auth(OkxAuth {
            api_key: exchange_request.api_key,
            passphrase: api_info.passphrase.unwrap(),
            timestamp: timestamp_utc,
            signature,
        })),
        Exchange::Coinbase => Box::new(Coinbase::default()),
    };

    let response = match exchange_request.request {
        Request::Empty => {
            panic!()
        }
        Request::Instruments(instruments) => exchange.get_instruments(instruments).await?,
        Request::Balances(balance) => exchange.get_balance(balance).await?,
        Request::PostOrder(order) => exchange.post_order(order).await?,
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
) -> Result<Vec<Candle>, ExchangeErrors> {
    if start_timestamp >= end_timestamp {
        return Err(ExchangeErrors::InvalidTimestamps);
    }

    let exchange = ExchangeImpl::new(exchange);
    let mut exchange_data = exchange.get_data(pair).ok_or(ExchangeErrors::MissingPair)?;

    let last_candle_timestamp = exchange_data
        .candles
        .last_timestamp()
        .unwrap_or(start_timestamp);
    ic_cdk::println!("Last candle timestamp: {}", last_candle_timestamp);

    let range_to_fetch = get_range_to_fetch(end_timestamp, last_candle_timestamp);
    ic_cdk::println!("Range to fetch: {:?}", range_to_fetch);

    // !!!! hardcoded interval
    let fetched_candles = match range_to_fetch {
        Some(ref range) => exchange.fetch_candles(pair, range.clone(), 1).await?,
        None => {
            vec![]
        }
    };

    ic_cdk::println!("Fetched candles: {:?}", fetched_candles.len());

    let range_to_get = match range_to_fetch {
        Some(_) if start_timestamp <= last_candle_timestamp => {
            Some(start_timestamp..last_candle_timestamp)
        }
        Some(_) => None,
        None => Some(start_timestamp..end_timestamp),
    };

    ic_cdk::println!("Range to get: {:?}", range_to_get);

    let stored_candles = range_to_get
        .and_then(|range| Some(exchange_data.candles.get_between(range)))
        .unwrap_or_default();

    exchange_data.candles.insert_many(fetched_candles.clone());
    exchange.set_data(pair, exchange_data);

    Ok(fetched_candles
        .into_iter()
        .chain(stored_candles.into_iter())
        .collect::<Vec<_>>())
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
