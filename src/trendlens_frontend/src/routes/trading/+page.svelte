<script lang="ts">
	import TradingHeader from '$components/tradingHeader.svelte';
	import TradingView from '$components/tradingView.svelte';
	import { anonymousBackend } from '$lib/canisters';
	import type { CandlestickData, UTCTimestamp } from 'lightweight-charts';
	import type { Candle } from '../../../../declarations/trendlens_backend/trendlens_backend.did';
	import type { SeriesDataItemTypeMap } from 'lightweight-charts';
	import * as Card from '$components/shad/ui/card/index';

	const timestamp_now = BigInt(Math.floor(Date.now() / 1000));
	const timestamp_minus_15m = timestamp_now - BigInt(15 * 60);

	console.log(timestamp_minus_15m, timestamp_now);
	let candlesFromBackend = $state<Candle[]>([]);

	// TODO: sorting should be done on backend
	let candlesTradingView = $derived.by<SeriesDataItemTypeMap['Candlestick'][]>(() => {
		return candlesFromBackend
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
	});

	$inspect(candlesTradingView);
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
					<TradingHeader
						completeSelection={async (exchange, pair) => {
							// TODO: replace these types with actual enums from front, map it somehow
							candlesFromBackend = await anonymousBackend.pull_candles(
								{ BtcUsd: null },
								{ Okx: null },
								timestamp_minus_15m,
								timestamp_now
							);
						}}
					/>
					<TradingView candlesData={candlesTradingView} />
				</Card.Content>
			</Card.Root>
		</div>
	</div>
</div>
