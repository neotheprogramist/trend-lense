<script lang="ts">
	import { Exchanges } from '$lib/exchange';
	import { mockAvailablePairs } from '$lib/mockPairs';
	import type { Pairs } from '$lib/pair';
	import Search from './search.svelte';

	interface TradingHeaderProps {
		onSelectionCompleted: (exchange: Exchanges, pair: Pairs) => void;
	}
	let { onSelectionCompleted }: TradingHeaderProps = $props();

	let selectedExchange = $state<Exchanges>(Exchanges.Okx);
	let selectedPair = $state<Pairs | null>(null);

	// should be taken from outside of fetched from api here
	let pairs = $derived.by(() => {
		if (selectedExchange in mockAvailablePairs) {
			return mockAvailablePairs[selectedExchange];
		} else {
			return [];
		}
	});
</script>

<div class="flex h-10 px-6">
	<select name="exchange" class="rounded-full grid-col-1" bind:value={selectedExchange}>
		{#each Object.keys(Exchanges) as exchangeName}
			<option value={exchangeName}>{exchangeName}</option>
		{/each}
	</select>

	<select
		name="pair"
		class="rounded-full grid-col-2"
		bind:value={selectedPair}
		onchange={() => {
			if (selectedPair) {
				onSelectionCompleted(selectedExchange, selectedPair);
			}
		}}
	>
		{#each pairs as pair}
			<option value={pair}>{pair}</option>
		{/each}
	</select>

	<div class="ml-auto flex items-center">
		<Search />
	</div>
</div>
