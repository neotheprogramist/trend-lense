use super::{
    api::{GetBalanceRequest, GetInstrumentsRequest},
    Okx,
};
use crate::{
    remote_exchanges::{
        request::{GeneralBalanceRequest, GeneralInstrumentsRequest},
        ExchangeErrors, UserData,
    },
    request_store::request::Response,
};

#[async_trait::async_trait]
impl UserData for Okx {
    async fn get_instruments(
        &self,
        req: GeneralInstrumentsRequest,
    ) -> Result<Response, ExchangeErrors> {
        let exchange_request = GetInstrumentsRequest {
            instrument_type: req.instrument_type,
            instrument_id: req.instrument_id,
        };

        let instrument_response = self
            .api_client
            .call(exchange_request, self.auth.as_ref())
            .await?;

        Ok(Response::Instruments(instrument_response))
    }

    async fn get_balance(
        &self,
        request: GeneralBalanceRequest,
    ) -> Result<Response, ExchangeErrors> {
        let currencies = match request.currency.len() {
            0 => None,
            _ => Some(request.currency.join(",")),
        };

        let exchange_request = GetBalanceRequest { currencies };

        let balances_response = self
            .api_client
            .call(exchange_request, self.auth.as_ref())
            .await?;

        Ok(Response::Balances(balances_response))
    }
}
