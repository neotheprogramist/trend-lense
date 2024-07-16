use crate::remote_exchanges::{
    okx::response::AccountInfo,
    request,
    response::{Instrument, OrderData},
};
use candid::CandidType;
use serde::{Deserialize, Serialize};

// this should be generic but right now its based on concrete types, easy to replace later

#[derive(Debug, Default, Clone, Deserialize, Serialize, CandidType)]
pub enum Request {
    #[default]
    Empty,
    Instruments(request::GeneralInstrumentsRequest),
    Balances(request::GeneralBalanceRequest),
    PostOrder(request::GeneralPostOrderRequest),
}

#[derive(Debug, Clone, Deserialize, CandidType)]
pub enum Response {
    Instruments(Vec<Instrument>),
    Balances(Vec<AccountInfo>),
    Order(OrderData),
}
