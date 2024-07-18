import type { Pair } from "../../../declarations/trendlens_backend/trendlens_backend.did";
import { anonymousBackend } from "./canisters";
import { Exchanges, handleExchange } from "./exchange";
import { handleInstrumentType } from "./instrumentType";
import { keyStore } from "./keystore.svelte";
import type { InstrumentType } from "./request";
import { isInstrumentsResponse } from "./response";
import { extractOkValue } from "./result";
import { finishSignature } from "./signature";
import { wallet } from "./wallet.svelte";

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

  // public userInstruments = $state<Map<[Exchanges, InstrumentType], Pair[]>>(
  //   new Map(),
  // );
  // public filteredUserInstruments = $state<PairWithExchanges[]>([]);

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

  // private async loadAllUserInstruments(instrumentType: InstrumentType) {
  //   const userExchanges = keyStore.exchanges();
  //   const promises = userExchanges.map((e) =>
  //     this.loadUserInstruments(e, instrumentType),
  //   );
  //   return await Promise.all(promises);
  // }

  // private async loadUserInstruments(
  //   exchange: Exchanges,
  //   instrumentType: InstrumentType,
  // ) {
  //   if (!wallet.actor) {
  //     throw new Error("wallet actor not defined");
  //   }

  //   const key = keyStore.getByExchange(exchange);

  //   if (!key) {
  //     throw new Error("api key not exist in local storage");
  //   }

  //   const requestNumber = await wallet.actor.add_instruction({
  //     api_key: key.apiKey,
  //     exchange: handleExchange(exchange),
  //     request: {
  //       Instruments: {
  //         instrument_id: [], // we want all instruments,
  //         instrument_type: handleInstrumentType(instrumentType),
  //       },
  //     },
  //   });

  //   const requestSignatureData =
  //     await wallet.actor.get_signature_string(requestNumber);

  //   const isoTimestamp = new Date().toISOString();

  //   const signature = await finishSignature(
  //     requestSignatureData,
  //     key.secretKey,
  //     isoTimestamp,
  //   );

  //   const result = await wallet.actor.run_request(
  //     requestNumber,
  //     signature,
  //     isoTimestamp,
  //   );

  //   try {
  //     const response = extractOkValue(result);

  //     if (isInstrumentsResponse(response)) {
  //       const instruments = response.Instruments.map((i) => i.instrument_id);
  //       const index: [Exchanges, InstrumentType] = [exchange, instrumentType];
  //       this.userInstruments.set(index, instruments);
  //     } else {
  //       throw new Error("Response returned not type of instruments");
  //     }
  //   } catch (err) {
  //     console.error(err);
  //   }
  // }

  private extractUniqueInstruments(type: InstrumentType): PairWithCount[] {
    const flatPairs: Pair[] = [];

    Object.keys(Exchanges).forEach((e) => {
      const exchange = e as Exchanges;
      const instruments = this.globalInstruments.get([exchange, type].join());

      if (instruments) {
        instruments.forEach((i) => flatPairs.push(i));
      }
    });

    return this.countExchanges(flatPairs).sort((a, b) => a.count < b.count ? 1: 0);
  }

  // public async filterByUser(type: InstrumentType, loaded?: boolean) {
  //   const instruments = await this.getAllUserInstruments(type, loaded);
  //   const intersect = instruments.filter((el) =>
  //     this.filteredInstruments.some(
  //       (e) => e.base == el.base && e.quote == el.quote,
  //     ),
  //   );

  //   this.filteredInstruments = intersect;
  // }

  // public async getAllUserInstruments(
  //   type: InstrumentType,
  //   loaded?: boolean,
  // ): Promise<Pair[]> {
  //   if (!this.loaded) {
  //     await this.loadAllUserInstruments(type);
  //   }

  //   return this.flatInstruments(this.userInstruments, type);
  // }

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
    // if (!this.loaded.values.((v) => v)) {
    await this.loadAllInstruments(type);
    // }

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
    console.log(
      exchange,
      instrumentType,
      instrument,
      this.globalInstruments.get([exchange, instrumentType].join()),
    );

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
