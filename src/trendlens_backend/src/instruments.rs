use crate::{
    exchange::Exchange,
    memory::{Memory, MemoryLocation, MEMORY_MANAGER},
    remote_exchanges::{okx::api::InstrumentType, response::Instrument},
    storable_wrapper::StorableWrapper,
};
use ic_stable_structures::StableBTreeMap;
use std::cell::RefCell;

// right now vec for convenience, but it should be a map or separate tree entries
type ExchangeInstruments =
    StableBTreeMap<(Exchange, InstrumentType), StorableWrapper<Vec<Instrument>>, Memory>;

thread_local! {
  static EXCHANGE_INSTRUMENTS: RefCell<ExchangeInstruments> = RefCell::new(
    StableBTreeMap::init(
        MEMORY_MANAGER.with(|m| m.borrow().get(MemoryLocation::ExchangeInstruments.memory_id())),
    )
);
}

pub fn get_instruments(
    exchange: Exchange,
    instrument_type: InstrumentType,
) -> Option<Vec<Instrument>> {
    EXCHANGE_INSTRUMENTS.with(|e| {
        e.borrow()
            .get(&(exchange, instrument_type))
            .map(|i| i.clone())
    })
}

pub fn save_instruments(
    exchange: Exchange,
    instrument_type: InstrumentType,
    instruments: Vec<Instrument>,
) {
    EXCHANGE_INSTRUMENTS.with(|e| {
        e.borrow_mut()
            .insert((exchange, instrument_type), StorableWrapper(instruments))
    });
}
