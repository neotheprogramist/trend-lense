<script lang="ts" context="module">
  export type FormFields<T> = {
    [K in keyof T]: T[K];
  };
</script>

<script lang="ts" generics="R">
  interface IProps {
    request: FormFields<R>;
  }

  let { request }: IProps = $props();

  let formData = $state<FormFields<R>>({ ...request });

  function handleInputChange(event: Event, key: keyof R) {
    const target = event.target as HTMLInputElement;
    formData[key] = target.value as any;

    console.log(formData[key]);
  }
  function handleNestedChange(event: CustomEvent, parentKey: keyof R) {
    formData[parentKey] = {
      ...(formData[parentKey] as object),
      ...event.detail,
    } as any;
    console.log(formData);
  }

  function isEnum(value: any): boolean {
    return (
      typeof value === "object" && Object.values(value).every((v) => v == null)
    );
  }
</script>

<form>
  {#if request}
    {#each Object.keys(request) as key}
      <div>
        <label>{key}</label>
        {#if typeof request[key as keyof R] === "object" && !isEnum(request[key as keyof R])}
          <svelte:self
            request={request[key as keyof R]}
            on:formChange={(e) => handleNestedChange(e, key as keyof R)}
          />
        {:else if isEnum(request[key as keyof R])}
          <select onchange={(e) => handleInputChange(e, key as keyof R)}>
            {#each Object.keys(request[key as keyof R] as ArrayLike<any>) as value}
              <option {value}>{value}</option>
            {/each}
          </select>
        {:else if typeof request[key as keyof R] === "number"}
          <input
            type="number"
            value={formData[key as keyof R] as number}
            oninput={(e) => handleInputChange(e, key as keyof R)}
          />
        {:else if typeof request[key as keyof R] === "string" && key.includes("date")}
          <input
            type="date"
            value={formData[key as keyof R] as string}
            oninput={(e) => handleInputChange(e, key as keyof R)}
          />
        {:else}
          <input
            type="text"
            value={formData[key as keyof R] as string}
            oninput={(e) => handleInputChange(e, key as keyof R)}
          />
        {/if}
      </div>
    {/each}
  {/if}
</form>
