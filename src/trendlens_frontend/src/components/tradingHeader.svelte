<script lang="ts">
  import Badge from "$components/shad/ui/badge/badge.svelte";
  import { Exchanges } from "$lib/exchange";

  interface IProps {
    selectedExchanges: Array<Exchanges>;
    availableExchanges: Array<Exchanges>;
  }

  let { selectedExchanges = $bindable(), availableExchanges }: IProps =
    $props();

  const toggleExchange = (e: Exchanges) => {
    if (selectedExchanges.includes(e)) {
      selectedExchanges = selectedExchanges.filter((ex) => ex !== e);
    } else {
      selectedExchanges = [...selectedExchanges, e];
    }
  };
</script>

{#if availableExchanges.length == 0}
  <Badge class="rounded-sm" variant="secondary">No exchanges available</Badge>
{:else}
  <div class="flex space-x-3">
    {#each availableExchanges as e}
      <Badge
        class="rounded-sm px-4 h-8"
        variant={!selectedExchanges.includes(e) ? "outline" : undefined}
        onclick={() => toggleExchange(e)}>{e}</Badge
      >
    {/each}
  </div>
{/if}
