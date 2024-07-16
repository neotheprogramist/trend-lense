import type { Exchange } from "../../../declarations/trendlens_backend/trendlens_backend.did";

export enum Exchanges {
  Okx = "Okx",
  Coinbase = "Coinbase",
  Kraken = "Kraken",
  Binance = "Binance",
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
    case Exchanges.Binance:
    case Exchanges.Kraken:
      return { Coinbase: null };
  }
};
