<script lang="ts">
  import type { Exchanges } from "$lib/exchange";
  import { getBalance } from "$lib/getBalance";
  import { pairToString } from "$lib/pair";
  import { inferTradeModes, PostOrderRequest } from "$lib/postOrder.svelte";
  import {
    InstrumentType,
    OrderSideType,
    OrderTypeType,
    TradeModeType,
  } from "$lib/request";
  import { mode } from "mode-watcher";
  import { untrack } from "svelte";
  import type { Pair } from "../../../declarations/trendlens_backend/trendlens_backend.did";
  import Badge from "./shad/ui/badge/badge.svelte";
  import { Button } from "./shad/ui/button/index";
  import * as Tabs from "./shad/ui/tabs/index";
  import { cn } from "./utils";

  interface IProps {
    exchange: Exchanges;
    instrument: Pair;
    instrumentType: InstrumentType;
    currentPrice: number;
    onPost: (request: PostOrderRequest) => void;
    onExecute: (request: PostOrderRequest) => void;
  }

  let {
    instrument,
    instrumentType,
    onExecute,
    onPost,
    exchange,
    currentPrice,
  }: IProps = $props();

  let balances = $state<{ base: number; quote: number }>({
    base: 0,
    quote: 0,
  });

  //@ts-ignore
  $effect(async () => {
    const fetchedBalances = await getBalance(exchange, [
      instrument.base,
      instrument.quote,
    ]);

    for (let i = 0; i < fetchedBalances.length; i++) {
      const balance = fetchedBalances[i];

      if (balance.currency == instrument.base) {
        balances.base = Number(balance.available);
      } else if (balance.currency == instrument.quote) {
        balances.quote = Number(balance.available);
      }
    }

    untrack(() => {
      request.changeInstrumentId(pairToString(instrument));
    });
  });

  const request = new PostOrderRequest(
    inferTradeModes(instrumentType),
    pairToString(instrument),
  );

  const orderTypes = Object.keys(OrderTypeType).map(
    (e) => e as keyof typeof OrderTypeType,
  );

  const handleOrderSideChange = (orderSite: OrderSideType) => {
    if (orderSite == request.orderSide) return;
    request.orderSide = orderSite;
  };

  const handleTradeModeChange = (tradeMode: TradeModeType) => {
    request.tradeMode = tradeMode;
  };

  const getFixedAmount = (amount: number) => {
    if (amount == 0) return "-";
    if (amount < 1) return amount.toFixed(6);
    return amount.toFixed(2);
  };
</script>

<form class="flex h-full flex-col items-center justify-start">
  <Tabs.Root bind:value={request.orderType} class="space-y-10">
    <div class="mt-16 flex w-full items-center justify-between">
      <span
        class={cn(
          "flex flex-1 rounded-md p-1 text-sm",
          $mode == "dark" ? "bg-[#292524]" : "bg-[#f5f5f4]",
        )}
      >
        <button
          class={cn(
            "flex-1 rounded-md py-1 text-center",
            request.orderSide == OrderSideType.Buy
              ? "bg-green-500"
              : "bg-transparent",
          )}
          onclick={() => handleOrderSideChange(OrderSideType.Buy)}>BUY</button
        >
        <button
          class={cn(
            "flex-1 rounded-md py-1 text-center",
            request.orderSide == OrderSideType.Sell
              ? "bg-red-500"
              : "bg-transparent",
          )}
          onclick={() => handleOrderSideChange(OrderSideType.Sell)}>SELL</button
        >
      </span>
    </div>

    <div class="flex justify-between">
      <Tabs.List>
        {#each orderTypes as orderType}
          <Tabs.Trigger value={orderType}
            >{orderType == "PostOnly" ? "Post" : orderType}</Tabs.Trigger
          >
        {/each}
      </Tabs.List>
    </div>

    <div class="space-x-2">
      {#if request.tradeModes.length > 1}
        {#each request.tradeModes as tradeMode}
          <Badge
            variant={!(request.tradeMode == tradeMode) ? "outline" : undefined}
            onclick={() => handleTradeModeChange(tradeMode)}>{tradeMode}</Badge
          >
        {/each}
      {/if}
    </div>

    <div class="w-1/3 space-y-1 text-center">
      <p class="text-sm">Market price</p>
      <p class="text- font-bold">{currentPrice.toFixed(2)}</p>
    </div>

    <div class="mx-auto space-y-5">
      <Tabs.Content value={request.orderType} class="space-y-2">
        {#if request.orderPriceRequired}
          <div>
            <label
              for="total"
              class={cn(
                "text-xs",
                $mode == "dark" ? "text-neutral-300" : "text-neutral-600",
              )}>Price target</label
            >
            <div
              id="total"
              class="flex rounded-md border px-3 py-2.5 outline-1"
            >
              <input
                type="text"
                class="flex-1 bg-transparent outline-none"
                placeholder="0.0"
                bind:value={request.orderPrice}
              />
            </div>
          </div>
        {/if}

        <div>
          <div class="flex justify-between">
            <label
              for="total"
              class={cn(
                "text-xs",
                $mode == "dark" ? "text-neutral-300" : "text-neutral-600",
              )}>Total value</label
            >
            {#if request.orderSide == OrderSideType.Buy}
              <button
                class="mr-2 text-xs"
                onclick={() => (request.size = balances.quote.toString())}
              >
                Max: {getFixedAmount(balances.quote)}
              </button>
            {:else}
              <button
                class="mr-2 text-xs"
                onclick={() => (request.size = balances.base.toString())}
              >
                Max: {getFixedAmount(balances.base)}
              </button>
            {/if}
          </div>

          <div id="total" class="flex rounded-md border px-3 py-2.5 outline-1">
            <input
              type="text"
              class="flex-1 bg-transparent outline-none"
              placeholder="0.0"
              bind:value={request.size}
            />
            <span
              class={cn(
                "mx-2",
                $mode == "dark" ? "text-neutral-300" : "text-neutral-600",
              )}
              >{request.orderSide == OrderSideType.Buy
                ? instrument.quote
                : instrument.base}</span
            >
          </div>
        </div>
      </Tabs.Content>
    </div>

    <div class="flex w-full flex-col space-y-4">
      <Button onclick={() => onExecute(request)}>Execute</Button>
      <Button variant="outline" onclick={() => onPost(request)}>Post</Button>
    </div>

    <!-- <div class="flex align-center text-lg">
      {#if request.orderSide == OrderSideType.Buy}
        {balances.quote == 0 ? "-" : balances.quote} {instrument.quote}
      {:else}
        {balances.base == 0 ? "-" : balances.base} {instrument.base}
      {/if}
    </div> -->
  </Tabs.Root>
</form>
