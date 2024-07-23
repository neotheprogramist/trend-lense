<script lang="ts">
  import InstrumentsSelect from "$components/instrumentsSelect.svelte";
  import MultiForm from "$components/multiForm.svelte";
  import OrdersList from "$components/ordersList.svelte";
  import RequestList from "$components/requestList.svelte";
  import Button from "$components/shad/ui/button/button.svelte";
  import * as Tabs from "$components/shad/ui/tabs";
  import TradeForm from "$components/tradeForm.svelte";
  import TradingHeader from "$components/tradingHeader.svelte";
  import TradingView from "$components/tradingView.svelte";
  import { anonymousBackend } from "$lib/canisters";
  import { Exchanges, handleExchange } from "$lib/exchange";
  import { instrumentsStore } from "$lib/instruments.svelte";
  import { handleInstrumentType } from "$lib/instrumentType";
  import { keyStore } from "$lib/keystore.svelte";
  import {
    executeRequest,
    postRequest,
    type PostOrderRequest,
  } from "$lib/postOrder.svelte";
  import { isPendingOrdersResponse } from "$lib/response";
  import { extractOkValue } from "$lib/result";
  import { finishSignature } from "$lib/signature";
  import { wallet } from "$lib/wallet.svelte";
  import type { SeriesDataItemTypeMap, UTCTimestamp } from "lightweight-charts";
  import { RefreshCcw } from "lucide-svelte";
  import { onMount } from "svelte";
  import type {
    Candle,
    GlobalPendingOrder,
    Pair,
    SignableInstruction,
  } from "../../../../../declarations/trendlens_backend/trendlens_backend.did";
  import type { PageData } from "./$types";

  interface IProps {
    data: PageData;
  }

  let { data }: IProps = $props();

  const ONE_MINUTE = 60 * 1000;
  const ONE_HOUR = 60 * ONE_MINUTE;

  let candlesFromBackend = $state<SeriesDataItemTypeMap["Candlestick"][]>([]);
  let fetchInterval = $state(5000);
  let lastTimestamp = $state<number>(Date.now() - ONE_HOUR * 12);
  let balances = $state<{
    base: number;
    quote: number;
  }>({ base: 0, quote: 0 });

  let selectedExchanges = $state<Exchanges[]>([Exchanges.Okx]);
  let selectedInstrument = $state<Pair | null>({ base: "BTC", quote: "EUR" });
  let currentPrice = $state<number>(0);

  let availableExchanges = $derived.by(() => {
    if (selectedInstrument) {
      return Object.keys(Exchanges)
        .map((e) => {
          const key = e as Exchanges;
          return instrumentsStore.hasExchangeInstrument(
            key,
            data.instrumentType,
            selectedInstrument!,
          )
            ? key
            : null;
        })
        .filter((e) => e !== null) as Exchanges[];
    }

    return [];
  });

  const transformCandleData = (
    candles: Candle[],
  ): SeriesDataItemTypeMap["Candlestick"][] => {
    return candles
      .map((candle) => {
        return {
          close: candle.close_price,
          high: candle.highest_price,
          low: candle.lowest_price,
          open: candle.open_price,
          time: Number(candle.timestamp) as UTCTimestamp,
        };
      })
      .sort((a, b) => a.time - b.time);
  };

  const updateCandles = async () => {
    if (selectedInstrument) {
      await fetchCandles(selectedExchanges[0], "BTC-USD");
    }
  };

  const fetchCandles = async (
    exchange: Exchanges,
    pair: string,
  ): Promise<void> => {
    const stopTimestamp = Date.now();

    try {
      const newCandles = extractOkValue(
        await anonymousBackend.pull_candles(
          pair,
          handleExchange(exchange),
          BigInt(Math.floor(lastTimestamp / 1000)),
          BigInt(Math.floor(stopTimestamp / 1000)),
        ),
      );

      currentPrice = newCandles[newCandles.length - 1].close_price;
      const transformedCandles = transformCandleData(newCandles);
      candlesFromBackend = transformedCandles;
    } catch (err) {
      console.error("Error fetching candles", err);
    }
  };

  const handlePost = async (request: PostOrderRequest) => {
    await postRequest(selectedExchanges[0], request);
  };

  const handleExecute = async (request: PostOrderRequest) => {
    await executeRequest(selectedExchanges[0], request);
  };

  const handleInstrumentChange = (i: Pair) => {
    selectedInstrument = i;
  };

  let requests = $state<SignableInstruction[][]>([]);
  let orders = $state<GlobalPendingOrder[]>([]);
  let selectedRequest = $state<SignableInstruction[] | null>(null);
  let selectedRequestIndex = $state<number | null>(null);
  let selectedInfoBar = $state<string | undefined>("requests");

  const fetchRequests = async () => {
    if (!wallet.actor) {
      throw new Error("No actor found");
    }

    const response = await wallet.actor.get_transactions();

    if (response.length > 0 && response[0]) {
      requests = response[0];
    }
  };

  const fetchOrders = async () => {
    if (!wallet.actor) {
      throw new Error("No actor found");
    }

    const [requestNumber, instructions] = await wallet.actor.add_transaction(
      selectedExchanges.map((e) => {
        const key = keyStore.getByExchange(e);

        if (!key) {
          throw new Error("No key found");
        }

        return {
          api_key: key.apiKey,
          exchange: handleExchange(e),
          request: {
            PendingOrders: {
              instrument_id: selectedInstrument!,
              instrument_type: handleInstrumentType(data.instrumentType),
            },
          },
        };
      }),
    );

    const timestamp = Math.round(Date.now() / 1000) - 10;
    const isoTimestamp = new Date().toISOString();
    let signatures = [];
    let newOrders: GlobalPendingOrder[] = [];

    for (let i = 0; i < selectedExchanges.length; i++) {
      const exchange = selectedExchanges[i];
      const key = keyStore.getByExchange(exchange);

      if (!key) {
        throw new Error("No key found");
      }

      const signature = await finishSignature(
        exchange,
        instructions[i].signature,
        key.secretKey,
        exchange == Exchanges.Coinbase ? timestamp.toString() : isoTimestamp,
      );

      signatures.push(signature);
    }

    const result = await wallet.actor.run_transaction(
      requestNumber,
      signatures,
      isoTimestamp,
      BigInt(timestamp),
    );

    try {
      const responses = extractOkValue(result);

      for (let i = 0; i < responses.length; i++) {
        const response = responses[i];
        if (isPendingOrdersResponse(response)) {
          newOrders = [...newOrders, ...response.PendingOrders];
        } else {
          throw new Error("Response returned not type of order");
        }
      }
    } catch (err) {
      throw new Error("Not type of response" + err);
    }

    orders = newOrders;
  };

  const handleRefreshClick = () => {
    if (selectedInfoBar === "requests") {
      fetchRequests();
    } else if (selectedInfoBar === "open_orders") {
      fetchOrders();
    }
  };

  onMount(() => {
    updateCandles();
  });
</script>

<div class="grid md:grid-cols-2 lg:grid-cols-10">
  <div class="col-span-10 border-l border-r py-5 px-7">
    <div class="color-primary">
      {#if selectedInstrument}
        {selectedInstrument.base + "/" + selectedInstrument.quote}
      {:else}
        No instrument selected
      {/if}
    </div>
  </div>

  <div class="col-span-2 border p-2 h-[650px]">
    <InstrumentsSelect
      instrumentType={data.instrumentType}
      onInstrumentSelect={handleInstrumentChange}
    />
  </div>

  <div class="col-span-6 border-t border-b h-[650px]">
    <Tabs.Root value="trading">
      <div class="flex justify-between items-center border-b p-1.5">
        <Tabs.List>
          <Tabs.Trigger value="trading">Trading</Tabs.Trigger>
          <Tabs.Trigger value="charts">Charts</Tabs.Trigger>
          <Tabs.Trigger value="info">Info</Tabs.Trigger>
        </Tabs.List>

        <TradingHeader bind:selectedExchanges {availableExchanges} />
      </div>

      <Tabs.Content value="trading" class="h-full">
        <TradingView candlesData={candlesFromBackend} />
      </Tabs.Content>
      <Tabs.Content value="charts">Charts</Tabs.Content>
      <Tabs.Content value="info">Info</Tabs.Content>
    </Tabs.Root>
  </div>

  <div
    class="col-span-2 border flex justify-center items-center relative h-[650px]"
  >
    {#if !wallet.actor}
      <div
        class="absolute top-0 left-0 w-full h-full backdrop-blur-sm z-10 flex justify-center items-center"
      >
        <Button variant="default" onclick={wallet.connect}
          >Connect wallet</Button
        >
      </div>
    {/if}

    {#if selectedExchanges.length == 0}
      <p>Select exchange to trade</p>
    {:else if selectedExchanges.length == 1}
      <TradeForm
        {currentPrice}
        exchange={selectedExchanges[0]}
        instrument={selectedInstrument!}
        instrumentType={data.instrumentType}
        onExecute={handleExecute}
        onPost={handlePost}
      />
    {:else}
      <MultiForm
        {currentPrice}
        exchanges={selectedExchanges}
        instrument={selectedInstrument!}
        instrumentType={data.instrumentType}
      />
    {/if}
  </div>

  <div class="col-span-10 border-b border-l border-r p-2 h-96">
    <Tabs.Root bind:value={selectedInfoBar} class="p-2">
      <div class="flex justify-between items-center">
        <Tabs.List>
          <Tabs.Trigger value="open_orders">Open Orders</Tabs.Trigger>
          <Tabs.Trigger value="requests">Requests</Tabs.Trigger>
          <Tabs.Trigger value="orders_history">Orders History</Tabs.Trigger>
        </Tabs.List>

        <RefreshCcw
          onclick={handleRefreshClick}
          class="w-5 cursor-pointer hover:-rotate-45 ease-in-out duration-300 hover:stroke-orange-600"
        />
      </div>

      <Tabs.Content value="open_orders">
        <OrdersList {orders} />
      </Tabs.Content>

      <Tabs.Content value="requests">
        <RequestList {requests} />
      </Tabs.Content>

      <Tabs.Content value="orders_history"></Tabs.Content>
    </Tabs.Root>
  </div>
</div>
