import type { Exchange } from "../../../declarations/trendlens_backend/trendlens_backend.did";

export enum Exchanges {
  Okx = "Okx",
  Coinbase = "Coinbase",
}

export type ExchangeKey = keyof typeof Exchanges;

export type CandleStickData = {
  timestamp: number;
  open_price: number;
  close_price: number;
  lowest_price: number;
  highest_price: number;
};

export const handleExchange = (exchange: Exchanges): Exchange => {
  switch (exchange) {
    case Exchanges.Okx:
      return { Okx: null };
    case Exchanges.Coinbase:
      return { Coinbase: null };
  }
};

export const toExchanges = (exchange: Exchange): Exchanges => {
  if (Object.is(exchange, { Okx: null })) return Exchanges.Okx;
  if (Object.is(exchange, { Coinbase: null })) return Exchanges.Coinbase;
  throw new Error("Invalid exchange");
};
