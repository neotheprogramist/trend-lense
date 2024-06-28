import { Pairs } from './pair';

// this is for mocking purposes, should be replaced by fetching available trading instruments from an api endpoint
export const exchangePairs = {
	Okx: [Pairs.BtcUsd, Pairs.EthUsd],
	Coinbase: [],
	Kraken: []
};
