<script lang="ts">
  import type { InstrumentType } from "$lib/request";
  import { onMount } from "svelte";
  import BindableSelect from "./bindableSelect.svelte";
  import { instrumentsStore } from "$lib/instruments.svelte";

  interface IProps {
    instrumentType: InstrumentType;
    onInstrumentSelect: (ins: string) => void;
  }

  let { instrumentType, onInstrumentSelect }: IProps = $props();
  let currentInstrument = $state<string | null>(null);

  onMount(async () => {
    await instrumentsStore.filterByType(instrumentType);
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
