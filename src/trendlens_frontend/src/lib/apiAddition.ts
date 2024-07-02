const BaseStatus = {
  NotConnected: "wallet not connected",
} as const;

const ApiStatus = {
  Registered: "key registered",
  InvalidApiData: "provided data invalid",
} as const;

type ValueOf<T> = T[keyof T];

export const ApiRegisterStatus = { ...BaseStatus, ...ApiStatus } as const;
export type ApiRegisterStatusType = ValueOf<typeof ApiRegisterStatus>;
