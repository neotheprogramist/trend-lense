use crate::{
    remote_exchanges::{request::GeneralInstrumentsRequest, ExchangeErrors, UserData},
    request_store::request::Response,
};

use super::{
    auth::CoinbaseAuth,
    request::GetProfileAccountsRequest,
    response::{self, CoinbaseResponse},
    Coinbase,
};


#[async_trait::async_trait]
impl UserData for Coinbase {
    async fn get_instruments(
        &self,
        _req: GeneralInstrumentsRequest,
    ) -> Result<Response, ExchangeErrors> {
        Ok(Response::Instruments(vec![]))
    }

    async fn get_balance(
        &self,
        _request: crate::remote_exchanges::request::GeneralBalanceRequest,
    ) -> Result<Response, ExchangeErrors> {
        // let profiles = GetProfilesRequest { active: true };

        // let profiles_response = self
        //     .api_client
        //     .call::<CoinbaseResponse<Vec<response::Profile>>, GetProfilesRequest, CoinbaseAuth>(
        //         profiles,
        //         self.auth.as_ref(),
        //     )
        //     .await?;

        // let accounts = profiles_response
        //     .iter()
        //     .map(|profile| async {
        //         let accounts = GetProfileAccountsRequest {

        //         };

        //     })
        //     .collect::<Vec<_>>();

        let accounts = GetProfileAccountsRequest {};

        let accounts =  self.api_client
          .call::<CoinbaseResponse<Vec<response::Account>>, GetProfileAccountsRequest, CoinbaseAuth>(
              accounts,
              self.auth.as_ref(),
          ).await?;

        Ok(Response::Balances(
            accounts.into_iter().map(Into::into).collect(),
        ))
    }

    async fn post_order(
        &self,
        _request: crate::remote_exchanges::request::GeneralPostOrderRequest,
    ) -> Result<Response, ExchangeErrors> {
        todo!()
    }
}
