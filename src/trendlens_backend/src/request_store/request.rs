use crate::remote_exchanges::okx::{api::GetInstrumentsRequest, response::Instrument};
use candid::CandidType;
use serde::{Deserialize, Serialize};

// this should be generic but right now its based on concrete types, easy to replace later

#[derive(Debug, Default, Clone, Serialize, Deserialize, CandidType)]
pub enum Request {
    #[default]
    Empty,
    Instruments(GetInstrumentsRequest),
}

#[derive(Debug, Clone, Deserialize, CandidType)]
pub enum Response {
    Instruments(Vec<Instrument>),
}
