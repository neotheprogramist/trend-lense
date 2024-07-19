use crate::{
    remote_exchanges::{
        request::{GeneralGetPendingOrdersRequest, GeneralInstrumentsRequest},
        response::GlobalPendingOrder,
        ExchangeErrors, UserData,
    },
    request_store::request::Response,
};

use super::{
    auth::CoinbaseAuth,
    request::{GetProfileAccountsRequest, OrderType, PendingOrdersRequest, PostOrderBody, },
    response::{self, CoinbaseResponse, OrderStatus},
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
            product_id: request.instrument_id.to_string(),
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

    async fn get_pending_orders(
        &self,
        request: GeneralGetPendingOrdersRequest,
    ) -> Result<Response, ExchangeErrors> {
        let exchange_request = PendingOrdersRequest {
            market_type: Some(request.instrument_type.to_string()),
            product_id: Some(request.instrument_id.to_string()),
        };

        let order_response = self
            .api_client
            .call::<CoinbaseResponse<Vec<response::Order>>, PendingOrdersRequest, CoinbaseAuth>(
                exchange_request,
                self.auth.as_ref(),
            )
            .await?;

        Ok(Response::PendingOrders(
            order_response
                .into_iter()
                .filter_map(|o| match o.status {
                    OrderStatus::Pending
                    | OrderStatus::Active
                    | OrderStatus::Received
                    | OrderStatus::Open => Some(GlobalPendingOrder {
                        instrument_type: request.instrument_type.to_string(),
                        instrument_id: o.product_id,
                        order_id: o.id,
                        price: o.price.unwrap_or(0.0),
                        size: o.size.unwrap_or(0.0),
                        side: o.side.to_string(),
                        order_type: o.order_type.unwrap_or("".to_string()),
                        trade_mode: o.time_in_force.unwrap_or("".to_string()),
                        accumulated_fill_quantity: o.filled_size,
                    }),
                    _ => None,
                })
                .collect(),
        ))
    }
}
