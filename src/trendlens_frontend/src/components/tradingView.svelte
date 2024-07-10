<svelte:options runes={false} />

<script lang="ts">
  import {
    ColorType,
    type SeriesDataItemTypeMap,
  } from "lightweight-charts";
  import { CandlestickSeries, Chart } from "svelte-lightweight-charts";

  export let candlesData: SeriesDataItemTypeMap["Candlestick"][] = [];

  const THEMES = {
    Dark: {
      chart: {
        layout: {
          background: {
            type: ColorType.Solid,
            color: "#2B2B43",
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
    Light: {
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

  const AVAILABLE_THEMES = Object.keys(THEMES);

  let selected = AVAILABLE_THEMES[0];
  $: theme = THEMES[selected as keyof typeof THEMES];

  const options = {
    autoSize: true,
    rightPriceScale: {
      borderVisible: false,
    },
    timeScale: {
      borderVisible: false,
    },
  };
</script>

<div class="flex justify-center">
  <Chart {...options} {...theme.chart} container={{ class: "trading-view" }}>
    <CandlestickSeries
      reactive={true}
      data={candlesData}
      upColor="rgba(255, 144, 0, 1)"
      downColor="#000"
      borderDownColor="rgba(255, 144, 0, 1)"
      borderUpColor="rgba(255, 144, 0, 1)"
      wickDownColor="rgba(255, 144, 0, 1)"
      wickUpColor="rgba(255, 144, 0, 1)"
    ></CandlestickSeries>
  </Chart>
</div>

<style>
  :global(.trading-view) {
    width: 100%;
    aspect-ratio: 16 / 9;
  }
</style>
