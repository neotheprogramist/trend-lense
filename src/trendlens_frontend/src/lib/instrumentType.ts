import { InstrumentType } from "./request";
import type { InstrumentType as BackendInstrumentType } from "../../../declarations/trendlens_backend/trendlens_backend.did";

export const handleInstrumentType = (
  instrument: InstrumentType,
): BackendInstrumentType => {
  switch (instrument) {
    case InstrumentType.Features:
      return {
        Futures: null,
      };
    case InstrumentType.Margin:
      return {
        Margin: null,
      };
    case InstrumentType.Spot:
      return {
        Spot: null,
      };
    case InstrumentType.Swap:
      return {
        Swap: null,
      };
  }
};
