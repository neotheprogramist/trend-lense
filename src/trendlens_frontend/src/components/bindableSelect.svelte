<script lang="ts" generics="T">
  import ScrollArea from "./shad/ui/scroll-area/scroll-area.svelte";
  import * as Select from "./shad/ui/select/index";

  interface IProps {
    placeholder?: string;
    value: T;
    items: T[];
    onChange?: (v: T) => void;
  }

  let { value = $bindable(), items, placeholder, onChange }: IProps = $props();
</script>

<Select.Root
  items={items.map((e) => {
    return { value: e };
  })}
  onSelectedChange={(e) => {
    if (e !== undefined) {
      value = e.value;

      if (onChange) {
        onChange(e.value);
      }
    }
  }}
>
  <Select.Trigger class="w-100%">
    {#if placeholder !== undefined}
      <Select.Value {placeholder} />
    {/if}
  </Select.Trigger>
  <Select.Content>
    <ScrollArea class="h-[200px] w-full rounded-md border p-4">
      {#each items as key}
        <Select.Item value={key}>{key}</Select.Item>
      {/each}
    </ScrollArea>
  </Select.Content>
</Select.Root>
