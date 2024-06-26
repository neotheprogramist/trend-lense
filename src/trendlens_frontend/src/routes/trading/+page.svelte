<script lang="ts">
	import TradingHeader from '$components/tradingHeader.svelte';
	import TradingView from '$components/tradingView.svelte';
	import { anonymousBackend } from '$lib/canisters';
	import type { CandlestickData, UTCTimestamp } from 'lightweight-charts';
	import type {
		Candle,
		Pair as CandidPair,
		Exchange as CandidExchange
	} from '../../../../declarations/trendlens_backend/trendlens_backend.did';
	import type { SeriesDataItemTypeMap } from 'lightweight-charts';
	import * as Card from '$components/shad/ui/card/index';
	import { Exchanges } from '$lib/exchange';
	import { Pair } from '$lib/pair';

	const ONE_MINUTE = 60 * 1000;
	const ONE_HOUR = 60 * ONE_MINUTE;

	let candlesFromBackend = $state<SeriesDataItemTypeMap['Candlestick'][]>([]);
	let selectedExchange = $state<Exchanges | null>(null);
	let selectedPair = $state<Pair | null>(null);

	let fetchInterval = $state(ONE_MINUTE);
	let interval = $state<NodeJS.Timeout | null>(null);
	let lastTimestamp = $state<number>(Date.now() - ONE_HOUR);
	let stopTimestamp = $state<number>(Date.now());

	const transformCandleData = (candles: Candle[]): SeriesDataItemTypeMap['Candlestick'][] => {
		return candles
			.map((candle) => {
				return {
					close: candle.close_price,
					high: candle.highest_price,
					low: candle.lowest_price,
					open: candle.open_price,
					time: Number(candle.timestamp) as UTCTimestamp
				};
			})
			.sort((a, b) => a.time - b.time);
	};

	const handlePair = (pair: Pair): CandidPair => {
		switch (pair) {
			case Pair.BtcUsd:
				return { BtcUsd: null };
			case Pair.EthUsd:
				return { EthUsd: null };
		}
	};

	const handleExchange = (exchange: Exchanges): CandidExchange => {
		switch (exchange) {
			case Exchanges.Okx:
				return { Okx: null };
			case Exchanges.Coinbase:
				return { Coinbase: null };
		}
	};

	const fetchNewCandles = async () => {
		stopTimestamp = Date.now();

		console.log('Fetching candles from', lastTimestamp, 'to', stopTimestamp);

		const newCandles = await anonymousBackend.pull_candles(
			handlePair(selectedPair!),
			handleExchange(selectedExchange!),
			BigInt(Math.floor(lastTimestamp / 1000)),
			BigInt(Math.floor(stopTimestamp / 1000))
		);

		lastTimestamp =
			Number(await anonymousBackend.get_last_timestamp(handleExchange(selectedExchange!))) * 1000 +
			1;

		console.log(lastTimestamp, stopTimestamp);

		const transformedCandles = transformCandleData(newCandles);
		candlesFromBackend = [...candlesFromBackend, ...transformedCandles];

		// invoke reactivity
		candlesFromBackend = candlesFromBackend;
	};

	const fetchCandles = (exchange: Exchanges, pair: Pair): void => {
		selectedExchange = exchange;
		selectedPair = pair;

		candlesFromBackend = [];

		if (interval) {
			clearInterval(interval);
		}

		fetchNewCandles();
		interval = setInterval(fetchNewCandles, fetchInterval);
	};

	$inspect(candlesFromBackend);
</script>

<div class="flex-1 flex justify-center mt-5">
	<div class="flex-col md:flex">
		<div class="grid gap-2 md:grid-cols-2 lg:grid-cols-7">
			<Card.Root class="col-span-3">
				<Card.Header>
					<Card.Title>Recent trades</Card.Title>
				</Card.Header>
				<Card.Content>There may be some trades or other statistics</Card.Content>
			</Card.Root>

			<Card.Root class="col-span-4">
				<Card.Header>
					<Card.Title>Price chart</Card.Title>
				</Card.Header>
				<Card.Content>
					<TradingHeader onSelectionCompleted={fetchCandles} />
					<TradingView candlesData={candlesFromBackend} />
				</Card.Content>
			</Card.Root>
		</div>
	</div>
</div>
