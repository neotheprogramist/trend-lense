use crate::pair::Pair;
use chain_data::ChainData;
use chain_data::TimestampBased;
use exchange::Candle;
use exchange::Exchange;
use remote_exchanges::okx::Okx;
use remote_exchanges::ExternalProvider;

mod api_client;
mod chain_data;
mod exchange;
mod pair;
mod remote_exchanges;
mod storable_wrapper;

#[ic_cdk::query]
fn __get_candid_interface_tmp_hack() -> String {
    include_str!("../trendlens_backend.did").to_string()
}

/// considering that stop > start
fn first_timestamp_to_fetch(start: u64, stop: u64, current: u64) -> Option<std::ops::Range<u64>> {
    if start <= current && stop <= current {
        return None;
    }

    Some(current..stop)
}

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

    let exchange = match exchange {
        Exchange::Okx => Okx::default(),
    };

    let mut stored_exchange_data = Okx::get_mut_chain_data();

    // first call is where the storing candles starts
    let last_candle_timestamp = stored_exchange_data
        .candles
        .last_timestamp()
        .unwrap_or(start_timestamp);

    let range_to_fetch =
        first_timestamp_to_fetch(start_timestamp, end_timestamp, last_candle_timestamp);

    // TODO: handle errors, return to caller
    let fetched_candles = match range_to_fetch {
        Some(ref range) => exchange
            .fetch_candles(pair, range.clone(), 1)
            .await
            .unwrap_or_default(),
        None => {
            return vec![];
        }
    };

    let range_to_get = if range_to_fetch.is_some() {
        start_timestamp..last_candle_timestamp
    } else {
        start_timestamp..end_timestamp
    };

    let stored_candles = stored_exchange_data.candles.get_between(range_to_get);

    stored_exchange_data
        .candles
        .insert_many(fetched_candles.clone());

    Okx::set_chain_data(stored_exchange_data);

    fetched_candles
        .into_iter()
        .chain(stored_candles.into_iter())
        .collect::<Vec<_>>()
}

ic_cdk::export_candid!();

#[cfg(test)]
mod tests {}
