import { AuthClient } from "@dfinity/auth-client";
import { canisterId as identityCanisterId } from "../../../declarations/internet_identity";
import {
  canisterId,
  createActor,
} from "../../../declarations/trendlens_backend/index";

export const anonymousBackend = createActor(canisterId);

export const connect = async () => {
  const authClient = await AuthClient.create();

  await new Promise((resolve) => {
    authClient.login({
      identityProvider:
        process.env.DFX_NETWORK === "ic"
          ? "https://identity.ic0.app"
          : `http://${identityCanisterId}.localhost:4943/`,
      onSuccess: () => resolve(undefined),
    });
  });

  const identity = authClient.getIdentity();
  const actor = createActor(canisterId, { agentOptions: { identity } });

  return { actor, identity };
};
