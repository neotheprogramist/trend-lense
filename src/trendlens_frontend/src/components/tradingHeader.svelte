<script lang="ts">
	import { Exchanges } from '$lib/exchange';
	import { mockAvailablePairs } from '$lib/mockPairs';

	// TODO: decide whether to do fetch logic here or outside of this component

  // PROPS DEFINITIONS
	interface TradingHeaderProps {
		completeSelection: (exchange: Exchanges, pair: string) => void;
	}
	let { completeSelection }: TradingHeaderProps = $props();
  // 


	let selectedExchange = $state<Exchanges>(Exchanges.Okx);
	let selectedPair = $state<string | null>(null);

	// should be taken from outside of fetched from api here
	let pairs = $derived.by(() => {
		if (selectedExchange in mockAvailablePairs) {
			return mockAvailablePairs[selectedExchange];
		} else {
			return [];
		}
	});
</script>

<div class="inline-grid grid-cols-3 gap-1 w-100">
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
				completeSelection(selectedExchange, selectedPair);
			}
		}}
	>
		{#each pairs as pair}
			<option value={pair}>{pair}</option>
		{/each}
	</select>
</div>
