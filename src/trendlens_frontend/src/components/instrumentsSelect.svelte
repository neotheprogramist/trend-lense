<script lang="ts">
  import { Exchanges, handleExchange } from "$lib/exchange";
  import { handleInstrumentType } from "$lib/instrumentType";
  import { keyStore } from "$lib/keystore.svelte";
  import type { InstrumentType } from "$lib/request";
  import { isInstrumentsResponse } from "$lib/response";
  import { extractOkValue } from "$lib/result";
  import { finishSignature } from "$lib/signature";
  import { wallet } from "$lib/wallet.svelte";
  // import { Toaster } from "$components/shad/ui/sonner/index";
  // import { toast } from "svelte-sonner";
  import type {
    Instrument,
    Pair,
  } from "../../../declarations/trendlens_backend/trendlens_backend.did";
  import { onMount } from "svelte";
  import BindableSelect from "./bindableSelect.svelte";
  import { instrumentsStore } from "$lib/instruments.svelte";
  import { PaintRoller } from "lucide-svelte";

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
