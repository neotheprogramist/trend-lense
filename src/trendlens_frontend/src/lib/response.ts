import type {
  Balance,
  Instrument,
  Order,
  OrderData,
  Response,
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

export function isOrdersResponse(
  response: Response,
): response is { OrdersInfo: Array<Order> } {
  return (response as { OrdersInfo: Array<Order> }).OrdersInfo !== undefined;
}
