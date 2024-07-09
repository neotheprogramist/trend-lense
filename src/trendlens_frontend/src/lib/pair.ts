import type { Pair } from "../../../declarations/trendlens_backend/trendlens_backend.did";

export const handlePair = (pair: string): Pair => {
  const [base, quote] = pair.split("-");
  return { base, quote };
};
