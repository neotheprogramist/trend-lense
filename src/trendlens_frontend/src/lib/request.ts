import type { ValueOf } from "./apiAddition";

export enum RequestType {
  Empty = "Empty",
  GetInstruments = "GetInstruments",
  GetBalance = "GetBalance",
  PostOrder = "PostOrder",
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

export enum OrderSideType {
  Buy = "Buy",
  Sell = "Sell",
}

export class OrderSide implements IEnum {
  public variant: OrderSideType;

  constructor(t: OrderSideType) {
    this.variant = t;
  }

  get v() {
    return this.variant;
  }

  set v(v: OrderSideType) {
    this.variant = v;
  }

  public getVariants() {
    return Object.keys(OrderSideType);
  }

  public getName() {
    return "order side";
  }

  public isEnum() {
    return true;
  }
}

export enum TradeModeType {
  Cash = "Cash",
  SpotIsolated = "SpotIsolated",
  Isolated = "Isolated",
  Cross = "Cross",
}

export enum OrderTypeType {
  Fok = "Fok",
  Ioc = "Ioc",
  Limit = "Limit",
  PostOnly = "PostOnly",
  Market = "Market",
}

export enum PositionSideType {
  Short = "Short",
  Long = "Long",
}

export class OrderType implements IEnum {
  public variant: OrderTypeType;

  constructor(t: OrderTypeType) {
    this.variant = t;
  }

  get v() {
    return this.variant;
  }

  set v(v: OrderTypeType) {
    this.variant = v;
  }

  public getVariants() {
    return Object.keys(OrderTypeType);
  }

  public getName() {
    return "order type";
  }

  public isEnum() {
    return true;
  }
}

export class TradeMode implements IEnum {
  public variant: TradeModeType;

  constructor(t: TradeModeType) {
    this.variant = t;
  }

  get v() {
    return this.variant;
  }

  set v(v: TradeModeType) {
    this.variant = v;
  }

  public getVariants() {
    return Object.keys(TradeModeType);
  }

  public getName() {
    return "trade mode";
  }

  public isEnum() {
    return true;
  }
}

export class PositionSide implements IEnum {
  public variant: PositionSideType;

  constructor(t: PositionSideType) {
    this.variant = t;
  }

  get v() {
    return this.variant;
  }

  set v(v: PositionSideType) {
    this.variant = v;
  }

  public getVariants() {
    return Object.keys(PositionSideType);
  }

  public getName() {
    return "position side";
  }

  public isEnum() {
    return true;
  }
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
    return Object.keys(InstrumentType);
  }

  public getName() {
    return "instrument type";
  }

  public isEnum() {
    return true;
  }
}

export const InstrumentType = {
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
  instrumentId: string | null;
} & BaseRequest;

export type BalanceRequests = {
  currencies: Array<string>;
} & BaseRequest;

export type PostOrderRequest = {
  marginCurrency: string | null;
  size: number;
  instrumentId: string;
  orderPrice: number | null;
  side: OrderSide;
  tradeMode: TradeMode;
  orderType: OrderType;
  positionSide: PositionSide | null;
} & BaseRequest;

export type InstrumentType = ValueOf<typeof InstrumentType>;
export type ExchangeRequest =
  | InstrumentsRequest
  | BalanceRequests
  | PostOrderRequest;
