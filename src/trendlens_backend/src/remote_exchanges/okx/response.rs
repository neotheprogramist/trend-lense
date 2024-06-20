use crate::exchange::Candle;
use serde::{self, Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[serde_as]
#[derive(Debug, Deserialize, Serialize)]
pub struct ApiResponse<T> {
    #[serde_as(as = "DisplayFromStr")]
    pub code: u32,
    pub msg: String,
    pub data: T,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CandleStick {
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "ts")]
    pub timestamp: u64,
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "o")]
    pub open_price: f64,
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "h")]
    pub highest_price: f64,
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "l")]
    pub lowest_price: f64,
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "c")]
    pub close_price: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub confirm: u8,
}

impl Into<Candle> for CandleStick {
    fn into(self) -> Candle {
        Candle {
            close_price: self.close_price,
            highest_price: self.highest_price,
            lowest_price: self.lowest_price,
            open_price: self.open_price,
            timestamp: self.timestamp,
        }
    }
}
