<script lang="ts">
	import { Exchanges } from '$lib/exchange';
	import { exchangePairs } from '$lib/exchangePairs';
	import type { Pairs } from '$lib/pair';
	import BindableSelect from './bindableSelect.svelte';

	interface TradingHeaderProps {
		onSelectionCompleted: (exchange: Exchanges, pair: Pairs) => void;
	}
	let { onSelectionCompleted }: TradingHeaderProps = $props();

	let selectedExchange = $state<Exchanges | null>(null);
	let selectedPair = $state<Pairs | null>(null);

	// should be taken from outside of fetched from api here
	let pairs = $derived.by(() => {
		if (!selectedExchange) {
			return [];
		}

		if (selectedExchange in exchangePairs) {
			return exchangePairs[selectedExchange];
		} else {
			return [];
		}
	});
</script>

<div class="flex h-10 px-6">
	<BindableSelect
		bind:value={selectedExchange}
		items={Object.keys(Exchanges)}
		placeholder={'exchange'}
	/>

	<BindableSelect
		bind:value={selectedPair}
		items={pairs}
		placeholder={'pair'}
		onChange={() => {
			if (selectedExchange && selectedPair) {
				onSelectionCompleted(selectedExchange, selectedPair);
			}
		}}
	/>
</div>
