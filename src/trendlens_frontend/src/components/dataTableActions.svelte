<script lang="ts">
  import { Button } from "$components/shad/ui/button/index";
  import * as DropdownMenu from "$components/shad/ui/dropdown-menu/index";
  import { Ellipsis } from "lucide-svelte";

  export let id: string;
  export let removeCallback: (id: string) => void;
</script>

<DropdownMenu.Root>
  <DropdownMenu.Trigger asChild let:builder>
    <Button
      variant="ghost"
      builders={[builder]}
      size="icon"
      class="relative h-8 w-8 p-0"
    >
      <span class="sr-only">Open menu</span>
      <Ellipsis class="h-4 w-4" />
    </Button>
  </DropdownMenu.Trigger>
  <DropdownMenu.Content>
    <DropdownMenu.Group>
      <DropdownMenu.Label>Actions</DropdownMenu.Label>
      <DropdownMenu.Item on:click={() => removeCallback(id)}>
        Remove
      </DropdownMenu.Item>
    </DropdownMenu.Group>
  </DropdownMenu.Content>
</DropdownMenu.Root>
