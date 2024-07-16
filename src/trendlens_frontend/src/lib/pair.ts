import type { Pair } from "../../../declarations/trendlens_backend/trendlens_backend.did";

export const pairFromString = (pair: string): Pair => {
  const [base, quote] = pair.split("-");
  return { base, quote };
};

export const pairToString = (pair: Pair): string => {
  return `${pair.base}-${pair.quote}`;
};
