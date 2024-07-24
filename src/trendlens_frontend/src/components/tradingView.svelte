<svelte:options runes={false} />

<script lang="ts">
  import { ColorType, type SeriesDataItemTypeMap } from "lightweight-charts";
  import { mode } from "mode-watcher";
  import { CandlestickSeries, Chart } from "svelte-lightweight-charts";

  export let candlesData: SeriesDataItemTypeMap["Candlestick"][] = [];

  const THEMES = {
    dark: {
      chart: {
        layout: {
          background: {
            type: ColorType.Solid,
            color: "#0c0a09",
          },
          lineColor: "#2B2B43",
          textColor: "#D9D9D9",
        },
        watermark: {
          color: "rgba(0, 0, 0, 0)",
        },
        crosshair: {
          color: "#758696",
        },
        grid: {
          vertLines: {
            color: "#2B2B43",
          },
          horzLines: {
            color: "#363C4E",
          },
        },
      },
      series: {
        topColor: "rgba(32, 226, 47, 0.56)",
        bottomColor: "rgba(32, 226, 47, 0.04)",
        lineColor: "rgba(32, 226, 47, 1)",
      },
    },
    light: {
      chart: {
        layout: {
          background: {
            type: ColorType.Solid,
            color: "#FFFFFF",
          },
          lineColor: "#2B2B43",
          textColor: "#191919",
        },
        watermark: {
          color: "rgba(0, 0, 0, 0)",
        },
        grid: {
          vertLines: {
            visible: false,
          },
          horzLines: {
            color: "#f0f3fa",
          },
        },
      },
      series: {
        topColor: "rgba(33, 150, 243, 0.56)",
        bottomColor: "rgba(33, 150, 243, 0.04)",
        lineColor: "rgba(33, 150, 243, 1)",
      },
    },
  };

  $: theme = THEMES[$mode as "dark" | "light"];

  const options = {
    autoSize: true,
    rightPriceScale: {
      borderVisible: false,
    },
    timeScale: {
      borderVisible: false,
      timeVisible: true,
    },
  };
</script>

<div class="flex justify-center">
  <Chart {...options} {...theme.chart} container={{ class: "trading-view" }}>
    <CandlestickSeries reactive={true} data={candlesData}></CandlestickSeries>
  </Chart>
</div>

<style>
  :global(.trading-view) {
    width: 100%;
    aspect-ratio: 16 / 9;
  }
</style>
