use crate::{remote_exchanges::{ExchangeErrors, UserData}, request_store::request::Response};
use super::{api::GetInstrumentsRequest, Okx};

#[async_trait::async_trait]
impl UserData for Okx {
    async fn get_instruments(
        &self,
        req: GetInstrumentsRequest,
    ) -> Result<Response, ExchangeErrors> {
        let instrument_response = self.api_client.call(req, self.auth.as_ref()).await?;

        Ok(Response::Instruments(instrument_response))
    }
}
