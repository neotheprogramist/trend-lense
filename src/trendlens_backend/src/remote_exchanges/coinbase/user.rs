use crate::{
    remote_exchanges::{
        request::{GeneralInstrumentsRequest, GeneralOrdersListRequest, OrderSide},
        response::Order,
        ExchangeErrors, UserData,
    },
    request_store::request::Response,
};

use super::{
    auth::CoinbaseAuth,
    request::{GetProfileAccountsRequest, OrdersRequest, PostOrderBody, Statuses},
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
            order_type: request.order_type.into(),
            size: match request.side {
                OrderSide::Buy => None,
                OrderSide::Sell => Some(request.size),
            },
            side: request.side.into(),
            funds: match request.side {
                OrderSide::Buy => Some(request.size),
                OrderSide::Sell => None,
            },
            price: request.order_price,
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
        request: GeneralOrdersListRequest,
    ) -> Result<Response, ExchangeErrors> {
        let exchange_request = OrdersRequest {
            market_type: Some(request.instrument_type.to_string()),
            product_id: Some(request.instrument_id.to_string()),
            limit: 100,
            status: Some(Statuses(vec![
                OrderStatus::Open.to_string(),
                OrderStatus::Pending.to_string(),
            ])),
        };

        let order_response = self
            .api_client
            .call::<CoinbaseResponse<Vec<response::Order>>, OrdersRequest, CoinbaseAuth>(
                exchange_request,
                self.auth.as_ref(),
            )
            .await?;

        Ok(Response::OrdersInfo(
            order_response
                .into_iter()
                .filter_map(|o| match o.status {
                    OrderStatus::Open | OrderStatus::Pending => Some(Order {
                        instrument_type: request.instrument_type.to_string(),
                        instrument_id: o.product_id,
                        order_id: o.id,
                        price: o.price.unwrap_or(0.0),
                        size: o.size.unwrap_or(o.funds.unwrap_or(0.0)),
                        side: o.side.to_string(),
                        order_type: o.order_type.unwrap_or("".to_string()),
                        trade_mode: o.time_in_force.unwrap_or("".to_string()),
                        accumulated_fill_quantity: o.filled_size,
                        state: o.status.to_string(),
                    }),
                    _ => None,
                })
                .collect(),
        ))
    }

    async fn get_done_orders(
        &self,
        request: GeneralOrdersListRequest,
    ) -> Result<Response, ExchangeErrors> {
        let exchange_request = OrdersRequest {
            market_type: Some(request.instrument_type.to_string()),
            product_id: Some(request.instrument_id.to_string()),
            limit: 100,
            status: Some(Statuses(vec![
                OrderStatus::Done.to_string(),
                OrderStatus::Rejected.to_string(),
            ])),
        };

        let order_response = self
            .api_client
            .call::<CoinbaseResponse<Vec<response::Order>>, OrdersRequest, CoinbaseAuth>(
                exchange_request,
                self.auth.as_ref(),
            )
            .await?;

        Ok(Response::OrdersInfo(
            order_response
                .into_iter()
                .filter_map(|o| match o.status {
                    OrderStatus::Done | OrderStatus::Rejected => Some(Order {
                        instrument_type: request.instrument_type.to_string(),
                        instrument_id: o.product_id,
                        order_id: o.id,
                        price: o.price.unwrap_or(0.0),
                        size: o.size.unwrap_or(o.funds.unwrap_or(0.0)),
                        side: o.side.to_string(),
                        order_type: o.order_type.unwrap_or("".to_string()),
                        trade_mode: o.time_in_force.unwrap_or("".to_string()),
                        accumulated_fill_quantity: o.filled_size,
                        state: o.status.to_string(),
                    }),
                    _ => None,
                })
                .collect(),
        ))
    }
}
