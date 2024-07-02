import type { Exchanges } from "./exchange";

export type ApiData = {
  apiKey: string;
  passphrase: string;
  exchange: Exchanges;
};

export type ApiWithSecret = {
  secretKey: string;
} & ApiData;

class KeyStore {
  keys = $state<ApiWithSecret[]>([]);
  static LOCAL_STORAGE_KEY = "api_keys";

  constructor() {}

  private ensureLoaded(): void {
    if (this.keys.length == 0) {
      this.load();
    }
  }

  public getByExchange(exchange: Exchanges): ApiWithSecret | null {
    this.ensureLoaded();

    const found = this.keys.find((el) => el.exchange == exchange);

    if (found === undefined) {
      return null;
    } else {
      return found;
    }
  }

  // TODO: add bound denying adding another key for exchange until previous
  // was removed
  public add(keyData: ApiWithSecret): void {
    this.ensureLoaded();
    this.keys = [...this.keys, keyData];
    this.save();
  }

  public remove(key: string): void {
    this.ensureLoaded();
    this.keys = this.keys.filter((k) => k.apiKey != key);
    this.save();
  }

  public save(): void {
    window.localStorage.setItem(
      KeyStore.LOCAL_STORAGE_KEY,
      JSON.stringify(this.keys),
    );
  }

  public load(): void {
    this.keys = [];

    const encodedKeys = window.localStorage.getItem(KeyStore.LOCAL_STORAGE_KEY);

    if (!encodedKeys) {
      return;
    }

    try {
      const decodedKeys: ApiWithSecret[] = JSON.parse(encodedKeys);
      this.keys = decodedKeys;
    } catch (e) {
      console.error(`Error decoding keys`);
    }
  }
}

export const keyStore = new KeyStore();
