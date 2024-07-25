import type { Pair } from "../../../declarations/trendlens_backend/trendlens_backend.did";
import { anonymousBackend } from "./canisters";
import { Exchanges, handleExchange } from "./exchange";
import { handleInstrumentType } from "./instrumentType";
import type { InstrumentType } from "./request";

export type ExchangeWithInstrumentType = string;

export interface PairWithCount {
  pair: Pair;
  count: number;
}

class InstrumentsStore {
  public globalInstruments = $state<Map<ExchangeWithInstrumentType, Pair[]>>(
    new Map(),
  );
  public loaded = $state<Map<ExchangeWithInstrumentType, boolean>>(new Map());
  public uniqueInstruments = $state<Map<InstrumentType, PairWithCount[]>>(
    new Map(),
  );

  constructor() {}

  private async loadExchangeInstruments(
    exchange: Exchanges,
    instrumentType: InstrumentType,
  ) {
    const fetchedPairs = await anonymousBackend.get_instruments(
      handleExchange(exchange),
      handleInstrumentType(instrumentType),
    );

    this.globalInstruments.set([exchange, instrumentType].join(), fetchedPairs);
  }

  public async loadAllInstruments(instrumentType: InstrumentType) {
    await Promise.all(
      Object.keys(Exchanges).map(async (e) => {
        const key = e as Exchanges;
        await this.loadExchangeInstruments(key, instrumentType);
        this.loaded.set([key, instrumentType].join(), true);
      }),
    );
  }

  private extractUniqueInstruments(type: InstrumentType): PairWithCount[] {
    const flatPairs: Pair[] = [];

    Object.keys(Exchanges).forEach((e) => {
      const exchange = e as Exchanges;
      const instruments = this.globalInstruments.get([exchange, type].join());

      if (instruments) {
        instruments.forEach((i) => flatPairs.push(i));
      }
    });

    return this.countExchanges(flatPairs).sort((a, b) =>
      a.count < b.count ? 1 : 0,
    );
  }

  private countExchanges(pairs: Pair[]): PairWithCount[] {
    const exchangeCount: Map<string, number> = new Map();

    pairs.forEach((pair) => {
      const pairString = pair.base + "-" + pair.quote;
      exchangeCount.set(pairString, (exchangeCount.get(pairString) || 0) + 1);
    });

    const exchangeCounts: PairWithCount[] = [];

    exchangeCount.forEach((count, key) => {
      const [base, quote] = key.split("-");
      exchangeCounts.push({ pair: { quote, base }, count });
    });

    return exchangeCounts;
  }

  public async getUniqueInstruments(
    type: InstrumentType,
  ): Promise<PairWithCount[]> {
    await this.loadAllInstruments(type);

    if (!this.uniqueInstruments.has(type)) {
      const instruments = this.extractUniqueInstruments(type);
      this.uniqueInstruments.set(type, instruments);
    }

    return this.uniqueInstruments.get(type) || [];
  }

  public hasExchangeInstrument(
    exchange: Exchanges,
    instrumentType: InstrumentType,
    instrument: Pair,
  ): boolean {
    return (
      this.globalInstruments
        .get([exchange, instrumentType].join())
        ?.some(
          (i) => i.base == instrument.base && i.quote == instrument.quote,
        ) || false
    );
  }
}

export const instrumentsStore = new InstrumentsStore();
