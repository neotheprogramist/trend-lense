<svelte:options runes={false} />

<script lang="ts">
	import type { SeriesDataItemTypeMap } from 'lightweight-charts';
	import { CandlestickSeries, Chart } from 'svelte-lightweight-charts';

	export let candlesData: SeriesDataItemTypeMap['Candlestick'][] = [];


  $: console.log(candlesData)
</script>

<Chart width={800} height={600}>
	<CandlestickSeries
		reactive={true}
		data={candlesData}
		upColor="rgba(255, 144, 0, 1)"
		downColor="#000"
		borderDownColor="rgba(255, 144, 0, 1)"
		borderUpColor="rgba(255, 144, 0, 1)"
		wickDownColor="rgba(255, 144, 0, 1)"
		wickUpColor="rgba(255, 144, 0, 1)"
	></CandlestickSeries>
</Chart>
