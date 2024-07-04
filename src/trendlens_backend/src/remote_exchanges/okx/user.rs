use super::{api::GetInstrumentsRequest, Okx};
use crate::{
    remote_exchanges::{request::GeneralInstrumentsRequest, ExchangeErrors, UserData},
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
        
        let instrument_response = self.api_client.call(exchange_request, self.auth.as_ref()).await?;

        Ok(Response::Instruments(instrument_response))
    }
}
