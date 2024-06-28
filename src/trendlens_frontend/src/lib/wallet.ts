import { writable } from 'svelte/store';
import { connect } from './canisters';
import type { ActorSubclass, Identity } from '@dfinity/agent';
import type { _SERVICE } from '../../../declarations/trendlens_backend/trendlens_backend.did';

const createWallet = () => {
	const { subscribe, set, update } = writable<{ connected: false } | IWallet>({
		connected: false
	});

	return {
		subscribe,
		connect: async () => {
			console.log('connecting')
			const { actor, identity } = await connect();
			set({ connected: true, actor, identity });
		},
		disconnect: () => {
			set({ connected: false });
		}
	};
};

export const wallet = createWallet();

export interface IWallet {
	connected: true;
	actor: ActorSubclass<_SERVICE>;
	identity: Identity;
}


