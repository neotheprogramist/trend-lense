<script lang="ts">
  import type { InstrumentType } from "$lib/request";
  import { onMount, untrack } from "svelte";
  import BindableSelect from "./bindableSelect.svelte";
  import { instrumentsStore } from "$lib/instruments.svelte";
  import { wallet } from "$lib/wallet.svelte";

  interface IProps {
    instrumentType: InstrumentType;
    onInstrumentSelect: (ins: string) => void;
  }

  let { instrumentType, onInstrumentSelect }: IProps = $props();
  let currentInstrument = $state<string | null>(null);


  // @ts-ignore
  $effect(async () => {
    console.log('running effect ')
    if (wallet.connected && wallet.actor) {
      
      untrack(async () => {
        await instrumentsStore.filterByUser(instrumentType, false);
      });
    }
  });
</script>

<BindableSelect
  bind:value={currentInstrument}
  items={instrumentsStore.filteredInstruments.map(
    (e) => e.base + "-" + e.quote,
  )}
  placeholder="instruments"
  onChange={(v) => {
    if (v) {
      onInstrumentSelect(v);
    }
  }}
/>
