import type {
  ExchangeErrors,
  Response,
  Result,
} from "../../../declarations/trendlens_backend/trendlens_backend.did";

export function isOk(result: Result): result is { Ok: Response } {
  return (result as { Ok: Response }).Ok !== undefined;
}

export function isExchangeErr(
  result: Result,
): result is { Err: ExchangeErrors } {
  return (result as { Err: ExchangeErrors }).Err !== undefined;
}
