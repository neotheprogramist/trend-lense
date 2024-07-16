<script lang="ts">
  import { Exchanges } from "$lib/exchange";
  import Badge from "$components/shad/ui/badge/badge.svelte";

  interface IProps {
    selectedExchange: Exchanges;
  }

  let availableExchanges = Object.keys(Exchanges).map((e) => e as Exchanges);

  let { selectedExchange = $bindable() }: IProps = $props();
</script>

<div class="grid grid-cols-8 h-8 gap-2">
  {#each availableExchanges as e}
    <Badge
      variant={selectedExchange != e ? "outline" : undefined}
      onclick={() => (selectedExchange = e)}>{e}</Badge
    >
  {/each}
</div>
