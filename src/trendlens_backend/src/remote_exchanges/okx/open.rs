use super::api::{GetOrderBookRequest, SpotCandleStickRequest};
use super::auth::OkxAuth;
use super::response::{ApiResponse, CandleStick, ConcreteInstrument, OrderBook};
use super::Okx;
use crate::exchange::TimeVolume;
use crate::remote_exchanges::request::GeneralInstrumentsRequest;
use crate::remote_exchanges::response::{Instrument, OrderBook as GlobalOrderBook};
use crate::{
    exchange::Candle,
    pair::Pair,
    remote_exchanges::{okx::api::GetInstrumentsRequestPublic, ExchangeErrors, OpenData},
};

#[async_trait::async_trait]
impl OpenData for Okx {
    async fn fetch_candles(
        &self,
        pair: &Pair,
        range: std::ops::Range<u64>,
        interval: u32,
    ) -> Result<Vec<Candle>, ExchangeErrors> {
        let request = SpotCandleStickRequest {
            end: Some(range.end * 1000),
            begin: Some(range.start * 1000),
            bar_size: Some(Okx::interval_string(interval)),
            index_name: Okx::instrument_id(pair).ok_or_else(|| ExchangeErrors::MissingIndex)?,
            results_limit: None,
        };

        let response = self
            .api_client
            .call::<ApiResponse<Vec<CandleStick>>, SpotCandleStickRequest, OkxAuth>(
                request,
                self.auth.as_ref(),
            )
            .await?;

        Ok(response
            .into_iter()
            .map(|concrete_candle| concrete_candle.into())
            .collect())
    }

    async fn get_taker_volume(
        &self,
        pair: &Pair,
        range: std::ops::Range<u64>,
    ) -> Result<Vec<TimeVolume>, ExchangeErrors> {
        let candles = self.fetch_candles(pair, range, 5).await.unwrap();

        Ok(candles
            .into_iter()
            .map(|candle| TimeVolume {
                timestamp: candle.timestamp,
                volume: candle.volume,
            })
            .collect())
    }

    async fn get_public_instruments(
        &self,
        request: GeneralInstrumentsRequest,
    ) -> Result<Vec<Instrument>, ExchangeErrors> {
        let okx_request = GetInstrumentsRequestPublic {
            instrument_id: request.instrument_id.and_then(|p| Okx::instrument_id(&p)),
            instrument_type: request.instrument_type,
        };

        let instrument_response = self
            .api_client
            .call::<ApiResponse<Vec<ConcreteInstrument>>, GetInstrumentsRequestPublic, OkxAuth>(
                okx_request,
                self.auth.as_ref(),
            )
            .await?;

        Ok(instrument_response
            .into_iter()
            .map(|concrete_instrument| concrete_instrument.into())
            .collect())
    }

    async fn get_orderbook(
        &self,
        pair: &Pair,
        size: u32,
    ) -> Result<GlobalOrderBook, ExchangeErrors> {
        let instrument_id = Okx::instrument_id(pair).ok_or_else(|| ExchangeErrors::MissingIndex)?;

        let orderbook_request = GetOrderBookRequest {
            instrument_id,
            depth: Some(size),
        };

        let response = self
            .api_client
            .call::<ApiResponse<Vec<OrderBook>>, GetOrderBookRequest, OkxAuth>(
                orderbook_request,
                self.auth.as_ref(),
            )
            .await?;

        Ok(response
            .into_iter()
            .next()
            .ok_or(ExchangeErrors::MissingPair)?
            .into())
    }
}
