use candid::CandidType;
use serde::Deserialize;
use crate::pair::Pair;
use super::okx::api::InstrumentType;

#[derive(Deserialize, Debug, Clone, CandidType)]
pub struct Instrument {
    pub instrument_id: Pair,
    pub instrument_type: InstrumentType,
}
