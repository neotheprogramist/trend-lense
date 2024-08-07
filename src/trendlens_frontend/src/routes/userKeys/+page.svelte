<script lang="ts">
  import ApiKeyDialog from "$components/apiKeyDialog.svelte";
  import DataTable from "$components/dataTable.svelte";
  import {
    ApiRegisterStatus,
    handleApiData,
    type ApiRegisterStatusType,
  } from "$lib/apiAddition";
  import { keyStore, type ApiWithSecret } from "$lib/keystore.svelte";
  import { wallet } from "$lib/wallet.svelte";
  import { onMount } from "svelte";

  const saveApiData = async (data: ApiWithSecret) => {
    if (!wallet.connected || !wallet.actor) {
      console.error("Wallet not connected");
      return;
    }

    keyStore.add(data);
  };

  const removeApiKey = async (apiKey: string) => {
    if (!wallet.connected || !wallet.actor) {
      return;
    }

    try {
      keyStore.keys = keyStore.keys.filter((e) => e.apiKey !== apiKey);
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
    const done = await wallet.actor.register_api_key(handleApiData(data));

    if (done) {
      saveApiData(data);

      return ApiRegisterStatus.Registered;
    } else {
      return ApiRegisterStatus.InvalidApiData;
    }
  };

  onMount(() => {
    keyStore.load();
  });
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
