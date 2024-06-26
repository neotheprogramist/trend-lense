export enum Exchanges {
	Okx = 'Okx',
	Coinbase = 'Coinbase'
}

export type ExchangeKey = keyof typeof Exchanges;

export type CandleStickData = {
	timestamp: number;
	open_price: number;
	close_price: number;
	lowest_price: number;
	highest_price: number;
};
