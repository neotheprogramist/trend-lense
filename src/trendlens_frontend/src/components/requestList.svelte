<script lang="ts">
  import { Exchanges } from "$lib/exchange";
  import { keyStore } from "$lib/keystore.svelte";
  import { wallet } from "$lib/wallet.svelte";
  import type { SignableInstruction } from "../../../declarations/trendlens_backend/trendlens_backend.did";
  import Button from "./shad/ui/button/button.svelte";
  import {
    isBalancesRequest,
    isInstrumentsRequest,
    isPostOrderRequest,
  } from "$lib/request";

  import * as Card from "$components/shad/ui/card/index";

  interface IProps {
    requests: SignableInstruction[][];
    onRequestSelect: (id: number) => void;
  }

  let { requests, onRequestSelect }: IProps = $props();

</script>

<div class="grid grid-cols-1 lg:grid-cols-2">
  {#each requests as transaction, id}
    <Card.Root class="m-5">
      <Card.Header>
        <Card.Title>Transaction name</Card.Title>
        <Card.Description>transaction id: {id}</Card.Description>
      </Card.Header>
      <Card.Content class="space-y-6">
        <h2 class="text-lg">Instructions</h2>
        <div>
          {#each transaction as ix}
            {@const exchange = Object.keys(ix.instruction.exchange)[0]}
            {@const requestType = Object.keys(ix.instruction.request)[0]}
            {@const request = ix.instruction.request}

            <fieldset class="space-y-3 rounded-lg border p-5">
              <legend class="-ml-1 px-1 text-sm font-medium"
                >{requestType}</legend
              >
              <span class="flex justify-between"
                >exchange: <span>{exchange}</span></span
              >
              <span class="flex justify-between flex-wrap"
                >api key: <span
                  >{ix.instruction.api_key.substring(0, 12)}...</span
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
        </div>
        <Button
          variant="outline"
          class="ml-auto"
          onclick={() => onRequestSelect(id)}>View</Button
        >
      </Card.Content>
      <Card.Footer></Card.Footer>
    </Card.Root>
  {/each}
</div>
