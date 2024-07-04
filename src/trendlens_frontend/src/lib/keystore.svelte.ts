import { type ApiData } from '../../../declarations/trendlens_backend/trendlens_backend.did';

export type ApiWithSecret = ApiData & {
	secret_key: string;
};

interface IKeyStore {
	get keys(): ApiWithSecret[];
	load(): void;
}

class KeyStore implements IKeyStore {
	private m_keys = $state<ApiWithSecret[]>([]);

	constructor() {}

	get keys() {
		return this.m_keys;
	}

	public load = () => {
		for (let i = 0; i < window.localStorage.length; i++) {
			const key = window.localStorage.key(i);

			if (key === null) continue;

			const item = window.localStorage.getItem(key);

			if (item === null) continue;

			const decodedItem: ApiWithSecret = JSON.parse(item);

			this.m_keys.push(decodedItem);
		}
	};
}

export const keyStore = new KeyStore();
