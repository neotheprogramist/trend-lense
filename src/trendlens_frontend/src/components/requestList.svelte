<script lang="ts">
  import { Exchanges } from "$lib/exchange";
  import { keyStore } from "$lib/keystore.svelte";
  import { finishSignature } from "$lib/signature";
  import { wallet } from "$lib/wallet.svelte";
  import type { ExchangeRequestInfo } from "../../../declarations/trendlens_backend/trendlens_backend.did";
  import Button from "./shad/ui/button/button.svelte";

  // TODO: replace it with some real structure
  let requests = $state<([ExchangeRequestInfo] | [])[]>([]);

  const fetchRequests = async () => {
    const fetchedRequests = await wallet.actor?.get_requests();

    console.log(fetchedRequests);
    if (fetchedRequests) {
      requests = fetchedRequests;
    }
  };

  // TODO: replace those numbers with something more meaningful
  const deleteRequest = async (id: number) => {
    await wallet.actor?.delete_request(id);
    requests = requests.filter((_, r_id) => r_id !== id);
  };

  const runRequest = async (id: number) => {
    const signatureData = await wallet.actor?.get_request_signature_string(id);

    // get this exchange
    const key = keyStore.getByExchange(Exchanges.Okx);
    if (!signatureData) {
      throw new Error("No signature found");
      return;
    }
    
    const now = new Date();
    const timestamp = now.toISOString();
   
    const signature = await finishSignature(
      signatureData,
      key?.secretKey,
      timestamp,
    );

    const res = await wallet.actor?.run_request(id, signature, timestamp);
    console.log(res);
  };

  $inspect({ requests });
</script>

<Button onclick={fetchRequests}>Refresh</Button>

<ul>
  {#each requests as request, id}
    <li>
      <span class="p-2"> {JSON.stringify(request)}</span>
      <Button on:click={() => runRequest(id)}>Run</Button>
      <Button on:click={() => deleteRequest(id)}>Delete</Button>
    </li>
  {/each}
</ul>
