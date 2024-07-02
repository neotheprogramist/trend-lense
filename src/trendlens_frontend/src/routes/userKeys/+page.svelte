<script lang="ts">
  import ApiKeyDialog from "$components/apiKeyDialog.svelte";
  import DataTable from "$components/dataTable.svelte";
  import { wallet } from "$lib/wallet.svelte";
  import type { ApiData } from "../../../../declarations/trendlens_backend/trendlens_backend.did";
  import { handleExchange, type Exchanges } from "$lib/exchange";
  import { keyStore, type ApiWithSecret } from "$lib/keystore.svelte";
  import {
    ApiRegisterStatus,
    type ApiRegisterStatusType,
  } from "$lib/apiAddition";
    import { onMount } from "svelte";

  const saveApiData = async (data: ApiWithSecret) => {
    if (!wallet.connected || !wallet.actor) {
      console.log("Wallet not connected");
      return;
    }

    keyStore.add(data);
  };

  const removeApiKey = async (apiKey: string) => {
		console.log('removing')
    if (!wallet.connected || !wallet.actor) {
      return;
    }

    try {
      await wallet.actor.remove_api_key(apiKey);
      keyStore.remove(apiKey);
    } catch (err) {
      console.error(err);
    }
  };

  const addApiKey = async (
    data: ApiWithSecret,
  ): Promise<ApiRegisterStatusType> => {
    if (!wallet.connected || !wallet.actor) {
      return ApiRegisterStatus.NotConnected;
    }

    const apiData: ApiData = {
      api_key: data.apiKey,
      passphrase: [data.passphrase],
      exchange: handleExchange(data.exchange),
    };

    const done = await wallet.actor.register_api_key(apiData);

    if (done) {
      saveApiData(data);

      return ApiRegisterStatus.Registered;
    } else {
      return ApiRegisterStatus.InvalidApiData;
    }
  };

	onMount(() => {
		keyStore.load()
	})
</script>

<div class="container mx-auto py-10">
  <div class="flex items-center py-4">
    <ApiKeyDialog onUpload={addApiKey} />
  </div>

  <div class="rounded-md border">
    <DataTable data={keyStore.keys} removeCallback={removeApiKey}>
      {#if wallet.connected}
        A list of your api keys
      {:else}
        Please connect your wallet
      {/if}
    </DataTable>
  </div>
</div>
