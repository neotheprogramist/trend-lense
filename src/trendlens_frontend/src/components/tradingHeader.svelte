<script lang="ts">
  import { Exchanges } from "$lib/exchange";
  import Badge from "$components/shad/ui/badge/badge.svelte";

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
  <Badge class="rounded-sm" variant="secondary">no exchanges available</Badge>
{:else}
  <div class="grid h-8 grid-cols-8 gap-2">
    {#each availableExchanges as e}
      <Badge
        class="rounded-sm"
        variant={!selectedExchanges.includes(e) ? "outline" : undefined}
        onclick={() => toggleExchange(e)}>{e}</Badge
      >
    {/each}
  </div>
{/if}
