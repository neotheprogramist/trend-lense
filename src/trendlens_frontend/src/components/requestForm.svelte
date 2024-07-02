<script lang="ts" context="module">
  export type Fields<T> = {
    [K in keyof T]: T[K];
  };
</script>

<script lang="ts" generics="R">
  import type { IEnum } from "$lib/request";
  import BindableSelect from "./bindableSelect.svelte";
  import Input from "./shad/ui/input/input.svelte";

  interface IProps {
    request: Fields<R>;
  }

  let { request = $bindable() }: IProps = $props();
  let data = $state<Fields<R>>({ ...request });

  function handleInputChange(event: Event, key: keyof R) {
    const target = event.target as HTMLInputElement;
    data[key] = target.value as any;
  }

  function handleSelectChange(key: keyof R, value: string) {
    data[key] = value as any;
  }

  function handleNestedChange(event: CustomEvent, parentKey: keyof R) {
    data[parentKey] = {
      ...(data[parentKey] as object),
      ...event.detail,
    } as any;
  }

  function isEnum(value: any): value is IEnum {
    return typeof value == "object" && "isEnum" in value;
  }
</script>

{#snippet input(k: keyof R, inputType)}
  <Input
    type={inputType}
    value={data[k]}
    oninput={(e) => handleInputChange(e, k)}
  />
{/snippet}

<form class="space-y-2">
  {#each Object.keys(request) as key}
    {@const k = key as keyof R}
    {@const v = request[k]}

    <div>
      <label for="">{key}</label>
      {#if typeof v === "object" && !isEnum(v)}
        <svelte:self
          request={v}
          on:formChange={(e) => handleNestedChange(e, k)}
        />
      {:else if isEnum(v)}
        <BindableSelect
          items={v.getVariants()}
          onChange={(v) => handleSelectChange(k, v)}
          bind:value={v.v}
          placeholder={v.getName()}
        />
      {:else if typeof v === "number"}
        {@render input(k, "number")}
      {:else if typeof v === "string" && key.includes("date")}
        {@render input(k, "date")}
      {:else}
        {@render input(k, "text")}
      {/if}
    </div>
  {/each}
</form>
