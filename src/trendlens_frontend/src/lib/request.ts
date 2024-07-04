import type { ValueOf } from "./apiAddition";

export enum RequestType {
  Empty = "Empty",
  Instruments = "Instruments",
}

export enum RequestPickState {
  ApiRegistered,
  RequestPicked,
  RequestFilled,
}

// should be generic on setters and getters
// but i use it in specific place where
// i can't infer those generics
export interface IEnum {
  isEnum(): boolean;
  getVariants(): string[];
  getName(): string;
  get v(): string;
  set v(v: string);
}

export class Instruments implements IEnum {
  public variant: InstrumentType;

  constructor(t: InstrumentType) {
    this.variant = t;
  }

  get v() {
    return this.variant;
  }

  set v(v: InstrumentType) {
    this.variant = v;
  }

  public getVariants() {
    return Object.keys(Instrument);
  }

  public getName() {
    return "instrument type";
  }

  public isEnum() {
    return true;
  }
}

export const Instrument = {
  Spot: "Spot",
  Features: "Features",
  Swap: "Swap",
  Margin: "Margin",
} as const;

type BaseRequest = {
  type: RequestType;
};

export type InstrumentsRequest = {
  instrumentType: Instruments;
  instrumentId: string;
} & BaseRequest;

export type SomeMockRequest = {
  id: number;
} & BaseRequest;

export type InstrumentType = ValueOf<typeof Instrument>;
export type ExchangeRequest = InstrumentsRequest | SomeMockRequest;
