<script lang="ts">
  import { handleApiData } from "$lib/apiAddition";
  import { Exchanges } from "$lib/exchange";
  import { getBalance } from "$lib/getBalance";
  import { keyStore } from "$lib/keystore.svelte";
  import { pairToString } from "$lib/pair";
  import {
    handleOrderSide,
    inferTradeModes,
    PostOrderRequest,
  } from "$lib/postOrder.svelte";
  import {
    InstrumentType,
    OrderSideType,
    OrderTypeType,
    TradeModeType,
  } from "$lib/request";
  import { extractOkValue } from "$lib/result";
  import { finishSignature } from "$lib/signature";
  import { wallet } from "$lib/wallet.svelte";
  import { mode } from "mode-watcher";
  import type { Pair } from "../../../declarations/trendlens_backend/trendlens_backend.did";
  import Badge from "./shad/ui/badge/badge.svelte";
  import Button from "./shad/ui/button/button.svelte";
  import Input from "./shad/ui/input/input.svelte";
  import Slider from "./shad/ui/slider/slider.svelte";
  import * as Tabs from "./shad/ui/tabs/index";
  import { cn } from "./utils";

  interface IProps {
    instrument: Pair;
    currentPrice: number;
    instrumentType: InstrumentType;
    exchanges: Exchanges[];
  }

  let { exchanges, instrument, instrumentType, currentPrice }: IProps =
    $props();

  type ExchangesBalances = {
    [key: string]: { base: number; quote: number };
  };

  let volumeRatio = $state([50]);

  let allVolumeRatios = $derived.by(() => {
    return [volumeRatio[0] / 100, (100 - volumeRatio[0]) / 100];
  });

  let exchangeBalances = $state<ExchangesBalances>({});

  const orderTypes = Object.keys(OrderTypeType).map(
    (e) => e as keyof typeof OrderTypeType,
  );

  const request = new PostOrderRequest(
    inferTradeModes(instrumentType),
    pairToString(instrument),
  );

  const handleOrderSideChange = (orderSite: OrderSideType) => {
    if (orderSite == request.orderSide) return;
    request.orderSide = orderSite;
  };

  const handleTradeModeChange = (tradeMode: TradeModeType) => {
    request.tradeMode = tradeMode;
  };

  const onPost = async () => {
    if (!wallet || !wallet.actor) {
      throw new Error("Wallet not connected");
    }

    const keys = exchanges
      .map((e) => keyStore.getByExchange(e))
      .filter((i) => i !== null);

    if (keys.length != exchanges.length) {
      throw new Error("Not all keys are present");
    }

    const response = await wallet.actor.split_transaction(
      keys.map((e) => handleApiData(e)),
      pairToString(instrument),
      handleOrderSide(request.orderSide),
      Number(request.size!),
      20,
      allVolumeRatios,
      1,
    );

    return extractOkValue(response);
  };

  const onExecute = async () => {
    if (!wallet || !wallet.actor) {
      throw new Error("Wallet not connected");
    }

    const [requestNumber, instructions] = await onPost();
    const timestamp = Math.round(Date.now() / 1000) - 1;
    const isoTimestamp = new Date().toISOString();

    let signatures = [];

    for (let i = 0; i < instructions.length; i++) {
      const exchange = exchanges[i];
      const key = keyStore.getByExchange(exchange);

      if (!key) {
        throw new Error("Key not found");
      }

      const signature = await finishSignature(
        exchanges[i],
        instructions[i].signature,
        key.secretKey,
        exchange == Exchanges.Coinbase ? timestamp.toString() : isoTimestamp,
      );

      signatures.push(signature);
    }

    await wallet.actor.run_transaction(
      requestNumber,
      signatures,
      isoTimestamp,
      BigInt(timestamp),
    );
  };

  const getFixedAmount = (amount: number) => {
    if (amount == 0) return "-";
    if (amount < 1) return amount.toFixed(6);
    return amount.toFixed(2);
  };

  //@ts-ignore
  $effect(async () => {
    await Promise.all(
      exchanges.map(async (e) => {
        const balances = await getBalance(e, [
          instrument.base,
          instrument.quote,
        ]);

        const exchangeBalance = { base: 0, quote: 0 };

        for (let i = 0; i < balances.length; i++) {
          const balance = balances[i];

          if (balance.currency == instrument.base) {
            exchangeBalance.base = Number(balance.available);
          } else if (balance.currency == instrument.quote) {
            exchangeBalance.quote = Number(balance.available);
          }
        }

        exchangeBalances[e] = exchangeBalance;
      }),
    );

    request.changeInstrumentId(pairToString(instrument));
  });
</script>

<form class="flex flex-col items-center justify-start h-full">
  <Tabs.Root bind:value={request.orderType} class="space-y-6">
    <div class="w-full mt-6 flex justify-between items-center">
      <span
        class={cn(
          "flex-1 flex rounded-md p-1 text-sm",
          $mode == "dark" ? "bg-[#292524]" : "bg-[#f5f5f4]",
        )}
      >
        <button
          class={cn(
            "rounded-md flex-1 text-center py-1",
            request.orderSide == OrderSideType.Buy
              ? "bg-green-500"
              : "bg-transparent",
          )}
          onclick={() => handleOrderSideChange(OrderSideType.Buy)}>BUY</button
        >
        <button
          class={cn(
            "rounded-md flex-1 text-center py-1",
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

    <div class="w-1/3 text-center space-y-1">
      <p class="text-sm">Market price</p>
      <p class="text- font-bold">{currentPrice.toFixed(2)}</p>
    </div>

    <Tabs.Content value={request.orderType} class="space-y-8">
      {#if request.orderType == OrderTypeType.Market}
        {#if request.orderPriceRequired}
          <Input type="number" placeholder="price" />
        {/if}

        <div class="px-3">
          <div class="flex justify-between">
            <span>{exchanges[0]}</span>
            <span>{exchanges[1]}</span>
          </div>

          <Slider bind:value={volumeRatio} max={100} step={1} class="w-full" />
        </div>

        <div>
          <label
            for="total"
            class={cn(
              "text-xs",
              $mode == "dark" ? "text-neutral-300" : "text-neutral-600",
            )}>Total value</label
          >
          <div id="total" class="flex border rounded-md px-3 py-2.5 outline-1">
            <input
              type="text"
              class="flex-1 outline-none bg-transparent"
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
      {:else}
        <p class="text-center">Unsupported</p>
      {/if}
    </Tabs.Content>

    <fieldset class="grid gap-6 rounded-lg border p-4">
      <div class="flex flex-col">
        {#each exchanges as e}
          {@const balance = exchangeBalances[e]}
          <div class="flex justify-between items-center">
            <div class="font-bold">{e}</div>
            <div class="ml-5 flex-col text-sm">
              <p>
                {getFixedAmount(balance ? balance.base : 0)}
                {instrument.base}
              </p>

              <p>
                {getFixedAmount(balance ? balance.quote : 0)}
                {instrument.quote}
              </p>
            </div>
          </div>
        {/each}
      </div>
    </fieldset>

    <div class="flex flex-col w-full space-y-4">
      <Button onclick={onExecute}>Execute</Button>
      <Button variant="outline" onclick={onPost}>Post</Button>
    </div>
  </Tabs.Root>
</form>
