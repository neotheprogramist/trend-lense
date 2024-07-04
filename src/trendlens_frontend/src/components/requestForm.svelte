<script lang="ts" context="module">
  export type Fields<T> = {
    [K in keyof T]: T[K];
  };
</script>

<script lang="ts" generics="R">
  import type { IEnum } from "$lib/request";
  import BindableSelect from "./bindableSelect.svelte";
  import Button from "./shad/ui/button/button.svelte";
  import Input from "./shad/ui/input/input.svelte";

  interface IProps {
    request: Fields<R>;
    onSubmit: () => void;
  }

  let { request = $bindable(), onSubmit }: IProps = $props();
  let keys = Object.keys(request).filter((k) => k !== "type");

  function handleInputChange(event: Event, key: keyof R) {
    const target = event.target as HTMLInputElement;
    request[key] = target.value as any;
  }

  function handleSelectChange(key: keyof R, value: string) {
    const obj = request[key];

    if (isEnum(obj)) {
      if (obj.getVariants().find((v) => v === value) !== undefined) {
        obj.v = value;
      } else {
        throw new Error(`Invalid value ${value} for enum ${key as string}`);
      }
    } else {
      request[key] = value as any;
    }

    request = request;
  }

  function handleNestedChange(event: CustomEvent, parentKey: keyof R) {
    request[parentKey] = {
      ...(request[parentKey] as object),
      ...event.detail,
    } as any;
  }

  function isEnum(value: any): value is IEnum {
    if (!value) return false;
    
    return typeof value == "object" && "isEnum" in value;
  }

  $inspect({ request });
</script>

{#snippet input(k: keyof R, inputType)}
  <Input
    type={inputType}
    value={request[k]}
    oninput={(e) => handleInputChange(e, k)}
  />
{/snippet}

<form class="space-y-2">
  {#each keys as key}
    {@const k = key as keyof R}
    {@const v = request[k]}
    {@debug k, v}

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

  <Button type="submit" on:click={onSubmit}>Submit</Button>
</form>
