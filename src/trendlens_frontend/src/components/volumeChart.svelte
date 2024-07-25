<script lang="ts">
  import { anonymousBackend } from "$lib/canisters";
  import { Exchanges, handleExchange } from "$lib/exchange";
  import { pairToString } from "$lib/pair";
  import { area, curveLinear, range, scaleLinear, scaleUtc } from "d3";
  import { untrack } from "svelte";
  import type { Pair } from "../../../declarations/trendlens_backend/trendlens_backend.did";

  interface IProps {
    exchanges: Exchanges[];
    instrument: Pair;
  }

  let { exchanges, instrument }: IProps = $props();

  const marginTop = 19; // the top margin, in pixels
  const marginRight = -5; // the right margin, in pixels
  const marginBottom = 30; // the bottom margin, in pixels
  const marginLeft = 50; // the left margin, in pixels
  const inset = 0; // inset the default range, in pixels
  const width = 700; // the outer width of the chart, in pixels
  const height = 500; // the outer height of the chart, in pixels
  const xLabel = ""; // a label for the y-axis
  const yLabel = "↑ Volume (in base coin)"; // a label for the y-axis
  const xFormat = ""; // a format specifier string for the y-axis
  const yFormat = $derived(instrument.base); // a format specifier string for the y-axis
  const horizontalGrid = true; // show horizontal grid lines
  const verticalGrid = false; // show vertical grid lines
  const colors = ["red", "blue", "green", "yellow"]; // fill color for dots && number of colors in fill array MUST match number of subsets in data
  const r = 3; // (fixed) radius of dots, in pixels
  const strokeWidth = 1; // stroke width of line, in pixels
  const fillOpacity = 0.3; // fill opacity of area
  const strokeLinecap = "round"; // stroke line cap of the line
  const strokeLinejoin = "round"; // stroke line join of the line
  const xScalefactor = width / 80; //y-axis number of values
  const yScalefactor = height / 40; //y-axis number of values
  const curve = curveLinear; // method of interpolation between points
  const xType = scaleUtc; // type of x-scale
  const insetTop = inset; // inset from top
  const insetRight = inset; // inset from right
  const insetBottom = inset; // inset fro bottom
  const insetLeft = inset; // inset from left
  const xRange = [marginLeft + insetLeft, width - marginRight - insetRight]; // [left, right]
  const yType = scaleLinear; // type of y-scale
  const yRange = [height - marginBottom - insetBottom, marginTop + insetTop]; // [bottom, top]

  let filteredI = $state<[number, number][]>([]);
  let areas = $state<(string | null)[]>([]);
  let xVals = $state<number[]>([]);
  let yVals = $state<number[]>([]);
  let points = $state<{ x: number; y: number; color: number }[]>([]);
  let subsets = $state<number[]>([]);
  let colorVals = $state<number[]>([]);

  const getVolumes = async (exchange: Exchanges, pair: Pair, id: number) => {
    if (!anonymousBackend) {
      throw new Error("Wallet not initialized");
    }

    const timestamp_secs = Math.floor(Date.now() / 1000);
    const volumes = await anonymousBackend.get_volumes(
      handleExchange(exchange),
      pairToString(pair),
      BigInt(timestamp_secs - 60 * 60 * 24),
    );

    if (volumes.length === 0) {
      return [];
    }

    const volumesUnwrapped = volumes[0];

    for (let i = 0; i < volumesUnwrapped.length; i++) {
      xVals.push(Number(volumesUnwrapped[i].timestamp));
      yVals.push(volumesUnwrapped[i].volume);
      colorVals.push(id);
      points.push({
        x: Number(volumesUnwrapped[i].timestamp),
        y: volumesUnwrapped[i].volume,
        color: id,
      });
    }
    subsets.push(id);
  };

  const I = $derived(range(xVals.length));
  const gaps = (d: { x: number; y: number; color: number }, i: number) =>
    !isNaN(xVals[i]) && !isNaN(yVals[i]);
  const cleanData: boolean[] = $derived(points.map(gaps));

  const xDomain = $derived([xVals[0], xVals[xVals.length - 1]]);
  const yDomain = $derived([0, Math.max(...yVals)]);
  const xScale = $derived(xType(xDomain, xRange));
  const yScale = $derived(yType(yDomain, yRange));
  const niceY = $derived(
    scaleLinear()
      .domain([0, Math.max(...yVals)])
      .nice(),
  );
  const chartArea = $derived(
    area()
      .defined((i) => cleanData[i])
      .curve(curve)
      .x((i) => xScale(xVals[i]))
      .y0(yScale(0))
      .y1((i) => yScale(yVals[i])),
  );

  $effect(() => {
    untrack(() => {
      areas = [];
    });

    colors.forEach((color, j) => {
      filteredI = [];
      //@ts-ignore
      filteredI = I.filter((el, i) => colorVals[i] === j);

      untrack(() => {
        areas.push(chartArea(filteredI));
      });
    });
  });

  const xTicks = $derived(xScale.ticks(xScalefactor));
  const xTicksFormatted = $derived(xTicks.map((el) => el.toLocaleTimeString()));
  const yTicks = $derived(niceY.ticks(yScalefactor));

  // @ts-ignore
  $effect(async () => {
    xVals = [];
    yVals = [];
    points = [];
    subsets = [];
    colorVals = [];

    for (let i = 0; i < exchanges.length; i++) {
      await getVolumes(exchanges[i], instrument, i);
    }
  });

  $inspect({ xVals, yVals });
</script>

<div class="flex h-full">
  <div class="flex flex-[4_4_0%] items-center justify-center">
    <svg {width} {height} viewBox="0 0 {width} {height}" cursor="crosshair">
      {#each areas as subsetArea, i}
        <g class="chartlines" pointer-events="none">
          <path
            class="line"
            fill={colors[i]}
            stroke={colors[i]}
            d={subsetArea}
            fill-opacity={fillOpacity}
            stroke-width={strokeWidth}
            stroke-linecap={strokeLinecap}
            stroke-linejoin={strokeLinejoin}
          />
        </g>
      {/each}

      <g
        class="y-axis"
        transform="translate({marginLeft}, 0)"
        pointer-events="none"
      >
        <path
          class="domain"
          stroke="black"
          d="M{insetLeft}, {marginTop} V{height - marginBottom + 6}"
        />
        {#each yTicks as tick, i}
          <g class="tick" transform="translate(0, {yScale(tick)})">
            <line class="tick-start" x1={insetLeft - 6} x2={insetLeft} />
            {#if horizontalGrid}
              <line
                class="tick-grid"
                x1={insetLeft}
                x2={width - marginLeft - marginRight}
              />
            {/if}
            <text class="text-sm" x="-{marginLeft}" y="5">{tick}</text>
          </g>
        {/each}
        <text class="text-sm" x="-{marginLeft}" y={marginTop - 10}
          >{yLabel}</text
        >
      </g>
      <g
        class="x-axis"
        transform="translate(0,{height - marginBottom - insetBottom})"
        pointer-events="none"
      >
        <path
          class="domain"
          stroke="black"
          d="M{marginLeft},0.5 H{width - marginRight}"
        />
        {#each xTicks as tick, i}
          <g class="tick" transform="translate({xScale(tick)}, 0)">
            <line class="tick-start" stroke="black" y2="6" />
            {#if verticalGrid}
              <line class="tick-grid" y2={-height} />
            {/if}
            <text font-size="8px" x={-marginLeft / 4} y="20"
              >{xTicksFormatted[i] + xFormat}</text
            >
          </g>
        {/each}
        <text x={width - marginLeft - marginRight - 40} y={marginBottom}
          >{xLabel}</text
        >
      </g>
    </svg>
  </div>
  <div class="flex flex-1 flex-col justify-center space-y-5">
    {#each exchanges as e, index}
      <div class="flex items-center">
        <div style="background-color: {colors[index]}" class="h-4 w-4"></div>
        <span class="ml-2">{e}</span>
      </div>
    {/each}
  </div>
</div>

<style>
  svg {
    max-width: 100%;
    height: auto;
    height: "intrinsic";
    margin: auto;
  }
  path {
    fill: "green";
  }
  .y-axis {
    font-size: "10px";
    font-family: sans-serif;
    text-anchor: "end";
  }
  .x-axis {
    font-size: "10px";
    font-family: sans-serif;
    text-anchor: "end";
  }
  .tick {
    opacity: 1;
  }
  .tick-start {
    stroke: white;
    stroke-opacity: 1;
  }
  .tick-grid {
    stroke: white;
    stroke-opacity: 0.2;
    font-size: "11px";
  }
  .tick text {
    fill: white;
    text-anchor: start;
  }
</style>
