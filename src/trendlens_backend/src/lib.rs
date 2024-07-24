use std::str::FromStr;

use crate::pair::Pair;
use api_store::{ApiData, ApiStore};
use chain_data::{ExchangeData, TimestampBased};
use exchange::{Candle, Exchange, ExchangeImpl, TimeVolume};
use ic_cdk::{query, update};
use instruments::save_instruments;
use remote_exchanges::{
    coinbase::{Coinbase, CoinbaseAuth},
    okx::{api::InstrumentType, auth::OkxAuth, Okx},
    request::{GeneralPostOrderRequest, OrderSide, OrderType, TradeMode},
    ExchangeErrors, UserData,
};
use request_store::{
    request::{Request, Response},
    Instruction, SignableInstruction, Transaction, TransactionStore,
};
use storable_wrapper::StorableWrapper;
use volume_store::{VolumesStore, VOLUME_STORE};

mod api_client;
mod api_store;
mod chain_data;
mod exchange;
mod instruments;
mod memory;
mod pair;
mod remote_exchanges;
mod request_store;
mod storable_wrapper;
mod volume_store;

#[ic_cdk::query]
fn __get_candid_interface_tmp_hack() -> String {
    include_str!("../trendlens_backend.did").to_string()
}

/// considering that stop > start
fn get_range_to_fetch(stop: u64, current: u64) -> Option<std::ops::Range<u64>> {
    (stop > current).then_some(current..stop)
}

// TODO: rename or get rid off
#[ic_cdk::query]
fn get_last_timestamp(exchange: Exchange, pair: String) -> Option<u64> {
    let exchange_impl = ExchangeImpl::new(exchange);

    let pair = Pair::from_str(&pair).expect("invalid pair");

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

#[ic_cdk::update]
fn add_transaction(instruction: Vec<Instruction>) -> (u32, Transaction) {
    let identity = ic_cdk::caller();

    let instructions = instruction
        .into_iter()
        .map(|i| {
            let exchange_impl = ExchangeImpl::new(i.exchange);
            let signature = exchange_impl.get_signature_string(&i.request);

            SignableInstruction {
                instruction: i,
                signature,
                executed: false,
            }
        })
        .collect::<Vec<_>>();

    let index = TransactionStore::add_transaction(&identity, instructions);
    let tx = TransactionStore::get_transaction(&identity, index).expect("missing tx");

    (index, tx)
}

#[ic_cdk::update]
fn delete_transaction(index: u32) {
    let identity = ic_cdk::caller();

    TransactionStore::delete_transaction(&identity, index);
}

#[ic_cdk::query]
fn get_transaction(index: u32) -> Option<Transaction> {
    let identity = ic_cdk::caller();

    TransactionStore::get_transaction(&identity, index)
}

#[ic_cdk::query]
fn get_transactions() -> Option<Vec<(u32, Transaction)>> {
    let identity = ic_cdk::caller();

    TransactionStore::get_transactions(&identity)
}

#[ic_cdk::update]
async fn refresh_instruments(
    exchange: Exchange,
    instrument_type: InstrumentType,
) -> Result<bool, ExchangeErrors> {
    let exchange_impl = ExchangeImpl::new(exchange);
    let instruments = exchange_impl.refresh_instruments(&instrument_type).await?;
    save_instruments(exchange, instrument_type, instruments);
    Ok(true)
}

#[ic_cdk::query]
fn get_instruments(exchange: Exchange, instrument_type: InstrumentType) -> Vec<Pair> {
    let exchange_impl = ExchangeImpl::new(exchange);

    exchange_impl.get_pairs(instrument_type)
}

#[ic_cdk::update]
async fn get_orderbook(exchange: Exchange, pair: String) -> f64 {
    let exchange_impl = ExchangeImpl::new(exchange);
    let pair = Pair::from_str(&pair).expect("invalid pair");
    let volume = exchange_impl
        .get_market_depth(&pair, &OrderSide::Buy, 50)
        .await;

    volume
}

#[ic_cdk::update]
async fn run_transaction(
    index: u32,
    signature: Vec<String>,
    timestamp_utc: String,
    timestamp: u64,
) -> Result<Vec<Response>, ExchangeErrors> {
    let identity = ic_cdk::caller();
    let tx = TransactionStore::get_transaction(&identity, index).expect("missing transaction");

    ic_cdk::println!("{:?}", tx);

    let mut responses = vec![];
    let mut done_instructions = vec![];

    for (i, signature) in tx.iter().zip(signature.iter()) {
        let api_info =
            ApiStore::get_by_api(&identity, &i.instruction.api_key).expect("api info not found");

        let exchange: Box<dyn UserData> = match i.instruction.exchange {
            Exchange::Okx => Box::new(Okx::with_auth(OkxAuth {
                api_key: i.instruction.api_key.clone(),
                passphrase: api_info.passphrase.unwrap(),
                timestamp: timestamp_utc.clone(),
                signature: signature.clone(),
            })),
            Exchange::Coinbase => Box::new(Coinbase::with_auth(CoinbaseAuth {
                api_key: i.instruction.api_key.clone(),
                passphrase: api_info.passphrase.unwrap(),
                signature: signature.clone(),
                timestamp,
            })),
        };

        let response = match i.instruction.request.clone() {
            Request::Empty => {
                panic!("Empty request")
            }
            Request::Instruments(instruments) => exchange.get_instruments(instruments).await?,
            Request::Balances(balance) => exchange.get_balance(balance).await?,
            Request::PostOrder(order) => exchange.post_order(order).await?,
            Request::OrdersList(orders_request) => match orders_request.pending {
                true => exchange.get_pending_orders(orders_request).await?,
                false => exchange.get_done_orders(orders_request).await?,
            },
        };

        ic_cdk::println!("{:?}", response);

        let done_ix = SignableInstruction {
            instruction: i.instruction.clone(),
            signature: i.signature.clone(),
            executed: true,
        };
        done_instructions.push(done_ix);
        responses.push(response);
    }

    TransactionStore::delete_transaction(&identity, index);
    TransactionStore::add_transaction(&identity, done_instructions);

    Ok(responses)
}

#[ic_cdk::init]
fn init() {
    let btc_usd_pair: Pair = Pair::from_str("btc-usd").expect("invalid pair");
    let exchange = ExchangeImpl::new(Exchange::Okx);
    exchange.set_data(btc_usd_pair, StorableWrapper(ExchangeData::default()));

    let btc_eur_pair: Pair = Pair::from_str("btc-eur").expect("invalid pair");
    exchange.set_data(btc_eur_pair, StorableWrapper(ExchangeData::default()));
}

#[ic_cdk::update]
fn initialize_pair(pair: String, exchange: Exchange) {
    let pair: Pair = Pair::from_str(&pair).expect("invalid pair");
    let exchange = ExchangeImpl::new(exchange);
    exchange.set_data(pair.clone(), StorableWrapper(ExchangeData::default()));
}

#[ic_cdk::update]
async fn split_transaction(
    keys: Vec<ApiData>,
    pair: String,
    order_side: OrderSide,
    size: f64,
    price_limit: u32,
    volume_ratios: Vec<f64>,
    ratios_weights: u32,
) -> Result<(u32, Transaction), ExchangeErrors> {
    let identity = ic_cdk::caller();
    let pair = Pair::from_str(&pair).expect("invalid pair");

    let mut volumes: Vec<f64> = vec![];

    for k in keys.iter() {
        let key =
            ApiStore::get_by_api(&identity, &k.api_key).ok_or(ExchangeErrors::MissingApiKey)?;
        let exchange = ExchangeImpl::new(key.exchange);

        volumes.push(
            exchange
                .get_market_depth(&pair, &order_side, price_limit)
                .await,
        )
    }

    let weights = volumes
        .iter()
        .zip(volume_ratios.iter())
        .map(|(v, r)| {
            ((v / volumes.iter().sum::<f64>()) + r * ratios_weights as f64)
                / (1.0 + ratios_weights as f64)
        })
        .collect::<Vec<_>>();

    let trade_cuts = weights.iter().map(|w| w * size).collect::<Vec<_>>();

    ic_cdk::println!("Trade cuts: {:?}", trade_cuts);

    let instructions: Vec<Result<Instruction, ExchangeErrors>> = keys
        .iter()
        .zip(trade_cuts.iter())
        .map(|(k, c)| {
            Ok(Instruction {
                api_key: k.api_key.clone(),
                exchange: k.exchange,
                request: Request::PostOrder(GeneralPostOrderRequest {
                    instrument_id: pair.clone(),
                    order_type: OrderType::Market,
                    side: order_side.clone(),
                    size: *c,
                    trade_mode: TradeMode::Cash,
                    margin_currency: None,
                    order_price: None,
                    position_side: None,
                }),
            })
        })
        .collect();

    let instructions = instructions.into_iter().collect::<Result<Vec<_>, _>>()?;

    Ok(add_transaction(instructions))
}

// TODO: split this function into smaller ones
// TODO: handle errors, return to caller
#[ic_cdk::update]
async fn pull_candles(
    pair: String,
    exchange: Exchange,
    start_timestamp: u64,
    end_timestamp: u64,
) -> Result<Vec<Candle>, ExchangeErrors> {
    if start_timestamp >= end_timestamp {
        return Err(ExchangeErrors::InvalidTimestamps);
    }

    let pair = Pair::from_str(&pair).expect("invalid pair");
    let exchange = ExchangeImpl::new(exchange);
    let mut exchange_data = exchange
        .get_data(pair.clone())
        .ok_or(ExchangeErrors::MissingPair)?;

    let last_candle_timestamp = exchange_data
        .candles
        .last_timestamp()
        .unwrap_or(start_timestamp);
    ic_cdk::println!("Last candle timestamp: {}", last_candle_timestamp);

    let range_to_fetch = get_range_to_fetch(end_timestamp, last_candle_timestamp);
    ic_cdk::println!("Range to fetch: {:?}", range_to_fetch);

    // !!!! hardcoded interval
    let fetched_candles = match range_to_fetch {
        Some(ref range) => exchange.fetch_index_candles(&pair, range.clone(), 1).await?,
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

#[query]
fn get_volumes(exchange: Exchange, pair: String, start: u64) -> Option<Vec<TimeVolume>> {
    let pair = Pair::from_str(&pair).expect("invalid pair");

    VOLUME_STORE.with_borrow(|v| {
        let store = v.get(&(exchange, pair))?;

        Some(store.get_between(start..std::u64::MAX))
    })
}

#[update]
fn initialize_volume_store(exchange: Exchange, pair: String, timestamp: u64) {
    let pair = Pair::from_str(&pair).expect("invalid pair");

    VOLUME_STORE.with_borrow_mut(|v| {
        let mut store = v.get(&(exchange, pair.clone())).unwrap_or_else(|| {
            let store = VolumesStore::default();
            v.insert((exchange, pair.clone()), StorableWrapper(store));
            v.get(&(exchange, pair.clone())).unwrap()
        });

        store.insert(
            timestamp,
            TimeVolume {
                timestamp,
                volume: 0.0,
            },
        );

        v.insert((exchange, pair), store);
    });
}

#[update]
async fn pull_volumes(
    exchange: Exchange,
    pair: String,
    end: u64,
) -> Result<Vec<TimeVolume>, ExchangeErrors> {
    let pair = Pair::from_str(&pair).expect("invalid pair");

    let timestamp = VOLUME_STORE.with_borrow(|v| -> Result<u64, ExchangeErrors> {
        let store = v
            .get(&(exchange, pair.clone()))
            .ok_or(ExchangeErrors::MissingPair)?;

        store
            .last_timestamp()
            .ok_or(ExchangeErrors::MissingTimestamp)
    })?;

    let range_to_fetch = get_range_to_fetch(end, timestamp);

    let fetched_volumes = match range_to_fetch {
        Some(ref range) => {
            let exchange_impl = ExchangeImpl::new(exchange);
            exchange_impl.get_taker_volume(&pair, range.clone()).await?
        }
        None => vec![],
    };

    VOLUME_STORE.with_borrow_mut(|v| {
        let mut store = v.get(&(exchange, pair.clone())).unwrap();

        store.insert_many(fetched_volumes.clone());

        v.insert((exchange, pair), store);
    });

    Ok(fetched_volumes)
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
