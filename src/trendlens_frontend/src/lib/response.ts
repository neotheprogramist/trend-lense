import type {
  Instrument,
  OrderData,
  Response,
  Balance,
  GlobalPendingOrder,
} from "../../../declarations/trendlens_backend/trendlens_backend.did";

export function isInstrumentsResponse(
  response: Response,
): response is { Instruments: Array<Instrument> } {
  return (
    (response as { Instruments: Array<Instrument> }).Instruments !== undefined
  );
}

export function isPostOrderResponse(
  response: Response,
): response is { Order: OrderData } {
  return (response as { Order: OrderData }).Order !== undefined;
}

export function isBalanceResponse(
  response: Response,
): response is { Balances: Array<Balance> } {
  return (response as { Balances: Array<Balance> }).Balances !== undefined;
}

export function isPendingOrdersResponse(
  response: Response,
): response is { PendingOrders: Array<GlobalPendingOrder> } {
  return (
    (response as { PendingOrders: Array<GlobalPendingOrder> }).PendingOrders !==
    undefined
  );
}
