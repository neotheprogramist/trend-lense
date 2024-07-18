import type {
  GeneralBalanceRequest,
  GeneralInstrumentsRequest,
  GeneralPostOrderRequest,
  Request,
} from "../../../declarations/trendlens_backend/trendlens_backend.did";
import type { ValueOf } from "./apiAddition";

export enum RequestType {
  Empty = "Empty",
  GetInstruments = "Get instruments",
  GetBalance = "Get balance",
  PostOrder = "Post order",
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
  Market = "Market",
  Limit = "Limit",
  Ioc = "Ioc",
  Fok = "Fok",
  PostOnly = "PostOnly",
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

export enum InstrumentType {
  Spot = "Spot",
  Features = "Features",
  Swap = "Swap",
  Margin = "Margin",
}

type BaseRequest = {
  type: RequestType;
};

export type InstrumentsRequest = {
  instrumentType: Instruments;
  instrumentId: OptionalField<string>;
} & BaseRequest;

export type BalanceRequests = {
  currencies: OptionalField<string>;
} & BaseRequest;

export interface IOptional<T> {
  value: T | null;
}

export class OptionalField<T> implements IOptional<T> {
  public value: T | null;

  constructor(v: T | null) {
    this.value = v;
  }
}

export type PostOrderRequest = {
  marginCurrency: OptionalField<string>;
  size: number;
  instrumentId: string;
  orderPrice: OptionalField<number>;
  side: OrderSide;
  tradeMode: TradeMode;
  orderType: OrderType;
  positionSide: OptionalField<PositionSide>;
} & BaseRequest;

export type ExchangeRequest =
  | InstrumentsRequest
  | BalanceRequests
  | PostOrderRequest;


export function isInstrumentsRequest(
  request: Request,
): request is { Instruments: GeneralInstrumentsRequest } {
  return (
    (request as { Instruments: GeneralInstrumentsRequest }).Instruments !==
    undefined
  );
}

export function isPostOrderRequest(
  request: Request,
): request is { PostOrder: GeneralPostOrderRequest } {
  return (
    (request as { PostOrder: GeneralPostOrderRequest }).PostOrder !== undefined
  );
}

export function isBalancesRequest(
  request: Request,
): request is { Balances: GeneralBalanceRequest } {
  return (
    (request as { Balances: GeneralBalanceRequest }).Balances !== undefined
  );
}
