import type {
  ApiClientErrors,
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
export function extractOkValue(
  result: Result_4,
): [number, Array<SignableInstruction>];
export function extractOkValue(
  result: Result | Result_1 | Result_2 | Result_3 | Result_4,
):
  | Array<Candle>
  | Array<Response>
  | Array<TimeVolume>
  | boolean
  | [number, Array<SignableInstruction>] {
  if ("Ok" in result) {
    return result.Ok;
  } else if ("Err" in result) {
    throw new Error(`Contract error`);
  }

  throw new Error("Unexpected error");
}

export function isExchangeErr(
  result: Result | Result_1 | Result_2 | Result_3 | Result_4,
): result is { Err: ExchangeErrors } {
  return (result as { Err: ExchangeErrors }).Err !== undefined;
}

export function isApiClientError(
  result: ExchangeErrors,
): result is { ApiClientError: ApiClientErrors } {
  return (
    (result as { ApiClientError: ApiClientErrors }).ApiClientError !== undefined
  );
}

export function isHttpApiClientError(
  result: ApiClientErrors,
): result is { Http: { status: bigint; body: string } } {
  return (
    (result as { Http: { status: bigint; body: string } }).Http !== undefined
  );
}
