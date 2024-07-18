<script lang="ts">
  import { Exchanges } from "$lib/exchange";
  import * as Tabs from "./shad/ui/tabs/index";
  import Toggle from "./shad/ui/toggle/toggle.svelte";
  import Badge from "./shad/ui/badge/badge.svelte";
  import Input from "./shad/ui/input/input.svelte";
  import {
    InstrumentType,
    OrderSideType,
    OrderTypeType,
    TradeModeType,
  } from "$lib/request";
  import {
    handleOrderSide,
    inferTradeModes,
    PostOrderRequest,
  } from "$lib/postOrder.svelte";
  import { pairToString } from "$lib/pair";
  import type { Pair } from "../../../declarations/trendlens_backend/trendlens_backend.did";
  import Slider from "./shad/ui/slider/slider.svelte";
  import { wallet } from "$lib/wallet.svelte";
  import Button from "./shad/ui/button/button.svelte";
  import { getBalance } from "$lib/getBalance";
  import { keyStore } from "$lib/keystore.svelte";
  import { handleApiData } from "$lib/apiAddition";
  import { extractOkValue } from "$lib/result";
    import { finishSignature } from "$lib/signature";

  interface IProps {
    instrument: Pair;
    instrumentType: InstrumentType;
    exchanges: Exchanges[];
  }

  let { exchanges, instrument, instrumentType }: IProps = $props();

  type ExchangesBalances = {
    [key: string]: { base: number; quote: number };
  };

  let volumeRatio = $state([50]);

  let allVolumeRatios = $derived.by(() => {
    const full = 100;

    return [full - volumeRatio[0], volumeRatio[0]];
  });

  let exchangeBalances = $state<ExchangesBalances>({});

  const orderTypes = Object.keys(OrderTypeType).map(
    (e) => e as keyof typeof OrderTypeType,
  );

  const request = new PostOrderRequest(
    inferTradeModes(instrumentType),
    pairToString(instrument),
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
      request.size!,
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

    const requestNumber = await onPost();
    const signatureData =
      await wallet.actor.get_signatures_metadata(requestNumber);
    const timestamp = Math.round(Date.now() / 1000) - 1;
    const isoTimestamp = new Date().toISOString();

    let signatures = []

    for (let i = 0; i < signatureData.length; i++) {
      const exchange = exchanges[i]
      const key = keyStore.getByExchange(exchange);

      if (!key) {
        throw new Error("Key not found");
      }

      const signature = await finishSignature(
        exchanges[i],
        signatureData[i],
        key.secretKey,
        exchange == Exchanges.Coinbase ? timestamp.toString() : isoTimestamp,
      );

      signatures.push(signature)
    }

    const result = await wallet.actor.run_transaction(
      requestNumber,
      signatures,
      isoTimestamp,
      BigInt(timestamp),
    );

    console.log(result)
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

<Tabs.Root bind:value={request.orderType} class="p-2 pb-10 space-y-10">
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
    <Tabs.Content value={request.orderType} class="space-y-5">
      {#if request.orderType == OrderTypeType.Market}
        {#if request.orderPrice.required}
          <Input type="number" placeholder="price" />
        {/if}

        <div class="flex justify-between">
          <span>{exchanges[0]}</span>
          <span>{exchanges[1]}</span>
        </div>

        <Slider bind:value={volumeRatio} max={100} step={1} class="w-full" />
        <Input type="number" placeholder="amount" bind:value={request.size} />
      {:else}
        Unsupported
      {/if}
    </Tabs.Content>
  </div>
  <div class="w-full gap-3 ml-auto inline-flex justify-evenly items-center">
    <div class="flex flex-col gap-2">
      {#if wallet.connected && wallet.actor}
        <Button variant="outline" onclick={onPost}>Post</Button>
      {:else}
        <Button variant="outline" onclick={wallet.connect}
          >Connect wallet</Button
        >
      {/if}

      {#if wallet.connected && wallet.actor}
        <Button onclick={onExecute}>Execute</Button>
      {:else}
        <Button variant="outline" onclick={wallet.connect}
          >Connect wallet</Button
        >
      {/if}
    </div>

    <fieldset class="grid gap-6 rounded-lg border p-4">
      <legend class="-ml-1 px-1 text-sm font-medium"> Balances </legend>
      <div class="flex gap-5">
        <div class="flex flex-col">
          {#each exchanges as e}
            {@const balance = exchangeBalances[e]}
            <div class="flex justify-between">
              <span class="font-bold">{e}</span>
              <span class="ml-5">
                {balance ? balance.base : "-"}
                {instrument.base} / {balance ? balance.quote : "-"}
                {instrument.quote}
              </span>
            </div>
          {/each}
        </div>
      </div>
    </fieldset>
  </div>
</Tabs.Root>
