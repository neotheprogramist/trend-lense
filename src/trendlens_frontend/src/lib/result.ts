import type {
  Candle,
  ExchangeErrors,
  Response,
  Result,
  Result_1,
} from "../../../declarations/trendlens_backend/trendlens_backend.did";

export function extractOkValue(result: Result): Array<Candle>;
export function extractOkValue(result: Result_1): Response;
export function extractOkValue(result: Result | Result_1): Array<Candle> | Response {
  if ("Ok" in result) {
    return result.Ok;
  } else if ("Err" in result) {
    throw new Error(`Err: ${JSON.stringify(result.Err)}`);
  }

  throw new Error("Unexpected result format");
}

export function isExchangeErr(
  result: Result,
): result is { Err: ExchangeErrors } {
  return (result as { Err: ExchangeErrors }).Err !== undefined;
}