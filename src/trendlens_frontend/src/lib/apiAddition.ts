import type { ApiData as BackendApiData } from "../../../declarations/trendlens_backend/trendlens_backend.did";
import { handleExchange } from "./exchange";
import type { ApiData } from "./keystore.svelte";

const BaseStatus = {
  NotConnected: "wallet not connected",
} as const;

const ApiStatus = {
  Registered: "key registered",
  InvalidApiData: "provided data invalid",
} as const;

export const handleApiData = (data: ApiData): BackendApiData => {
  return {
    api_key: data.apiKey,
    passphrase: [data.passphrase],
    exchange: handleExchange(data.exchange),
  };
};

export type ValueOf<T> = T[keyof T];
export const ApiRegisterStatus = { ...BaseStatus, ...ApiStatus } as const;
export type ApiRegisterStatusType = ValueOf<typeof ApiRegisterStatus>;
