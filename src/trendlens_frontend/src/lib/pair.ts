import type { Pair } from "../../../declarations/trendlens_backend/trendlens_backend.did";

export enum Pairs {
	BtcUsd = 'BtcUsd',
	EthUsd = 'EthUsd'
}

export const handlePair = (pair: Pairs): Pair => {
	switch (pair) {
		case Pairs.BtcUsd:
			return { BtcUsd: null };
		case Pairs.EthUsd:
			return { EthUsd: null };
	}
};
