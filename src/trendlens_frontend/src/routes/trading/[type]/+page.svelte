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
  import Separator from "$components/shad/ui/separator/separator.svelte";
  import TradeForm from "$components/tradeForm.svelte";
  import { executeRequest, type PostOrderRequest } from "$lib/postOrder.svelte";
  import { getBalance } from "$lib/getBalance";

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

  const availableExchanges = Object.keys(Exchanges).map((e) => e as Exchanges);

  let selectedExchanges = $state<Exchanges[]>([data.exchange]);
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

    fetchNewCandles(selectedExchanges[0], selectedInstrument!);
    interval = setInterval(fetchNewCandles, fetchInterval);
  };

  const handlePost = (request: PostOrderRequest) => {
    throw new Error("Not implemented");
  };

  const handleExecute = async (request: PostOrderRequest) => {
    await executeRequest(selectedExchanges[0], request);
  };

  const fetchBalances = async (instrumentId: string) => {
    const [base, quote] = instrumentId.split("-");

    const fetched = await getBalance(selectedExchanges[0]);

    console.log(fetched)
    const baseBalance = fetched[0].details
      .filter((e) => e.ccy === base)
      .map((e) => e.eq)[0];
    const quoteBalance = fetched[0].details
      .filter((e) => e.ccy === quote)
      .map((e) => e.eq)[0];

    balances = {
      base: baseBalance == undefined ? 0 : Number(baseBalance),
      quote: quoteBalance == undefined ? 0 : Number(quoteBalance),
    };

    console.log(balances);

    console.log("balances", balances);
  };

  const handleInstrumentChange = (i: string) => {
    selectedInstrument = i;
    fetchBalances(i);
    fetchCandles();
  };

  onMount(async () => {
    await instrumentsStore.filterByType(data.instrumentType, false);
  });
</script>

<div class="mt-2 grid md:grid-cols-2 lg:grid-cols-8">
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
      <Tabs.Content value="charts">Change your password here.</Tabs.Content>
      <Tabs.Content value="info">Change your password here.</Tabs.Content>
    </Tabs.Root>

    <Separator orientation="horizontal" />
    {#if selectedInstrument}
      <TradeForm
        {balances}
        exchange={selectedExchanges[0]}
        instrumentId={selectedInstrument}
        instrumentType={data.instrumentType}
        onExecute={handleExecute}
        onPost={handlePost}
      />
    {:else}
      Select instrument to trade
    {/if}
  </div>

  <div class="col-span-2 border"></div>
</div>
