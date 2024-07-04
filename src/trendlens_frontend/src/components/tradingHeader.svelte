<script lang="ts">
  import { Exchanges } from "$lib/exchange";
  import { exchangePairs } from "$lib/exchangePairs";
  import type { Pairs } from "$lib/pair";
  import BindableSelect from "./bindableSelect.svelte";

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

<div class="grid grid-cols-8 h-8 gap-2">
  <div class="col-span-2">
    <BindableSelect
      bind:value={selectedExchange}
      items={Object.keys(Exchanges)}
      placeholder="exchange"
    />
  </div>

  <div class="col-span-2">
    <BindableSelect
      bind:value={selectedPair}
      items={pairs}
      placeholder="pair"
      onChange={() => {
        if (selectedExchange && selectedPair) {
          onSelectionCompleted(selectedExchange, selectedPair);
        }
      }}
    />
  </div>
</div>
