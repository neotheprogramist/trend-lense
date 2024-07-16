use crate::{
    remote_exchanges::{request::GeneralInstrumentsRequest, ExchangeErrors, UserData},
    request_store::request::Response,
};

use super::{
    auth::CoinbaseAuth,
    request::{GetProfileAccountsRequest, OrderType, PostOrderBody},
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
        request: crate::remote_exchanges::request::GeneralPostOrderRequest,
    ) -> Result<Response, ExchangeErrors> {
        let request_coinbase = PostOrderBody {
            order_type: OrderType::Market,
            size: Some(request.size.to_string()),
            side: request.side.into(),
            funds: None,
            price: None,
            product_id: request.instrument_id,
        };

        let order = self
            .api_client
            .call::<CoinbaseResponse<response::OrderResponse>, PostOrderBody, CoinbaseAuth>(
                request_coinbase,
                self.auth.as_ref(),
            )
            .await?;

        Ok(Response::Order(order.into()))
    }
}
