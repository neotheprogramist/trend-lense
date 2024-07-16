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

class InstrumentsStore {
  public globalInstruments = $state<Map<[Exchanges, InstrumentType], Pair[]>>(
    new Map(),
  );
  public userInstruments = $state<Map<[Exchanges, InstrumentType], Pair[]>>(
    new Map(),
  );
  public filteredUserInstruments = $state<Pair[]>([]);
  public filteredInstruments = $state<Pair[]>([]);

  constructor() {}

  private async loadExchangeGlobalInstruments(
    exchange: Exchanges,
    instrumentType: InstrumentType,
  ) {
    const fetchedPairs = await anonymousBackend.get_instruments(
      handleExchange(exchange),
      handleInstrumentType(instrumentType),
    );
    const index: [Exchanges, InstrumentType] = [exchange, instrumentType];
    this.globalInstruments.set(index, fetchedPairs);
  }

  private async loadAllGlobalInstruments(instrumentType: InstrumentType) {
    await Promise.all(
      Object.keys(Exchanges).map((e) =>
        this.loadExchangeGlobalInstruments(e as Exchanges, instrumentType),
      ),
    );
  }

  private async loadAllUserInstruments(instrumentType: InstrumentType) {
    const userExchanges = keyStore.exchanges();
    const promises = userExchanges.map((e) =>
      this.loadUserInstruments(e, instrumentType),
    );
    return await Promise.all(promises);
  }

  private async loadUserInstruments(
    exchange: Exchanges,
    instrumentType: InstrumentType,
  ) {
    if (!wallet.actor) {
      throw new Error("wallet actor not defined");
    }

    const key = keyStore.getByExchange(exchange);

    if (!key) {
      throw new Error("api key not exist in local storage");
    }

    const requestNumber = await wallet.actor.add_instruction({
      api_key: key.apiKey,
      exchange: handleExchange(exchange),
      request: {
        Instruments: {
          instrument_id: [], // we want all instruments,
          instrument_type: handleInstrumentType(instrumentType),
        },
      },
    });

    const requestSignatureData =
      await wallet.actor.get_signature_string(requestNumber);

    const isoTimestamp = new Date().toISOString();

    const signature = await finishSignature(
      requestSignatureData,
      key.secretKey,
      isoTimestamp,
    );

    const result = await wallet.actor.run_request(
      requestNumber,
      signature,
      isoTimestamp,
    );

    try {
      const response = extractOkValue(result);

      if (isInstrumentsResponse(response)) {
        const instruments = response.Instruments.map((i) => i.instrument_id);
        const index: [Exchanges, InstrumentType] = [exchange, instrumentType];
        this.userInstruments.set(index, instruments);
      } else {
        throw new Error("Response returned not type of instruments");
      }
    } catch (err) {
      console.error(err);
    }
  }

  private flatInstruments(type: InstrumentType): Pair[] {
    const flatUnique = new Set<Pair>();

    this.globalInstruments.forEach((values, key) => {
      const [_, instrumentType] = key;
      if (instrumentType === type) {
        values.forEach((value) => flatUnique.add(value));
      }
    });

    return Array.from(flatUnique);
  }

  public async filterByType(type: InstrumentType) {
    this.filteredInstruments = await this.getAllInstruments(type, true);
  }

  public async getAllUserInstruments(
    type: InstrumentType,
    loaded?: boolean,
  ): Promise<Pair[]> {
    if (loaded != undefined && loaded != false) {
      await this.loadAllUserInstruments(type);
    }

    return this.flatInstruments(type);
  }

  public async getAllInstruments(
    type: InstrumentType,
    loaded?: boolean,
  ): Promise<Pair[]> {
    if (loaded != undefined && loaded != false) {
      await this.loadAllGlobalInstruments(type);
    }

    return this.flatInstruments(type);
  }
}

export const instrumentsStore = new InstrumentsStore();
