import type {
  OrderSide,
  OrderType,
  TradeMode,
  PositionSide as BackendPositionSide,
} from "../../../declarations/trendlens_backend/trendlens_backend.did";
import { handleExchange, type Exchanges } from "./exchange";
import { keyStore } from "./keystore.svelte";
import {
  OrderSideType,
  InstrumentType,
  OrderTypeType,
  TradeModeType,
  PositionSideType,
} from "./request";
import { isPostOrderResponse } from "./response";
import { extractOkValue } from "./result";
import { finishSignature } from "./signature";
import { wallet } from "./wallet.svelte";

export class PostOrderRequest {
  instrumentId: string;
  tradeModes: TradeModeType[];

  // // skip for now
  // marginCurrency: string;
  tradeMode = $state<TradeModeType | null>(null);
  positionSide = $state<PositionSideType | null>(null);
  size = $state<number | undefined>(undefined);
  orderSide = $state<OrderSideType>(OrderSideType.Buy);
  orderType = $state<OrderTypeType>(OrderTypeType.Market);

  orderPrice: { value?: number; required: boolean } = $derived.by(() => {
    const required = this.orderType != OrderTypeType.Market;
    return { value: undefined, required };
  });

  constructor(tradeModes: TradeModeType[], instrumentId: string) {
    this.instrumentId = instrumentId;
    this.tradeModes = tradeModes;

    if (tradeModes.length > 0) {
      this.tradeMode = tradeModes[0];
    }
  }
}

const handleOrderSide = (orderSide: OrderSideType): OrderSide => {
  switch (orderSide) {
    case OrderSideType.Buy:
      return { Buy: null };
    case OrderSideType.Sell:
      return { Sell: null };
  }
};

const handlePositionSide = (
  positionSide: PositionSideType,
): BackendPositionSide => {
  switch (positionSide) {
    case PositionSideType.Long:
      return { Long: null };
    case PositionSideType.Short:
      return { Short: null };
  }
};

const handleOrderType = (orderType: OrderTypeType): OrderType => {
  switch (orderType) {
    case OrderTypeType.Limit:
      return { Limit: null };
    case OrderTypeType.Market:
      return { Market: null };
    case OrderTypeType.Fok:
      return { Fok: null };
    case OrderTypeType.Ioc:
      return { Ioc: null };
    case OrderTypeType.PostOnly:
      return { PostOnly: null };
  }
};

const handleTradeMode = (tradeMode: TradeModeType): TradeMode => {
  switch (tradeMode) {
    case TradeModeType.Cash:
      return { Cash: null };
    case TradeModeType.Cross:
      return { Cross: null };
    case TradeModeType.Isolated:
      return { Isolated: null };
    case TradeModeType.SpotIsolated:
      return { SpotIsolated: null };
  }
};

export const inferTradeModes = (
  instrumentType: InstrumentType,
): TradeModeType[] => {
  switch (instrumentType) {
    case InstrumentType.Spot:
      return [TradeModeType.Cash];
    case InstrumentType.Margin:
      return [TradeModeType.Cross, TradeModeType.Isolated];
    case InstrumentType.Features:
    case InstrumentType.Swap:
      throw new Error("Not implemented");
  }
};

export const postRequest = async (
  exchange: Exchanges,
  request: PostOrderRequest,
) => {
  if (!wallet.actor) {
    throw new Error("wallet actor not defined");
  }

  const key = keyStore.getByExchange(exchange);

  if (!key) {
    throw new Error("api key not exist in local storage");
  }

  return await wallet.actor.add_instruction({
    api_key: key.apiKey,
    exchange: handleExchange(exchange),
    request: {
      PostOrder: {
        instrument_id: request.instrumentId,
        trade_mode: handleTradeMode(request.tradeMode!),
        margin_currency: [],
        order_price:
          request.orderPrice.value != undefined
            ? [request.orderPrice.value]
            : [],
        order_type: handleOrderType(request.orderType),
        side: handleOrderSide(request.orderSide),
        size: request.size ?? 0,
        position_side: request.positionSide
          ? [handlePositionSide(request.positionSide)]
          : [],
      },
    },
  });
};

export const executeRequest = async (
  exchange: Exchanges,
  request: PostOrderRequest,
) => {
  if (!wallet.actor) {
    throw new Error("wallet actor not defined");
  }

  const key = keyStore.getByExchange(exchange);

  if (!key) {
    throw new Error("api key not exist in local storage");
  }


  const requestNumber = await postRequest(exchange, request);

  const requestSignatureData =
    await wallet.actor.get_signature_string(requestNumber);

  const isoTimestamp = new Date().toISOString();

  const signature = await finishSignature(
    requestSignatureData,
    key.secretKey,
    isoTimestamp,
  );

  const result = await wallet.actor!.run_request(
    requestNumber,
    signature,
    isoTimestamp,
  );

  try {
    const response = extractOkValue(result);

    if (isPostOrderResponse(response)) {
      const order = response.Order;

      console.log(order.code);
    } else {
      throw new Error("Response returned not type of order");
    }
  } catch (err) {
    console.error(err);
  }
};