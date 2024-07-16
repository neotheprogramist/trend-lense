<script lang="ts">
  import TradingHeader from "$components/tradingHeader.svelte";
  import TradingView from "$components/tradingView.svelte";
  import { anonymousBackend } from "$lib/canisters";
  import type { UTCTimestamp } from "lightweight-charts";
  import type { Candle } from "../../../../../declarations/trendlens_backend/trendlens_backend.did";
  import type { SeriesDataItemTypeMap } from "lightweight-charts";
  import * as Card from "$components/shad/ui/card/index";
  import { Exchanges, handleExchange } from "$lib/exchange";
  import { handlePair, Pairs } from "$lib/pair";
  import RequestCreator from "$components/requestCreator.svelte";
  import RequestList from "$components/requestList.svelte";
  import type { PageData } from "./$types";
  import { onMount } from "svelte";
  import DataTable from "$components/dataTable.svelte";
  import type { InstrumentType } from "$lib/request";
  import { wallet } from "$lib/wallet.svelte";
  import InstrumentsLoader from "$components/instrumentsLoader.svelte";
  import { extractOkValue } from "$lib/result";

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
  let selectedExchange = $state<Exchanges>(data.exchange);

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

  const fetchNewCandles = async (exchange: Exchanges, pair: Pairs) => {
    stopTimestamp = Date.now();

    console.log("Fetching candles from", lastTimestamp, "to", stopTimestamp);

    try {
      const newCandles = extractOkValue(
        await anonymousBackend.pull_candles(
          handlePair(pair),
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
            handlePair(pair),
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

  const fetchCandles = (pair: Pairs): void => {
    candlesFromBackend = [];

    if (interval) {
      clearInterval(interval);
    }

    fetchNewCandles(selectedExchange, pair);
    interval = setInterval(fetchNewCandles, fetchInterval);
  };
  console.log("e");

  $inspect(data);
</script>

<div class="grid gap-2 md:grid-cols-2 lg:grid-cols-7">
  <Card.Root class="col-span-3">
    <Card.Header>
      <Card.Title>Instruments</Card.Title>
    </Card.Header>
    <Card.Content>
      {#if wallet.connected && wallet.actor}
        <InstrumentsLoader
          instrumentType={data.instrumentType}
          onInstrumentsArrived={(exchange, instruments) =>
            console.log(exchange, instruments)}
        />
      {:else}
        wallet not connected
      {/if}

      <!-- <RequestList /> -->
    </Card.Content>
  </Card.Root>

  <Card.Root class="col-span-4">
    <Card.Header>
      <Card.Title>Price chart</Card.Title>
    </Card.Header>
    <Card.Content>
      <TradingHeader
        onSelectionCompleted={fetchCandles}
        bind:selectedExchange
      />
      <TradingView candlesData={candlesFromBackend} />
    </Card.Content>
  </Card.Root>

  <Card.Root class="col-span-7">
    <Card.Header>
      <Card.Title>Actions</Card.Title>
    </Card.Header>
    <Card.Content>
      <!-- {#if selectedExchange}
        <RequestCreator exchange={selectedExchange} />
      {:else}
        Choose exchange and pair to view options
      {/if} -->
    </Card.Content>
  </Card.Root>
</div>
