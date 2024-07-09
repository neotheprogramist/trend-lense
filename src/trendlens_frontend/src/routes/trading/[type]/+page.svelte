<script lang="ts">
  import TradingHeader from "$components/tradingHeader.svelte";
  import TradingView from "$components/tradingView.svelte";
  import { anonymousBackend } from "$lib/canisters";
  import type { UTCTimestamp } from "lightweight-charts";
  import type {
    Pair,
    Candle,
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
  let selectedInstrument = $state<string | null>(null);

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

    fetchNewCandles(selectedExchange, selectedInstrument!);
    interval = setInterval(fetchNewCandles, fetchInterval);
  };

  onMount(async () => {
    await instrumentsStore.filterByType(data.instrumentType);
  });
</script>

<div class="grid gap-2 md:grid-cols-2 lg:grid-cols-7">
  <Card.Root class="col-span-3">
    <Card.Header>
      <Card.Title>Instruments</Card.Title>
    </Card.Header>
    <Card.Content>
      <Tabs.Root
        value="open"
        onValueChange={(v) => {
          console.log(v);
        }}
      >
        <div class="grid grid-cols-5">
          <div class="col-span-2">
            <Tabs.List>
              <Tabs.Trigger value="open">Open</Tabs.Trigger>
              <Tabs.Trigger value="account">Account</Tabs.Trigger>
            </Tabs.List>
          </div>
          <div></div>
          <div class="col-span-2">
            <InstrumentsSelect
              instrumentType={data.instrumentType}
              onInstrumentSelect={(s) => {
                selectedInstrument = s;
                fetchCandles();
              }}
            />
          </div>
        </div>
        <Tabs.Content value="open">list of open commands</Tabs.Content>
        <Tabs.Content value="account">
          {#if wallet.connected && wallet.actor}
            {#if selectedInstrument}
              <RequestCreator
                exchange={selectedExchange}
                instrumentId={selectedInstrument}
              />
            {:else}
              select instrument
            {/if}
          {:else}
            connect the wallet
          {/if}
        </Tabs.Content>
      </Tabs.Root>

      <!-- <RequestList /> -->
    </Card.Content>
  </Card.Root>

  <Card.Root class="col-span-4">
    <Card.Header>
      <Card.Title>Price chart</Card.Title>
    </Card.Header>
    <Card.Content>
      <TradingHeader bind:selectedExchange />
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
