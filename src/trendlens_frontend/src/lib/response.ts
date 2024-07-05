import type {
  Instrument,
  Response,
} from "../../../declarations/trendlens_backend/trendlens_backend.did";

export function isInstrumentsResponse(
  response: Response,
): response is { Instruments: Array<Instrument> } {
  return (
    (response as { Instruments: Array<Instrument> }).Instruments !== undefined
  );
}
