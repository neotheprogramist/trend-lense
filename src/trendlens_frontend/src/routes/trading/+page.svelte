<script lang="ts">
	import TradingHeader from '$src/components/tradingHeader.svelte';
	import TradingView from '$src/components/tradingView.svelte';
	import { anonymousBackend } from '$src/lib/canisters';
	import type { CandlestickData, UTCTimestamp } from 'lightweight-charts';
	import type { Candle } from '../../../../declarations/trendlens_backend/trendlens_backend.did';
	import type { SeriesDataItemTypeMap } from 'lightweight-charts';

	const timestamp_now = BigInt(Math.floor(Date.now() / 1000));
	const timestamp_minus_15m = timestamp_now - BigInt(15 * 60);

	console.log(timestamp_minus_15m, timestamp_now);
	let candlesFromBackend = $state<Candle[]>([]);

	let candlesTradingView = $derived.by<SeriesDataItemTypeMap['Candlestick'][]>(() => {
		return candlesFromBackend.map((candle) => {
			return {
				close: candle.close_price,
				high: candle.highest_price,
				low: candle.lowest_price,
				open: candle.open_price,
				time: Number(candle.timestamp) as UTCTimestamp
			};
		});
	});

	$inspect(candlesTradingView);
</script>

<div>
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
</div>
