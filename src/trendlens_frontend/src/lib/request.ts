export enum RequestType {
  Empty = "Empty",
  Instruments = "Instruments",
}

export enum RequestPickState {
  ApiRegistered,
  RequestPicked,
  RequestFilled,
}

export enum InstrumentType {
  Spot = 'Spot',
  Features = 'Features',
  Swap = 'Swap',
  Margin = 'Margin',
}

export type InstrumentsRequest = {
  instrumentType: InstrumentType;
  instrumentId: string;
};

export type Requests = InstrumentsRequest;