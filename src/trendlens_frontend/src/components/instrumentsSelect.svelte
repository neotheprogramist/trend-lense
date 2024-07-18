<script lang="ts">
  import { Badge } from "$components/shad/ui/badge";
  import * as Command from "$components/shad/ui/command";
  import { instrumentsStore } from "$lib/instruments.svelte";
  import type { InstrumentType } from "$lib/request";
  import type { Pair } from "../../../declarations/trendlens_backend/trendlens_backend.did";

  interface IProps {
    instrumentType: InstrumentType;
    onInstrumentSelect: (instrument: Pair) => void;
  }

  let { instrumentType, onInstrumentSelect }: IProps = $props();

  const onPairSelect = (v: string) => {
    console.log(v);
    const upperCase = v.toUpperCase();
    const [base, quote] = upperCase.split("-");
    onInstrumentSelect({ base, quote });
  };

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

<Command.Root
  class="h-full"
  filter={(value, search) => {
    const lowercaseValue = value.toLowerCase();
    const lowercaseSearch = search.toLowerCase();

    if (lowercaseValue.includes(lowercaseSearch)) return 1;
    return 0;
  }}
>
  <Command.Input placeholder="Type a instrument to search..." />
  <Command.List>
    {#await instrumentsStore.getUniqueInstruments(instrumentType)}
      <Command.Loading>Loading…</Command.Loading>
    {:then instruments}
      <Command.Empty>No results found.</Command.Empty>
      <Command.Group heading="instruments">
        {#each instruments as instrument}
          {@const count = instrument.count}
          {@const name = instrument.pair.base + "-" + instrument.pair.quote}

          <div class="grid grid-cols-6">
            <Command.Item class="col-span-4" onSelect={onPairSelect}
              >{name}</Command.Item
            >
            <div class="col-span-2 flex justify-center">
              <Badge
                variant="secondary"
                class="flex h-6 w-12 shrink-0 items-center justify-center rounded-full"
              >
                {count}
              </Badge>
            </div>
          </div>
        {/each}
      </Command.Group>
    {/await}
  </Command.List>
</Command.Root>
