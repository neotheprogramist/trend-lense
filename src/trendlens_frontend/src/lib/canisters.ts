import { createActor, canisterId } from '../../../declarations/trendlens_backend/index';
import { AuthClient } from '@dfinity/auth-client';
import type { _SERVICE } from '../../../declarations/trendlens_backend/trendlens_backend.did';
import { canisterId as identityCanisterId } from '../../../declarations/internet_identity';

export const anonymousBackend = createActor(canisterId);

export const connect = async () => {
	let authClient = await AuthClient.create();

	await new Promise((resolve) => {
		authClient.login({
			identityProvider:
				process.env.DFX_NETWORK === 'ic'
					? 'https://identity.ic0.app'
					: `http://${identityCanisterId}.localhost:4943/`,
			onSuccess: () => resolve(undefined)
		});
	});

	const identity = authClient.getIdentity();
	const actor = createActor(canisterId, { agentOptions: { identity } });

	return { actor, identity };
};
