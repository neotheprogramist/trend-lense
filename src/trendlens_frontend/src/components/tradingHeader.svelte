<script lang="ts">
  import { Exchanges } from "$lib/exchange";
  import { exchangePairs } from "$lib/exchangePairs";
  import type { Pairs } from "$lib/pair";
  import BindableSelect from "./bindableSelect.svelte";

  interface IProps {
    onSelectionCompleted: (pair: Pairs) => void;
    selectedExchange: Exchanges;
  }
  let { onSelectionCompleted, selectedExchange = $bindable() }: IProps =
    $props();

  let selectedPair = $state<Pairs | null>(null);

  // should be taken from outside of fetched from api here
  let pairs = $derived.by(() => {
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
        if (selectedPair) {
          onSelectionCompleted(selectedPair);
        }
      }}
    />
  </div>
</div>
