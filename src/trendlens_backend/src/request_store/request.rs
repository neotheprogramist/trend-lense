use crate::remote_exchanges::okx::response::{AccountInfo, Instrument};
use candid::CandidType;
use serde::{Deserialize, Serialize};

// this should be generic but right now its based on concrete types, easy to replace later

#[derive(Debug, Default, Clone, Deserialize, Serialize, CandidType)]
pub enum Request {
    #[default]
    Empty,
    Instruments(crate::remote_exchanges::request::GeneralInstrumentsRequest),
    Balances(crate::remote_exchanges::request::GeneralBalanceRequest),
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, CandidType)]
pub struct In {
    pub a: u32,
}

#[derive(Debug, Clone, Deserialize, CandidType)]
pub enum Response {
    Instruments(Vec<Instrument>),
    Balances(AccountInfo),
}
