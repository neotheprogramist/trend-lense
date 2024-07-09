<script lang="ts" context="module">
  export type Fields<T> = {
    [K in keyof T]: T[K];
  };
</script>

<script lang="ts" generics="R">
  import type { IEnum, IOptional } from "$lib/request";
  import BindableSelect from "./bindableSelect.svelte";
  import Input from "./shad/ui/input/input.svelte";

  interface IProps {
    request: Fields<R>;
  }

  let { request = $bindable() }: IProps = $props();

  const keys = Object.keys(request).filter((k) => k !== "type");

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

  function isIOptional<T>(value: any): value is IOptional<T> {
    return (
      typeof value === "object" &&
      value !== null &&
      "value" in value &&
      (value.value === null || typeof value.value !== "undefined")
    );
  }
</script>

{#snippet input(k: keyof R, inputType)}
  <Input
    type={inputType}
    value={request[k]}
    oninput={(e) => handleInputChange(e, k)}
  />
{/snippet}

<form class="w-100% space-x-2 grid grid-cols-2">
  {#each keys as key}
    {@const k = key as keyof R}
    {@const v = request[k]}
    <div>
      {#if v == null}
        {#if isIOptional(request) && isEnum(request.value)}
          <BindableSelect
            items={request.value.getVariants()}
            onChange={(v) => handleSelectChange(k, v)}
            bind:value={request.value.v}
            placeholder={request.value.getName()}
          />
        {:else if isIOptional<string>(request)}
          <Input
            placeholder="not required"
            type="text"
            value={request[k]}
            oninput={(e) => handleInputChange(e, k)}
          />
        {/if}
      {:else if typeof v === "object" && !isEnum(v)}
        <label for="">{key}</label>
        <svelte:self
          request={v}
          on:formChange={(e) => handleNestedChange(e, k)}
        />
      {:else if isEnum(v)}
        <label for="">{key}</label>
        <BindableSelect
          items={v.getVariants()}
          onChange={(v) => handleSelectChange(k, v)}
          bind:value={v.v}
          placeholder={v.getName()}
        />
      {:else if typeof v === "number"}
        <label for="">{key}</label>
        {@render input(k, "number")}
      {:else if typeof v === "string" && key.includes("date")}
        <label for="">{key}</label>
        {@render input(k, "date")}
      {:else}
        <label for="">{key}</label>
        {@render input(k, "text")}
      {/if}
    </div>
  {/each}
</form>
