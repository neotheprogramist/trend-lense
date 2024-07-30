<script lang="ts">
  import { Exchanges, toExchanges } from "$lib/exchange";
  import { keyStore } from "$lib/keystore.svelte";
  import { isPostOrderRequest } from "$lib/request";
  import { finishSignature } from "$lib/signature";
  import { wallet } from "$lib/wallet.svelte";
  import { Send, X } from "lucide-svelte";
  import type { SignableInstruction } from "../../../declarations/trendlens_backend/trendlens_backend.did";
  import { request } from "http";
  import { extractOkValue } from "$lib/result";
  import { toast } from "svelte-sonner";
  import { exec } from "child_process";

  interface IProps {
    requests: [number, SignableInstruction[]][];
  }

  let { requests = $bindable() }: IProps = $props();

  const deleteRequest = async (id: number) => {
    if (!wallet.actor) {
      throw new Error("No actor found");
    }

    const executeToast = toast.info("Deleting request", {
      duration: 10000,
    });

    await wallet.actor.delete_transaction(id);

    toast.dismiss(executeToast)

    requests = requests.filter(([elId, _]) => elId != id);
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

    const executeToast = toast.loading("Execution status", {
      description: "executing: " + signatures.length + " instructions",
      duration: 10000,
    });

    const result = await wallet.actor.run_transaction(
      transactionId,
      signatures,
      isoTimestamp,
      BigInt(timestamp),
    );

    toast.dismiss(executeToast);

    try {
      const unwrapped = extractOkValue(result);

      if (unwrapped.length > 0) {
        toast.info("Execution status", {
          description:
            "executed: " + unwrapped.length + "/" + signatures.length,
        });
      } else {
        toast.error("Execution failed", {
          description: "at least one of instructions failed",
        });
      }
    } catch (err) {
      toast.error("Execution failed", {
        description: "unknown",
      });
    }
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

<div class="mt-5 flex px-6">
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
        {#if transaction.length == 1}
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

            <tr class="text-center">
              <td>
                {#if side == "Buy"}
                  <p class="rounded-xl bg-green-900 py-0.5 text-green-400">
                    Buy
                  </p>
                {:else}
                  <p class="rounded-xl bg-red-900 py-0.5 text-red-400">Sell</p>
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
                <div class="flex items-center space-x-4">
                  <Send
                    onclick={() => runRequest(id, transaction)}
                    class="w-5 cursor-pointer stroke-blue-400"
                  />
                  <X
                    onclick={() => deleteRequest(id)}
                    class="w-5 cursor-pointer stroke-red-400"
                  />
                </div>
              </td>
            </tr>
          {/if}
        {:else if transaction.length == 2}
          {#if isPostOrderRequest(transaction[0].instruction.request) && isPostOrderRequest(transaction[1].instruction.request)}
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

            {@const exchange2 = Object.keys(
              transaction[1].instruction.exchange,
            )[0]}
            {@const size2 = transaction[1].instruction.request.PostOrder.size}

            <tr class="text-center">
              <td>
                {#if side == "Buy"}
                  <p class="rounded-xl bg-green-900 py-0.5 text-green-400">
                    Buy
                  </p>
                {:else}
                  <p class="rounded-xl bg-red-900 py-0.5 text-red-400">Sell</p>
                {/if}
              </td>
              <td class="py-3">{instrument.base}/{instrument.quote}</td>
              <td>{exchange} / {exchange2}</td>
              <td>{transaction[0].executed} / {transaction[1].executed}</td>
              <td>{orderType}</td>
              <td>{orderPrice[0] ? orderPrice[0] : "-"}</td>
              <td>{size.toFixed(4)} / {size2.toFixed(4)}</td>
              <td>{tradeMode}</td>
              <td class="w-16">
                <div class="flex items-center space-x-4">
                  <Send
                    onclick={() => runRequest(id, transaction)}
                    class="w-5 cursor-pointer stroke-blue-400"
                  />
                  <X
                    onclick={() => deleteRequest(id)}
                    class="w-5 cursor-pointer stroke-red-400"
                  />
                </div>
              </td>
            </tr>
          {/if}
        {/if}
      {/each}
    </tbody>
  </table>
</div>
