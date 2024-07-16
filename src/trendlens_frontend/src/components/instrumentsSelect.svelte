<script lang="ts">
  import type { InstrumentType } from "$lib/request";
  import { onMount, untrack } from "svelte";
  import BindableSelect from "./bindableSelect.svelte";
  import { instrumentsStore } from "$lib/instruments.svelte";
  import { wallet } from "$lib/wallet.svelte";
  import type { Pair } from "../../../declarations/trendlens_backend/trendlens_backend.did";

  interface IProps {
    instrumentType: InstrumentType;
    onInstrumentSelect: (instrument: Pair) => void;
  }

  let { instrumentType, onInstrumentSelect }: IProps = $props();
  let currentInstrument = $state<string | null>(null);

  // // @ts-ignore
  // $effect(async () => {
  //   console.log("running effect ");
  //   if (wallet.connected && wallet.actor) {
  //     untrack(async () => {
  //       await instrumentsStore.filterByUser(instrumentType, false);
  //     });
  //   }
  // });
</script>

{#await instrumentsStore.getUniqueInstruments(instrumentType)}
  loading instruments...
{:then instruments}
  {@const instrumentNames = instruments.map((e) => e.base + "-" + e.quote)}

  <BindableSelect
    value={null}
    items={instrumentNames}
    placeholder="instruments"
    onChange={(v) => {
      if (v) {
        const pair = v.split("-");
        onInstrumentSelect({ base: pair[0], quote: pair[1] });
      }
    }}
  />
{/await}
