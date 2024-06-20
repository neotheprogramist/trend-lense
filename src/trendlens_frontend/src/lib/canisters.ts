import { createActor, canisterId } from '../../../declarations/trendlens_backend/index';
import { AuthClient } from '@dfinity/auth-client';
import { HttpAgent, type ActorSubclass } from '@dfinity/agent';
import type { _SERVICE } from '../../../declarations/trendlens_backend/trendlens_backend.did';

export const anonymousBackend = createActor(canisterId);

export const connect = async (): Promise<ActorSubclass<_SERVICE>> => {
	let authClient = await AuthClient.create();

	await new Promise((resolve) => {
		authClient.login({
			identityProvider:
				process.env.DFX_NETWORK === 'ic'
					? 'https://identity.ic0.app'
					: `http://${process.env.IDENTITY_CANISTER_ID}.localhost:4943/`,
			onSuccess: () => resolve(undefined)
		});
	});
	const identity = authClient.getIdentity();

	console.log(identity.getPrincipal().toText());

	const agent = new HttpAgent({ identity });
	return createActor(canisterId, { agent });
};
