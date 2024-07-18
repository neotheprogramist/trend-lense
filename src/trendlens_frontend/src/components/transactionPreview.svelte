<script lang="ts">
  import * as Card from "$components/shad/ui/card/index";
  import { Exchanges, toExchanges } from "$lib/exchange";
  import { keyStore } from "$lib/keystore.svelte";
  import {
    isBalancesRequest,
    isInstrumentsRequest,
    isPostOrderRequest,
  } from "$lib/request";
  import { finishSignature } from "$lib/signature";
  import { wallet } from "$lib/wallet.svelte";
  import type { SignableInstruction } from "../../../declarations/trendlens_backend/trendlens_backend.did";
  import Button from "./shad/ui/button/button.svelte";

  interface IProps {
    transactionId: number | null;
    transaction: SignableInstruction[] | null;
    onTransactionDelete: (id: number) => void;
  }

  let { transaction, onTransactionDelete, transactionId }: IProps = $props();

  const deleteRequest = async (id: number) => {
    if (!wallet.actor) {
      throw new Error("No actor found");
    }

    await wallet.actor.delete_transaction(id);

    onTransactionDelete(id);
  };

  const runRequest = async (id: number) => {
    if (!wallet.actor) {
      throw new Error("No actor found");
    }

    if (!transaction || !transactionId) {
      throw new Error("No transaction found");
    }

    const timestamp = Math.round(Date.now() / 1000) - 1;
    const isoTimestamp = new Date().toISOString();

    let signatures = [];

    for (let i = 0; i < transaction.length; i++) {
      const exchange = toExchanges(transaction[i].instruction.exchange);
      const key = keyStore.getByExchange(exchange);

      if (!key) {
        throw new Error("Key not found");
      }

      const signature = await finishSignature(
        exchange,
        transaction[i].signature,
        key.secretKey,
        exchange == Exchanges.Coinbase ? timestamp.toString() : isoTimestamp,
      );

      signatures.push(signature);
    }

    const result = await wallet.actor.run_transaction(
      transactionId,
      signatures,
      isoTimestamp,
      BigInt(timestamp),
    );

    console.log(result);
  };

  $inspect(transactionId);
</script>

{#if transaction && transactionId}
  <Card.Root class="m-5">
    <Card.Content class="space-y-6">
      <h2 class="text-lg">Instructions</h2>
      <div>
        {#each transaction as ix}
          {@const exchange = Object.keys(ix.instruction.exchange)[0]}
          {@const requestType = Object.keys(ix.instruction.request)[0]}
          {@const request = ix.instruction.request}

          <fieldset class="space-y-3 rounded-lg border p-5">
            <legend class="-ml-1 px-1 text-sm font-medium">{requestType}</legend
            >
            <span class="flex justify-between"
              >exchange: <span>{exchange}</span></span
            >
            <span class="flex justify-between flex-wrap"
              >api key: <span>{ix.instruction.api_key.substring(0, 12)}...</span
              ></span
            >

            {#if isBalancesRequest(request)}
              <span class="flex justify-between"
                >currency : <span>{request.Balances.currency}</span></span
              >
            {:else if isInstrumentsRequest(request)}
              <span class="flex justify-between"
                >instrument: <span>{request.Instruments.instrument_id}</span
                ></span
              >
              <span class="flex justify-between"
                >instrument type: <span
                  >{request.Instruments.instrument_type}</span
                ></span
              >
            {:else if isPostOrderRequest(request)}
              <span class="flex justify-between"
                >price: <span>{request.PostOrder.order_price}</span></span
              >
              <span class="flex justify-between"
                >size: <span>{request.PostOrder.size}</span></span
              >
            {/if}

            <span class="flex justify-between"
              >executed: <span>{ix.executed}</span></span
            >
          </fieldset>
        {/each}

        <Button
          variant="outline"
          class="ml-auto mt-5"
          onclick={() => deleteRequest(transactionId)}>Delete</Button
        >
        <Button
          variant="outline"
          class="ml-auto mt-5"
          onclick={() => runRequest(transactionId)}>Execute</Button
        >
      </div>
    </Card.Content>
    <Card.Footer></Card.Footer>
  </Card.Root>
{:else}
  <p>No transaction found</p>
{/if}
