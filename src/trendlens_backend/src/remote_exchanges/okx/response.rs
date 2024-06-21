use crate::exchange::Candle;
use serde::{self, Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[serde_as]
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ApiResponse<T> {
    #[serde_as(as = "DisplayFromStr")]
    pub code: u32,
    pub msg: String,
    pub data: T,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize() {
        let response = r#"{"code":"0","msg":"success","data":[{"ts":"1620000000000","o":"1.0","h":"1.0","l":"1.0","c":"1.0","confirm":"1"}]}"#;
        let response: ApiResponse<Vec<CandleStick>> = serde_json::from_str(response).unwrap();

        assert_eq!(response.code, 0);
        assert_eq!(response.msg, "success");
        assert_eq!(response.data.len(), 1);
        assert_eq!(response.data[0].timestamp, 1620000000000);
        assert_eq!(response.data[0].open_price, 1.0);
        assert_eq!(response.data[0].highest_price, 1.0);
        assert_eq!(response.data[0].lowest_price, 1.0);
        assert_eq!(response.data[0].close_price, 1.0);
        assert_eq!(response.data[0].confirm, 1);
    }
}