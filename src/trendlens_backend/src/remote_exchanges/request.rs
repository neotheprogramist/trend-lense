use candid::CandidType;
use serde::{Deserialize, Serialize};

use super::okx::api::InstrumentType;

#[derive(Debug, Clone, Deserialize, Serialize, CandidType)]
pub struct GeneralInstrumentsRequest {
    pub instrument_type: InstrumentType,
    pub instrument_id: Option<String>,
}
