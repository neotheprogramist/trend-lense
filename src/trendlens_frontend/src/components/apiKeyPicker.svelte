<script lang="ts">

  import { Exchanges, handleExchange } from "$lib/exchange";
  import { keyStore } from "$lib/keystore.svelte.ts";
  //   import * as Dialog from "$components/shad/ui/dialog/index";
  //   import Button from "./shad/ui/button/button.svelte";
  //   import { type ApiWithSecret } from "$lib/keystore.svelte";

  interface IProps {
    exchange: Exchanges;
    onApiKeyPick: () => void;
  }

  let { onApiKeyPick, exchange }: IProps = $props();

  const pickApiKey = (api_key: string) => {
    keyStore.focus(api_key);
    onApiKeyPick();
  };

  $effect(() => {
    if (keyStore.keys.length == 0) {
      keyStore.load();
    }
  });
</script>

<!-- {#if focussed && focussed.exchange == handleExchange(exchange)}
  <Button><Dialog.Trigger>Add</Dialog.Trigger></Button>

  <Dialog.Root>
    <Dialog.Content class="sm:max-w-[425px]">
      <Dialog.Header>
        <Dialog.Title>Api key select</Dialog.Title>
        <Dialog.Description
          >Tell us if you would like to use last key</Dialog.Description
        >
      </Dialog.Header>

      <Dialog.Footer>
        <Button type="submit" on:click={() => console.log("yy")}>Yes</Button>
        <Button type="submit" on:click={() => console.log("yy")}>No</Button>
      </Dialog.Footer>
    </Dialog.Content>
  </Dialog.Root>
{:else} -->
<ul>
  <!-- {#each keyStore.m_keys as key}
			<li>
				<button onclick={() => pickApiKey(key.api_key)}>{key.api_key}</button>
			</li>
		{/each} -->
  {#each keyStore.keys as key}
    <li>
      <button onclick={() => pickApiKey(key.api_key)}>{key.api_key}</button>
    </li>
  {/each}
</ul>
<!-- {/if} -->
