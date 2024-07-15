use super::{okx::api::InstrumentType, ExchangeErrors};
use crate::pair::Pair;
use candid::CandidType;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub trait ApiResponseWrapper<R: DeserializeOwned>: DeserializeOwned {
    fn extract_response(self) -> Result<R, ExchangeErrors>;
}

#[derive(Deserialize, Debug, Clone, CandidType, Serialize)]
pub struct Instrument {
    pub instrument_id: Pair,
    pub instrument_type: InstrumentType,
}

#[derive(Deserialize, Debug, Clone, CandidType, Serialize)]
pub struct OrderData {
    pub code: String,
}
