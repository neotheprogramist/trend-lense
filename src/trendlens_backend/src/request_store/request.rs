use crate::remote_exchanges::{
    request,
    response::{Balance, Order, Instrument, OrderData},
};
use candid::CandidType;
use serde::{Deserialize, Serialize};


#[derive(Debug, Default, Clone, Deserialize, Serialize, CandidType)]
pub enum Request {
    #[default]
    Empty,
    Instruments(request::GeneralInstrumentsRequest),
    Balances(request::GeneralBalanceRequest),
    PostOrder(request::GeneralPostOrderRequest),
    OrdersList(request::GeneralOrdersListRequest),
}

#[derive(Debug, Clone, Deserialize, CandidType)]
pub enum Response {
    Instruments(Vec<Instrument>),
    Balances(Vec<Balance>),
    Order(OrderData),
    OrdersInfo(Vec<Order>),
}
