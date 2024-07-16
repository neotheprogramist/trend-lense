use super::{api::IndexCandleStickRequest, Okx};
use crate::remote_exchanges::request::GeneralInstrumentsRequest;
use crate::remote_exchanges::response::Instrument;
use crate::{
    exchange::Candle,
    pair::Pair,
    remote_exchanges::{okx::api::GetInstrumentsRequestPublic, ExchangeErrors, OpenData},
};

#[async_trait::async_trait]
impl OpenData for Okx {
    async fn fetch_candles(
        &self,
        pair: Pair,
        range: std::ops::Range<u64>,
        interval: u32,
    ) -> Result<Vec<Candle>, ExchangeErrors> {
        let index_name = Okx::instrument_id(pair).ok_or_else(|| ExchangeErrors::MissingIndex)?;

        let candle_request = IndexCandleStickRequest {
            after_timestamp: None,
            before_timestamp: Some(range.start * 1000),
            bar_size: Some(Okx::interval_string(interval)),
            index_name,
            results_limit: None,
        };

        let candle_response = self
            .api_client
            .call(candle_request, self.auth.as_ref())
            .await?;

        Ok(candle_response
            .into_iter()
            .map(|concrete_candle| concrete_candle.into())
            .collect())
    }

    async fn get_public_instruments(
        &self,
        request: GeneralInstrumentsRequest,
    ) -> Result<Vec<Instrument>, ExchangeErrors> {
        let okx_request = GetInstrumentsRequestPublic {
            instrument_id: request.instrument_id.and_then(|p| Okx::instrument_id(p)),
            instrument_type: request.instrument_type,
        };

        let instrument_response = self
            .api_client
            .call(okx_request, self.auth.as_ref())
            .await?;

        Ok(instrument_response
            .into_iter()
            .map(|concrete_instrument| concrete_instrument.into())
            .collect())
    }
}
