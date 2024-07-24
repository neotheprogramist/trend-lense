import type {
  PositionSide as BackendPositionSide,
  OrderSide,
  OrderType,
  Result,
  Result_1,
  Result_2,
  Result_3,
  TradeMode,
} from "../../../declarations/trendlens_backend/trendlens_backend.did";
import { Exchanges, handleExchange } from "./exchange";
import { keyStore } from "./keystore.svelte";
import { pairFromString } from "./pair";
import {
  InstrumentType,
  OrderSideType,
  OrderTypeType,
  PositionSideType,
  TradeModeType,
} from "./request";
import { isPostOrderResponse } from "./response";
import {
  extractOkValue,
  isApiClientError,
  isExchangeErr,
  isHttpApiClientError,
} from "./result";
import { finishSignature } from "./signature";
import { wallet } from "./wallet.svelte";

export class PostOrderRequest {
  instrumentId: string;
  tradeModes: TradeModeType[];

  // // skip for now
  // marginCurrency: string;
  tradeMode = $state<TradeModeType | null>(null);
  positionSide = $state<PositionSideType | null>(null);
  size = $state<string | undefined>(undefined);
  orderSide = $state<OrderSideType>(OrderSideType.Buy);
  orderType = $state<OrderTypeType>(OrderTypeType.Market);

  orderPrice = $state<string | undefined>(undefined);
  orderPriceRequired = $derived.by(() => {
    return this.orderType != OrderTypeType.Market;
  });

  constructor(tradeModes: TradeModeType[], instrumentId: string) {
    this.instrumentId = instrumentId;
    this.tradeModes = tradeModes;

    if (tradeModes.length > 0) {
      this.tradeMode = tradeModes[0];
    }
  }

  changeInstrumentId(instrumentId: string) {
    this.instrumentId = instrumentId;
  }
}

export const handleOrderSide = (orderSide: OrderSideType): OrderSide => {
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

export const handleOrderType = (orderType: OrderTypeType): OrderType => {
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

export const handleTradeMode = (tradeMode: TradeModeType): TradeMode => {
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

  return await wallet.actor.add_transaction([
    {
      api_key: key.apiKey,
      exchange: handleExchange(exchange),
      request: {
        PostOrder: {
          instrument_id: pairFromString(request.instrumentId),
          trade_mode: handleTradeMode(request.tradeMode!),
          margin_currency: [],
          order_price:
            request.orderPrice != undefined ? [Number(request.orderPrice)] : [],
          order_type: handleOrderType(request.orderType),
          side: handleOrderSide(request.orderSide),
          size: request.size ? Number(request.size) : 0,
          position_side: request.positionSide
            ? [handlePositionSide(request.positionSide)]
            : [],
        },
      },
    },
  ]);
};

export const extractApiHttpError = (result: Result_2) => {
  if (isExchangeErr(result)) {
    if (isApiClientError(result.Err)) {
      if (isHttpApiClientError(result.Err.ApiClientError)) {
        return `HTTP error: ${result.Err.ApiClientError.Http.status} ${result.Err.ApiClientError.Http.body}`;
      }
    }
  }
  console.log(result)

  throw new Error("Unknown API client error");
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

  const [requestNumber, instructions] = await postRequest(exchange, request);

  const timestamp = Math.round(Date.now() / 1000) - 1;
  const isoTimestamp = new Date().toISOString();

  const signature = await finishSignature(
    exchange,
    instructions[0].signature,
    key.secretKey,
    exchange == Exchanges.Coinbase ? timestamp.toString() : isoTimestamp,
  );

  const result = await wallet.actor.run_transaction(
    requestNumber,
    [signature],
    isoTimestamp,
    BigInt(timestamp),
  );

  console.log(result)

  try {
    const response = extractOkValue(result)[0];

    if (isPostOrderResponse(response)) {
      return response.Order.message;
    } else {
      throw new Error("Unexpected response");
    }
  } catch (err) {
    return extractApiHttpError(result);
  }
};
