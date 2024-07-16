<script lang="ts">
  import * as Tabs from "$components/shad/ui/tabs";
  import { tradingMode } from "$lib/tradeMode.svelte";
  let { children } = $props();
</script>

<Tabs.Root
  value="single"
  class="mt-5"
  onValueChange={(v) => {
    if (v !== undefined) {
      console.log(v)
      tradingMode.switch();
    }
  }}
>
  <Tabs.List>
    <Tabs.Trigger value="single">Single</Tabs.Trigger>
    <Tabs.Trigger value="multiple">Multiple</Tabs.Trigger>
  </Tabs.List>
  <Tabs.Content value="single">
    {@render children()}
  </Tabs.Content>
  <!-- <Tabs.Content value="multiple">{@render children()}</Tabs.Content> -->
</Tabs.Root>
