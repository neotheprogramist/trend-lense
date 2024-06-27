use serde::{Deserialize, Serialize};
use crate::remote_exchanges::okx::api::{GetInstrumentsRequest, IndexCandleStickRequest};

// this should be generic but right now its based on concrete types, easy to replace later

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub enum Request {
    #[default]
    Empty,
    GetInstruments(GetInstrumentsRequest),
    GetCandles(IndexCandleStickRequest),
}
