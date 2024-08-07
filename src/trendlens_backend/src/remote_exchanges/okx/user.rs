use super::{
    api::{
        GetBalanceRequest, GetInstrumentsRequest, OrdersHistoryRequest, PendingOrdersRequest,
        PlaceOrderBody,
    },
    auth::OkxAuth,
    response::{
        AccountInfo, ApiResponse, ConcreteInstrument, Order as OkxOrder, PlaceOrderDetails,
    },
    Okx,
};
use crate::{
    remote_exchanges::{
        request::{
            GeneralBalanceRequest, GeneralInstrumentsRequest, GeneralOrdersListRequest,
            GeneralPostOrderRequest,
        },
        response::{Balance, Instrument, Order, OrderData},
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
            instrument_id: req.instrument_id.and_then(|p| Self::instrument_id(&p)),
        };

        let instrument_response = self
            .api_client
            .call::<ApiResponse<Vec<ConcreteInstrument>>, GetInstrumentsRequest, OkxAuth>(
                exchange_request,
                self.auth.as_ref(),
            )
            .await?;

        let parsed_instruments = instrument_response
            .into_iter()
            .map(|e| Instrument {
                instrument_id: e.instrument_id,
                instrument_type: e.instrument_type,
            })
            .collect();

        Ok(Response::Instruments(parsed_instruments))
    }

    async fn get_balance(
        &self,
        request: GeneralBalanceRequest,
    ) -> Result<Response, ExchangeErrors> {
        let currencies = request.currency.and_then(|c| Some(c.join(",")));
        let exchange_request = GetBalanceRequest { currencies };

        let balances_response = self
            .api_client
            .call::<ApiResponse<Vec<AccountInfo>>, GetBalanceRequest, OkxAuth>(
                exchange_request,
                self.auth.as_ref(),
            )
            .await?;

        Ok(Response::Balances(
            balances_response
                .into_iter()
                .flat_map(|r| Into::<Vec<Balance>>::into(r))
                .collect::<Vec<_>>(),
        ))
    }

    async fn post_order(
        &self,
        request: GeneralPostOrderRequest,
    ) -> Result<Response, ExchangeErrors> {
        let exchange_request = PlaceOrderBody {
            side: Self::side_string(request.side),
            instrument_id: request.instrument_id.to_string(),
            order_type: Self::order_type_string(request.order_type),
            size: request.size.to_string(),
            order_price: request.order_price.map(|p| p.to_string()),
            trade_mode: Self::trade_mode_string(request.trade_mode),
            ..Default::default()
        };

        let order_response = self
            .api_client
            .call::<ApiResponse<Vec<PlaceOrderDetails>>, PlaceOrderBody, OkxAuth>(
                exchange_request,
                self.auth.as_ref(),
            )
            .await?;

        Ok(Response::Order(OrderData {
            id: order_response[0].order_id.clone(),
            message: order_response[0].status_message.clone(),
        }))
    }

    async fn get_pending_orders(
        &self,
        request: GeneralOrdersListRequest,
    ) -> Result<Response, ExchangeErrors> {
        let exchange_request = PendingOrdersRequest {
            instrument_id: Some(request.instrument_id.to_string()),
            instrument_type: Some(request.instrument_type),
        };

        let order_response = self
            .api_client
            .call::<ApiResponse<Vec<OkxOrder>>, PendingOrdersRequest, OkxAuth>(
                exchange_request,
                self.auth.as_ref(),
            )
            .await?;

        Ok(Response::OrdersInfo(
            order_response
                .into_iter()
                .map(|o| Order {
                    instrument_type: o.instrument_type.to_string(),
                    instrument_id: o.instrument_id,
                    order_id: o.order_id,
                    price: o.price,
                    size: o.size,
                    side: o.side.to_string(),
                    order_type: o.order_type.to_string(),
                    trade_mode: o.trade_mode.to_string(),
                    accumulated_fill_quantity: o.accumulated_fill_quantity,
                    state: o.state.to_string(),
                })
                .collect(),
        ))
    }

    async fn get_done_orders(
        &self,
        request: GeneralOrdersListRequest,
    ) -> Result<Response, ExchangeErrors> {
        let exchange_request = OrdersHistoryRequest {
            instrument_id: Some(request.instrument_id.to_string()),
            instrument_type: request.instrument_type,
        };

        let order_response = self
            .api_client
            .call::<ApiResponse<Vec<OkxOrder>>, OrdersHistoryRequest, OkxAuth>(
                exchange_request,
                self.auth.as_ref(),
            )
            .await?;

        Ok(Response::OrdersInfo(
            order_response
                .into_iter()
                .map(|o| Order {
                    instrument_type: o.instrument_type.to_string(),
                    instrument_id: o.instrument_id,
                    order_id: o.order_id,
                    price: o.price,
                    size: o.size,
                    side: o.side.to_string(),
                    order_type: o.order_type.to_string(),
                    trade_mode: o.trade_mode.to_string(),
                    accumulated_fill_quantity: o.accumulated_fill_quantity,
                    state: o.state.to_string(),
                })
                .collect(),
        ))
    }
}
