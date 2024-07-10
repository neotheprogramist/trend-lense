use candid::CandidType;
use serde::{Deserialize, Serialize};
use crate::pair::Pair;
use super::okx::api::InstrumentType;

#[derive(Deserialize, Debug, Clone, CandidType, Serialize)]
pub struct Instrument {
    pub instrument_id: Pair,
    pub instrument_type: InstrumentType,
}


#[derive(Deserialize, Debug, Clone, CandidType, Serialize)]
pub struct OrderData {
    pub code: String
}