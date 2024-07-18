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
  import { wallet } from "$lib/wallet.svelte";
  import type { Pair } from "../../../declarations/trendlens_backend/trendlens_backend.did";
  import Badge from "./shad/ui/badge/badge.svelte";
  import { Button } from "./shad/ui/button/index";
  import Input from "./shad/ui/input/input.svelte";
  import * as Tabs from "./shad/ui/tabs/index";
  import Toggle from "./shad/ui/toggle/toggle.svelte";

  interface IProps {
    exchange: Exchanges;
    instrument: Pair;
    instrumentType: InstrumentType;
    onPost: (request: PostOrderRequest) => void;
    onExecute: (request: PostOrderRequest) => void;
  }

  let { instrument, instrumentType, onExecute, onPost, exchange }: IProps =
    $props();

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

    request.changeInstrumentId(pairToString(instrument));
  });

  const request = new PostOrderRequest(
    inferTradeModes(instrumentType),
    pairToString(instrument),
  );

  const orderTypes = Object.keys(OrderTypeType).map(
    (e) => e as keyof typeof OrderTypeType,
  );

  const handleOrderSideChange = () => {
    request.orderSide =
      request.orderSide == OrderSideType.Buy
        ? OrderSideType.Sell
        : OrderSideType.Buy;
  };

  const handleTradeModeChange = (tradeMode: TradeModeType) => {
    request.tradeMode = tradeMode;
  };
</script>

<form>
  <Tabs.Root bind:value={request.orderType} class="space-y-10 p-2">
    <div class="flex justify-between">
      <Tabs.List>
        {#each orderTypes as orderType}
          <Tabs.Trigger value={orderType}>{orderType}</Tabs.Trigger>
        {/each}
      </Tabs.List>

      <div>
        <Toggle
          class="font-bold"
          onPressedChange={handleOrderSideChange}
          pressed={request.orderSide == OrderSideType.Buy}>BUY</Toggle
        >

        <Toggle
          class="font-bold"
          onPressedChange={handleOrderSideChange}
          pressed={request.orderSide == OrderSideType.Sell}>SELL</Toggle
        >
      </div>
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

    <div class="mx-auto mt-4 w-3/4 space-y-5">
      <Tabs.Content value={request.orderType} class="space-y-2">
        {#if request.orderPrice.required}
          <Input type="number" placeholder="price" />
        {/if}

        <Input type="number" placeholder="amount" bind:value={request.size} />
      </Tabs.Content>
    </div>
    <div class="ml-auto inline-flex w-full justify-between gap-3">
      <div>
        {#if wallet.connected && wallet.actor}
          <Button variant="outline" onclick={() => onPost(request)}>Post</Button
          >
          <Button onclick={() => onExecute(request)}>Execute</Button>
        {:else}
          <Button variant="outline" onclick={wallet.connect}
            >Connect wallet</Button
          >
        {/if}
      </div>

      <div class="align-center flex text-lg">
        {#if request.orderSide == OrderSideType.Buy}
          {balances.quote == 0 ? "-" : balances.quote} {instrument.quote}
        {:else}
          {balances.base == 0 ? "-" : balances.base} {instrument.base}
        {/if}
      </div>
    </div>
  </Tabs.Root>
</form>
