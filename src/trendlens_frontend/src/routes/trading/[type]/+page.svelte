<script lang="ts">
  import TradingHeader from "$components/tradingHeader.svelte";
  import TradingView from "$components/tradingView.svelte";
  import { anonymousBackend } from "$lib/canisters";
  import type { UTCTimestamp } from "lightweight-charts";
  import type {
    Pair,
    Candle,
    SignableInstruction,
    Order,
  } from "../../../../../declarations/trendlens_backend/trendlens_backend.did";
  import type { SeriesDataItemTypeMap } from "lightweight-charts";
  import * as Card from "$components/shad/ui/card/index";
  import { Exchanges, handleExchange } from "$lib/exchange";
  import type { PageData } from "./$types";
  import { onMount } from "svelte";
  import { wallet } from "$lib/wallet.svelte";
  import { extractOkValue } from "$lib/result";
  import InstrumentsSelect from "$components/instrumentsSelect.svelte";
  import { instrumentsStore } from "$lib/instruments.svelte";
  import * as Tabs from "$components/shad/ui/tabs";
  import RequestCreator from "$components/requestCreator.svelte";
  import Separator from "$components/shad/ui/separator/separator.svelte";
  import TradeForm from "$components/tradeForm.svelte";
  import { executeRequest, type PostOrderRequest } from "$lib/postOrder.svelte";
  import { getBalance } from "$lib/getBalance";
  import MultiForm from "$components/multiForm.svelte";
  import RequestList from "$components/requestList.svelte";
  import Button from "$components/shad/ui/button/button.svelte";
  import TransactionPreview from "$components/transactionPreview.svelte";
  import OrdersList from "$components/ordersList.svelte";
  import { keyStore } from "$lib/keystore.svelte";
  import { pairFromString, pairToString } from "$lib/pair";
  import { handleInstrumentType } from "$lib/instrumentType";
  import { finishSignature } from "$lib/signature";
  import { isOrdersResponse } from "$lib/response";
  import VolumeChart from "$components/volumeChart.svelte";

  interface IProps {
    data: PageData;
  }

  let { data }: IProps = $props();

  const ONE_MINUTE = 60 * 1000;
  const ONE_HOUR = 60 * ONE_MINUTE;

  let candlesFromBackend = $state<SeriesDataItemTypeMap["Candlestick"][]>([]);
  let fetchInterval = $state(ONE_MINUTE);
  let interval = $state<number | null>(null);
  let lastTimestamp = $state<number>(Date.now() - ONE_HOUR);
  let stopTimestamp = $state<number>(Date.now());
  let balances = $state<{
    base: number;
    quote: number;
  }>({ base: 0, quote: 0 });

  let selectedExchanges = $state<Exchanges[]>([]);
  let selectedInstrument = $state<Pair | null>(null);

  let availableExchanges = $derived.by(() => {
    if (selectedInstrument) {
      return Object.keys(Exchanges)
        .map((e) => {
          const key = e as Exchanges;
          console.log(key);
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

  const fetchNewCandles = async (exchange: Exchanges, pair: string) => {
    stopTimestamp = Date.now();

    console.log("Fetching candles from", lastTimestamp, "to", stopTimestamp);
    console.log(pair);
    try {
      const newCandles = extractOkValue(
        await anonymousBackend.pull_candles(
          pair,
          handleExchange(exchange),
          BigInt(Math.floor(lastTimestamp / 1000)),
          BigInt(Math.floor(stopTimestamp / 1000)),
        ),
      );

      console.log("new candles", newCandles);

      lastTimestamp =
        Number(
          await anonymousBackend.get_last_timestamp(
            handleExchange(exchange),
            pair,
          ),
        ) *
          1000 +
        1;

      const transformedCandles = transformCandleData(newCandles);
      candlesFromBackend = [...candlesFromBackend, ...transformedCandles];

      // invoke reactivity
      candlesFromBackend = candlesFromBackend;
    } catch (err) {
      console.error("Error fetching candles", err);
    }
  };

  const fetchCandles = (): void => {
    candlesFromBackend = [];

    if (interval) {
      clearInterval(interval);
    }

    // fetchNewCandles(selectedExchanges[0], selectedInstrument!);
    // interval = setInterval(fetchNewCandles, fetchInterval);
  };

  const handlePost = (request: PostOrderRequest) => {
    throw new Error("Not implemented");
  };

  const handleExecute = async (request: PostOrderRequest) => {
    await executeRequest(selectedExchanges[0], request);
  };

  const handleInstrumentChange = (i: Pair) => {
    selectedInstrument = i;
    // fetchBalances(i);
    fetchCandles();
  };

  let requests = $state<SignableInstruction[][]>([]);
  let orders = $state<Order[]>([]);
  let done_orders = $state<Order[]>([]);

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

  const fetchOrders = async (pending: boolean) => {
    if (!wallet.actor) {
      throw new Error("No actor found");
    }

    console.log("fetching orders");

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
            OrdersList: {
              instrument_id: selectedInstrument!,
              instrument_type: handleInstrumentType(data.instrumentType),
              pending,
            },
          },
        };
      }),
    );

    const timestamp = Math.round(Date.now() / 1000) - 1;
    const isoTimestamp = new Date().toISOString();
    let signatures = [];
    let newOrders: Order[] = [];

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
        if (isOrdersResponse(response)) {
          newOrders = [...newOrders, ...response.OrdersInfo];
        } else {
          throw new Error("Response returned not type of order");
        }
      }
    } catch (err) {
      throw new Error("Not type of response" + err);
    }

    orders = newOrders;
  };

  const handleRefreshClick = async () => {
    if (selectedInfoBar === "requests") {
      await fetchRequests();
    } else if (selectedInfoBar === "open_orders") {
      orders = [];
      await fetchOrders(true);
    } else if (selectedInfoBar === "orders_history") {
      orders = [];
      await fetchOrders(false);
    }
  };
</script>

<div class="mt-2 grid md:grid-cols-2 lg:grid-cols-8">
  <div class="col-span-8 border-t border-l border-r p-2">
    <div class="color-primary">
      {#if selectedInstrument}
        {selectedInstrument.base + "/" + selectedInstrument.quote}
      {:else}
        no instrument selected
      {/if}
    </div>
  </div>
  <div class="col-span-2 border p-2">
    <InstrumentsSelect
      instrumentType={data.instrumentType}
      onInstrumentSelect={handleInstrumentChange}
    />
  </div>

  <div class="col-span-4 border-t border-b">
    <div class="p-2">
      <TradingHeader bind:selectedExchanges {availableExchanges} />
    </div>
    <Separator orientation="horizontal" />
    <Tabs.Root value="trading" class="p-2">
      <Tabs.List>
        <Tabs.Trigger value="trading">Trading</Tabs.Trigger>
        <Tabs.Trigger value="charts">Charts</Tabs.Trigger>
        <Tabs.Trigger value="info">Info</Tabs.Trigger>
      </Tabs.List>
      <Tabs.Content value="trading">
        <TradingView candlesData={candlesFromBackend} />
      </Tabs.Content>
      <Tabs.Content value="charts">
        {#if selectedInstrument}
          <VolumeChart
            instrument={selectedInstrument}
            exchanges={selectedExchanges}
          />
        {:else}
          Select instrument to view volume chart
        {/if}
      </Tabs.Content>
      <Tabs.Content value="info">Change your password here.</Tabs.Content>
    </Tabs.Root>

    <Separator orientation="horizontal" />

    {#if selectedExchanges.length == 0}
      Select exchange to trade
    {:else if selectedExchanges.length == 1}
      <TradeForm
        exchange={selectedExchanges[0]}
        instrument={selectedInstrument!}
        instrumentType={data.instrumentType}
        onExecute={handleExecute}
        onPost={handlePost}
      />
    {:else}
      <MultiForm
        exchanges={selectedExchanges}
        instrument={selectedInstrument!}
        instrumentType={data.instrumentType}
      />
    {/if}
  </div>

  <div class="col-span-2 border">
    <TransactionPreview
      transaction={selectedRequest}
      transactionId={selectedRequestIndex}
      onTransactionDelete={(id) => {
        selectedRequest = null;
        selectedRequestIndex = null;
        requests.splice(id, 1);
      }}
    />
  </div>

  <div class="col-span-8 border-b border-l border-r p-2">
    <Tabs.Root bind:value={selectedInfoBar} class="p-2">
      <div class="flex justify-between">
        <Tabs.List>
          <Tabs.Trigger value="requests">Requests</Tabs.Trigger>
          <Tabs.Trigger value="open_orders">Open orders</Tabs.Trigger>
          <Tabs.Trigger value="orders_history">Orders History</Tabs.Trigger>
        </Tabs.List>
        <Button onclick={handleRefreshClick}>Refresh</Button>
      </div>

      <Tabs.Content value="requests">
        <RequestList
          {requests}
          onRequestSelect={(id) => {
            selectedRequestIndex = id;
            selectedRequest = requests[id];
            console.log(selectedRequest);
          }}
        />
      </Tabs.Content>
      <Tabs.Content value="open_orders">
        <OrdersList {orders} />
      </Tabs.Content>
      <Tabs.Content value="orders_history">
        <OrdersList {orders} />
      </Tabs.Content>
    </Tabs.Root>
  </div>
</div>
