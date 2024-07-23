import type {
  Candle,
  ExchangeErrors,
  Response,
  Result,
  Result_1,
  Result_2,
  Result_3,
  Result_4,
  SignableInstruction,
  TimeVolume,
} from "../../../declarations/trendlens_backend/trendlens_backend.did";

export function extractOkValue(result: Result): Array<Candle>;
export function extractOkValue(result: Result_1): Array<TimeVolume>;
export function extractOkValue(result: Result_2): boolean;
export function extractOkValue(result: Result_3): Array<Response>;
export function extractOkValue(result: Result_4): [number, Array<SignableInstruction>];
export function extractOkValue(
  result: Result | Result_1 | Result_2 | Result_3 |  Result_4,
):
  | Array<Candle>
  | Array<Response>
  | Array<TimeVolume>
  | boolean
  | [number, Array<SignableInstruction>] {
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
