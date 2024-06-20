use crate::pair::Pair;
use exchange::Candle;
use exchange::Exchange;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap};
use remote_exchanges::okx::Okx;
use remote_exchanges::ExternalProvider;
use std::cell::RefCell;
use storable_wrapper::StorableWrapper;

mod api_client;
mod exchange;
mod pair;
mod remote_exchanges;
mod storable_wrapper;

const CANDLE_STORE_MEMORY_ID: MemoryId = MemoryId::new(1);
const NANOS_IN_SEC: u64 = 1_000_000_000;

type Timestamp = u64;
type PairIndex = u32;
type Memory = VirtualMemory<DefaultMemoryImpl>;
type CandleStore = StableBTreeMap<(PairIndex, Timestamp), StorableWrapper<Candle>, Memory>;

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
    RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static PAIRS: RefCell<CandleStore> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(CANDLE_STORE_MEMORY_ID)),
        )
    );
}

#[ic_cdk::query]
fn __get_candid_interface_tmp_hack() -> String {
    include_str!("../trendlens_backend.did").to_string()
}

#[ic_cdk::update]
async fn fetch_from_api(pair: Pair, exchange: Exchange) -> Vec<Candle> {
    let exchange = match exchange {
        Exchange::Okx => Okx::default(),
    };

    let now = ic_cdk::api::time() / NANOS_IN_SEC;
    let now_minus_five = 60 * 5;

    // TODO: handle errors, return to caller
    let candles_data = match exchange
        .get_candles(pair, now - now_minus_five, now, 5)
        .await
    {
        Ok(candles) => candles,
        Err(err) => {
            ic_cdk::println!("{:?}", err);
            return vec![];
        }
    };

    PAIRS.with(|f| {
        let mut borrowed = f.borrow_mut();

        candles_data.iter().for_each(|candle| {
            borrowed.insert(
                (pair.into(), candle.timestamp),
                StorableWrapper::<Candle>(candle.clone()),
            );
        });
    });

    return candles_data;
}

ic_cdk::export_candid!();