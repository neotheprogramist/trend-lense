<script lang="ts">
  import { Exchanges, toExchanges } from "$lib/exchange";
  import { keyStore } from "$lib/keystore.svelte";
  import { isPostOrderRequest } from "$lib/request";
  import { finishSignature } from "$lib/signature";
  import { wallet } from "$lib/wallet.svelte";
  import { Send, X } from "lucide-svelte";
  import type { SignableInstruction } from "../../../declarations/trendlens_backend/trendlens_backend.did";

  interface IProps {
    requests: [number, SignableInstruction[]][];
  }

  let { requests }: IProps = $props();

  const deleteRequest = async (id: number) => {
    if (!wallet.actor) {
      throw new Error("No actor found");
    }

    try {
      await wallet.actor.delete_transaction(id);
    } catch (e) {
      console.error(e);
    }
  };

  const runRequest = async (
    transactionId: number,
    transaction: SignableInstruction[],
  ) => {
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

    await wallet.actor.run_transaction(
      transactionId,
      signatures,
      isoTimestamp,
      BigInt(timestamp),
    );
  };

  function filterRequests(requests: [number, SignableInstruction[]][]) {
    return requests.filter(([id, transaction]) => {
      if (
        isPostOrderRequest(transaction[0].instruction.request) &&
        !transaction[0].executed
      )
        return true;

      return false;
    });
  }
</script>

<div class="flex px-6 mt-5">
  <table class="w-full">
    <thead>
      <tr>
        <th class="py-4"></th>
        <th>Instrument</th>
        <th>Exchange</th>
        <th>Executed</th>
        <th>Order Types</th>
        <th>Order Price</th>
        <th>Size</th>
        <th>Trade Mode</th>
      </tr>
    </thead>
    <tbody>
      {#each filterRequests(requests) as [id, transaction]}
        {#if isPostOrderRequest(transaction[0].instruction.request)}
          {@const side = Object.keys(
            transaction[0].instruction.request.PostOrder.side,
          )[0]}
          {@const instrument =
            transaction[0].instruction.request.PostOrder.instrument_id}
          {@const exchange = Object.keys(
            transaction[0].instruction.exchange,
          )[0]}
          {@const orderType = Object.keys(
            transaction[0].instruction.request.PostOrder.order_type,
          )[0]}
          {@const tradeMode = Object.keys(
            transaction[0].instruction.request.PostOrder.trade_mode,
          )[0]}
          {@const orderPrice =
            transaction[0].instruction.request.PostOrder.order_price}
          {@const size = transaction[0].instruction.request.PostOrder.size}

          <!-- {#if transaction.length == 1} -->
            <tr class="text-center">
              <td>
                {#if side == "Buy"}
                  <p class="text-green-400 bg-green-900 rounded-xl py-0.5">
                    Buy
                  </p>
                {:else}
                  <p class="text-red-400 bg-red-900 rounded-xl py-0.5">Sell</p>
                {/if}
              </td>
              <td class="py-3">{instrument.base}/{instrument.quote}</td>
              <td>{exchange}</td>
              <td>{transaction[0].executed}</td>
              <td>{orderType}</td>
              <td>{orderPrice[0] ? orderPrice : "-"}</td>
              <td>{size}</td>
              <td>{tradeMode}</td>
              <td class="w-16">
                <div class="flex space-x-4 items-center">
                  <Send
                    onclick={() => runRequest(id, transaction)}
                    class="cursor-pointer w-5 stroke-blue-400"
                  />
                  <X
                    onclick={() => deleteRequest(id)}
                    class="cursor-pointer w-5 stroke-red-400"
                  />
                </div>
              </td>
            </tr>
          {/if}
        <!-- {/if} -->
      {/each}
    </tbody>
  </table>
</div>
