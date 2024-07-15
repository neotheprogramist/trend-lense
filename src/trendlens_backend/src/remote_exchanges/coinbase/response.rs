use std::str::FromStr;

use crate::{pair::Pair, remote_exchanges::{
    okx::api::InstrumentType, response::{ApiResponseWrapper, Instrument}, ExchangeErrors
}};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ConcreteInstrument {
    pub display_name: String,
}

impl Into<Instrument> for ConcreteInstrument {
    fn into(self) -> Instrument {
        Instrument {
            instrument_id: Pair::from_str(self.display_name.as_str()).expect("Invalid pair"),
            instrument_type: InstrumentType::Spot,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct CoinbaseResponse<T> {
    pub data: T,
}

impl<R: DeserializeOwned> ApiResponseWrapper<R> for CoinbaseResponse<R> {
    fn extract_response(self) -> Result<R, ExchangeErrors> {
        Ok(self.data)
    }
}
