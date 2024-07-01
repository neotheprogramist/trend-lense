import { type ApiData } from "../../../declarations/trendlens_backend/trendlens_backend.did";

export type ApiWithSecret = ApiData & {
  secret_key: string;
};

class KeyStore {
  public m_keys = $state<ApiWithSecret[]>([]);
  public m_focussed = $state<ApiWithSecret | null>(null);

  constructor() {
    // Initialization is already handled in property declaration
  }

  public focus(key: string): void {
    const found = this.m_keys.find((el) => el.api_key === key);

    if (found !== undefined) {
      this.m_focussed = found;
    }
  }

  public load(): void {
    this.m_keys = []; // Reset the keys array before loading

    for (let i = 0; i < window.localStorage.length; i++) {
      const key = window.localStorage.key(i);

      if (key === null) continue;

      const item = window.localStorage.getItem(key);

      if (item === null) continue;

      try {
        const decodedItem: ApiWithSecret = JSON.parse(item);
        this.m_keys.push(decodedItem);
      } catch (e) {
        console.error(`Error parsing localStorage item with key ${key}:`, e);
      }
    }
  }
}

export const keyStore = new KeyStore();
