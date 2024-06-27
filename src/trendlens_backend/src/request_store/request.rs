use crate::remote_exchanges::okx::api::GetInstrumentsRequest;
use candid::CandidType;
use serde::{Deserialize, Serialize};

// this should be generic but right now its based on concrete types, easy to replace later

#[derive(Debug, Serialize, Deserialize, Default, Clone, CandidType)]
pub enum Request {
    #[default]
    Empty,
    GetInstruments(GetInstrumentsRequest),
}
