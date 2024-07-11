<script lang="ts">
  import { Button } from "./shad/ui/button/index";
  import {
    InstrumentType,
    OrderSideType,
    OrderTypeType,
    TradeModeType,
  } from "$lib/request";
  import * as Tabs from "./shad/ui/tabs/index";
  import Toggle from "./shad/ui/toggle/toggle.svelte";
  import Badge from "./shad/ui/badge/badge.svelte";
  import Input from "./shad/ui/input/input.svelte";

  import { inferTradeModes, PostOrderRequest } from "$lib/postOrder.svelte";
  import type { Exchanges } from "$lib/exchange";

  interface IProps {
    exchange: Exchanges;
    instrumentId: string;
    instrumentType: InstrumentType;
    onPost: (request: PostOrderRequest) => void;
    onExecute: (request: PostOrderRequest) => void;
  }

  let { instrumentId, instrumentType, onExecute, onPost, exchange }: IProps =
    $props();

  const request = new PostOrderRequest(
    inferTradeModes(instrumentType),
    instrumentId,
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
  <Tabs.Root bind:value={request.orderType} class="p-2 space-y-2">
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

    <div class="mt-4 w-3/4 mx-auto space-y-5">
      <Tabs.Content value={request.orderType} class="space-y-2">
        {#if request.orderPrice.required}
          <Input type="number" placeholder="price" />
        {/if}
        <Input type="number" placeholder="amount" bind:value={request.size} />
      </Tabs.Content>
      <div class="flex justify-between w-3/4 mx-auto">
        <Button onclick={() => onPost(request)}>Post</Button>
        <Button onclick={() => onExecute(request)}>Post & Execute</Button>
      </div>
    </div>
  </Tabs.Root>
</form>
