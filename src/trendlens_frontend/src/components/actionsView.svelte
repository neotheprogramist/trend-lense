<script lang="ts">
  import { onMount } from "svelte";
  import { TradingClient } from "$lib/tradingClient";
  import type { Action } from "$lib/tradingClient";
  import {
    ColorType,
    CrosshairMode,
    type SeriesMarker,
    type Time,
    type SeriesMarkerPosition,
    type SeriesMarkerShape,
    type IChartApi,
    type ISeriesApi,
  } from "lightweight-charts";
  import { mode } from "mode-watcher";
  import { Chart, LineSeries } from "svelte-lightweight-charts";

  let predictedActions: Action[] = [];
  let loading = $state(false);
  let error = $state<string | null>(null);
  let { isVisible }: { isVisible: boolean } = $props();


  onMount(async () => {
    await fetchPredictions();
  });

  async function fetchPredictions() {
    console.log("fetchPredictions");
    try {
      loading = true;
      error = null;
      const client = new TradingClient();

      // Get current date and format it
      const endDate = new Date();
      const startDate = new Date();
      endDate.setDate(endDate.getDate() - 1);
      startDate.setDate(startDate.getDate() - 30);

      const formatDate = (date: Date) => date.toISOString().split("T")[0];

      const response = await client.getActions({
        start: formatDate(startDate),
        end: formatDate(endDate),
        pair: "BTC-USD",
        interval: "1d",
      });

      predictedActions = Array.isArray(response.actions)
        ? response.actions
        : [];
    } catch (err) {
      error =
        err instanceof Error ? err.message : "Failed to fetch predictions";
      console.error("Error fetching predictions:", err);
    } finally {
      loading = false;
    }
  }

  let lineData = $derived(
    predictedActions
      .sort(
        (a, b) =>
          new Date(a.timestamp).getTime() - new Date(b.timestamp).getTime(),
      )
      .map((action) => {
        const time = new Date(action.timestamp).getTime() / 1000;
        return {
          time: time as Time,
          value: action.action === 0 ? 0.5 : action.action === 1 ? 0 : 1,
        };
      }),
  );

  let markers = $derived(
    predictedActions
      .sort(
        (a, b) =>
          new Date(a.timestamp).getTime() - new Date(b.timestamp).getTime(),
      )
      .map((action) => {
        const time = new Date(action.timestamp).getTime() / 1000;
        return {
          time: time as Time,
          position: "inBar" as SeriesMarkerPosition,
          color: getActionColor(action.action),
          shape: (action.action === 0
            ? "circle"
            : "arrowDown") as SeriesMarkerShape,
          text: getActionLabel(action.action),
          size: 2,
        };
      }),
  );

  const getActionColor = (action: number) => {
    switch (action) {
      case 0:
        return "#9ca3af"; // Hold - gray
      case 1:
        return "#22c55e"; // Buy - green
      case 2:
        return "#ef4444"; // Sell - red
      default:
        return "#e5e7eb";
    }
  };

  const getActionLabel = (action: number) => {
    switch (action) {
      case 0:
        return "HOLD";
      case 1:
        return "BUY";
      case 2:
        return "SELL";
      default:
        return "";
    }
  };

  const THEMES = {
    dark: {
      layout: {
        background: {
          type: ColorType.Solid,
          color: "#0c0a09",
        },
        textColor: "#D9D9D9",
      },
      grid: {
        vertLines: { color: "#2B2B43" },
        horzLines: { color: "#363C4E" },
      },
    },
    light: {
      layout: {
        background: {
          type: ColorType.Solid,
          color: "#FFFFFF",
        },
        textColor: "#191919",
      },
      grid: {
        vertLines: { visible: false },
        horzLines: { visible: false },
      },
    },
  };

  let theme = $derived(THEMES[$mode as "dark" | "light"]);

  const options = {
    autoSize: true,
    height: 400,
    layout: {
      background: { type: ColorType.Solid, color: "transparent" },
      fontFamily: "system-ui",
    },
    rightPriceScale: {
      borderVisible: false,
      visible: true,
      scaleMargins: {
        top: 0.1,
        bottom: 0.1,
      },
      minValue: -0.2,
      maxValue: 1.2,
      ticksVisible: false,
    },
    timeScale: {
      borderVisible: false,
      timeVisible: true,
      fixLeftEdge: true,
      fixRightEdge: true,
    },
    grid: {
      vertLines: { visible: false },
      horzLines: { visible: false },
    },
    crosshair: {
      horzLine: { visible: false },
      vertLine: { visible: true },
    },
  };

</script>

{#if loading}
  <div class="flex items-center justify-center">
    <span class="loading">Loading predictions...</span>
  </div>
{:else if error}
  <div class="error-message">
    {error}
    <button onclick={fetchPredictions}>Retry</button>
  </div>
{:else}
  <div class="flex flex-col gap-4">
    <div class="flex items-center gap-4 p-4 text-sm">
      <div class="flex items-center gap-2">
        <div
          class="h-4 w-4 rounded-full"
          style="background-color: {getActionColor(1)}"
        ></div>
        <span>Buy</span>
      </div>
      <div class="flex items-center gap-2">
        <div
          class="h-4 w-4 rounded-full"
          style="background-color: {getActionColor(2)}"
        ></div>
        <span>Sell</span>
      </div>
      <div class="flex items-center gap-2">
        <div
          class="h-4 w-4 rounded-full"
          style="background-color: {getActionColor(0)}"
        ></div>
        <span>Hold</span>
      </div>
    </div>

    <div class="relative h-[400px] min-h-[400px] w-full">
      {#key isVisible}
        <Chart {...options} {...theme.layout}>
          {#if lineData.length > 0}
            <LineSeries
              data={lineData}
              {markers}
              reactive={true}
              lineWidth={3}
              lineType={2}
              priceFormat={{ type: "custom", formatter: () => "" }}
            />
          {/if}
        </Chart>
      {/key}
    </div>
  </div>
{/if}

<style>
  :global(.trading-actions) {
    width: 100%;
    aspect-ratio: 16 / 9;
    min-height: 400px;
  }

  .loading {
    /* Add your loading styles */
  }

  .error-message {
    color: red;
    padding: 1rem;
  }

  .actions-container {
    /* Add your container styles */
  }
</style>
