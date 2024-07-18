import type {
  Candle,
  ExchangeErrors,
  Response,
  Result,
  Result_1,
  Result_2,
  Result_3,
} from "../../../declarations/trendlens_backend/trendlens_backend.did";

export function extractOkValue(result: Result): Array<Candle>;
export function extractOkValue(result: Result_1): boolean;
export function extractOkValue(result: Result_2): Array<Response>;
export function extractOkValue(result: Result_3): number;
export function extractOkValue(
  result: Result | Result_1 | Result_2 | Result_3,
): Array<Candle> | Array<Response> | boolean | number {
  if ("Ok" in result) {
    return result.Ok;
  } else if ("Err" in result) {
    console.log(result.Err);
    // throw new Error(`Err: ${JSON.stringify(result.Err)}`);
  }

  throw new Error("Unexpected result format");
}

export function isExchangeErr(
  result: Result,
): result is { Err: ExchangeErrors } {
  return (result as { Err: ExchangeErrors }).Err !== undefined;
}
