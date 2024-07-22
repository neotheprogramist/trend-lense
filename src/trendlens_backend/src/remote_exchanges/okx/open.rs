use super::api::{GetOrderBookRequest, InstrumentType, TakerVolumeRequest};
use super::auth::OkxAuth;
use super::response::{ApiResponse, CandleStick, ConcreteInstrument, OrderBook, TakerVolume};
use super::{api::IndexCandleStickRequest, Okx};
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
        let index_name = Okx::instrument_id(pair).ok_or_else(|| ExchangeErrors::MissingIndex)?;

        ic_cdk::println!("index_name: {:?}", index_name);
        let candle_request = IndexCandleStickRequest {
            after_timestamp: None,
            before_timestamp: Some(range.start * 1000),
            bar_size: Some(Okx::interval_string(interval)),
            index_name,
            results_limit: None,
        };

        let candle_response = self
            .api_client
            .call::<ApiResponse<Vec<CandleStick>>, IndexCandleStickRequest, OkxAuth>(
                candle_request,
                self.auth.as_ref(),
            )
            .await?;

        Ok(candle_response
            .into_iter()
            .map(|concrete_candle| concrete_candle.into())
            .collect())
    }

    async fn get_taker_volume(
        &self,
        pair: &Pair,
        range: std::ops::Range<u64>,
    ) -> Result<Vec<TimeVolume>, ExchangeErrors> {
        let request = TakerVolumeRequest {
            begin: Some(range.start),
            end: Some(range.end),
            period: None,
            currency: pair.base.clone(),
            instrument_type: Some(InstrumentType::Spot),
        };

        let response = self
            .api_client
            .call::<ApiResponse<Vec<TakerVolume>>, TakerVolumeRequest, OkxAuth>(
                request,
                self.auth.as_ref(),
            )
            .await?;

        Ok(response
            .into_iter()
            .map(|taker_volume| TimeVolume {
                timestamp: taker_volume.timestamp,
                volume: taker_volume.sell_volume + taker_volume.buy_volume,
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
