<script lang="ts">
  import { TradingClient } from "$lib/tradingClient";
  import type { Action } from "$lib/tradingClient";
  import {
    ColorType,
    type Time,
    type SeriesMarkerPosition,
    type SeriesMarkerShape,
  } from "lightweight-charts";
  import { mode } from "mode-watcher";
  import { Chart, LineSeries } from "svelte-lightweight-charts";
  import * as Select from "$components/shad/ui/select";
  import { Button } from "$components/shad/ui/button";
  import { CalendarDays } from "lucide-svelte";
  import { Calendar } from "lucide-svelte";
  import { type DateRange } from "bits-ui";
  import {
    DateFormatter,
    type DateValue,
    getLocalTimeZone,
    today,
  } from "@internationalized/date";
  import { cn } from "./utils";
  import { RangeCalendar } from "$components/shad/ui/range-calendar";
  import * as Popover from "$components/shad/ui/popover";

  let predictedActions = $state<Action[]>([]);
  let loading = $state(false);
  let error = $state<string | null>(null);
  let { isVisible }: { isVisible: boolean } = $props();

  type RangeOption = { label: string; value: string };

  const ranges: RangeOption[] = [
    { label: "Last 7 days", value: "7d" },
    { label: "Last 14 days", value: "14d" },
    { label: "Last 30 days", value: "30d" },
    { label: "Custom Range", value: "custom" },
  ];

  let selectedRange = $state<string>("30d");

  const df = new DateFormatter("en-US", { dateStyle: "medium" });

  let dateRange = $state<DateRange>({
    start: today(getLocalTimeZone()).subtract({ days: 30 }),
    end: today(getLocalTimeZone()).subtract({ days: 15 }),
  });

  $inspect(dateRange);

  let startValue = $state<DateValue | undefined>(undefined);

  function getDateRange(range: string): { start: Date; end: Date } {
    if (range === "custom" && dateRange?.start && dateRange?.end) {
      return {
        start: dateRange.start.toDate(getLocalTimeZone()),
        end: dateRange.end.toDate(getLocalTimeZone()),
      };
    }

    const end = new Date();
    end.setDate(end.getDate());
    const start = new Date();

    switch (range) {
      case "7d":
        start.setDate(start.getDate() - 7);
        break;
      case "14d":
        start.setDate(start.getDate() - 14);
        break;
      case "30d":
        start.setDate(start.getDate() - 30);
        break;
      default:
        throw new Error(`Invalid range: ${range}`);
    }

    return { start, end };
  }

  async function handleRangeChange(value: string) {
    selectedRange = value;

    if (selectedRange !== "custom") {
      await fetchPredictions();
    }
  }

  async function fetchPredictions() {
    console.log("fetchPredictions");
    try {
      loading = true;
      error = null;
      const client = new TradingClient();

      const { start, end } = getDateRange(selectedRange);
    
      const formatDate = (date: Date) => date.toISOString().split("T")[0];

      console.log("start", formatDate(start));
      console.log("end", formatDate(end));

      const response = await client.getActions({
        start: formatDate(start),
        end: formatDate(end),
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
    [...predictedActions]
      .sort(
        (a, b) =>
          new Date(a.timestamp).getTime() - new Date(b.timestamp).getTime(),
      )
      .map((action) => {
        const date = new Date(action.timestamp);
        date.setUTCHours(date.getUTCHours() + 1);
        return {
          time: (date.getTime() / 1000) as Time,
          value: action.action === 0 ? 0.5 : action.action === 1 ? 0 : 1,
        };
      }),
  );

  let markers = $derived(
    [...predictedActions]
      .sort(
        (a, b) =>
          new Date(a.timestamp).getTime() - new Date(b.timestamp).getTime(),
      )
      .map((action) => {
        const date = new Date(action.timestamp);
        date.setUTCHours(date.getUTCHours() + 1);
        return {
          time: (date.getTime() / 1000) as Time,
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

  const maxDate = today(getLocalTimeZone());
  const minDate = maxDate.subtract({ days: 30 });

  $inspect(markers, predictedActions, lineData);
</script>

<!-- Add this before the chart -->
<div class="flex items-center justify-between p-4">
  <div class="flex items-center gap-4">
    <div class="flex items-center gap-2">
      <CalendarDays class="h-4 w-4" />
      <Select.Root
        selected={selectedRange}
        onSelectedChange={(v) => {
          v && handleRangeChange(v.value as string);
        }}
        options={ranges}
      >
        <Select.Trigger class="w-[180px]">
          <Select.Value placeholder="Select range" />
        </Select.Trigger>
        <Select.Content>
          <Select.Group>
            {#each ranges as range}
              <Select.Item value={range.value}>{range.label}</Select.Item>
            {/each}
          </Select.Group>
        </Select.Content>
      </Select.Root>
    </div>

    {#if selectedRange === "custom"}
      <div class="grid gap-2">
        <Popover.Root openFocus>
          <Popover.Trigger asChild let:builder>
            <Button
              variant="outline"
              class={cn(
                "w-[300px] justify-start text-left font-normal",
                !dateRange && "text-muted-foreground",
              )}
              builders={[builder]}
            >
              <Calendar class="mr-2 h-4 w-4" />
              {#if dateRange && dateRange.start}
                {#if dateRange.end}
                  {df.format(dateRange.start.toDate(getLocalTimeZone()))} - {df.format(
                    dateRange.end.toDate(getLocalTimeZone()),
                  )}
                {:else}
                  {df.format(dateRange.start.toDate(getLocalTimeZone()))}
                {/if}
              {:else if startValue}
                {df.format(startValue.toDate(getLocalTimeZone()))}
              {:else}
                Pick a date
              {/if}
            </Button>
          </Popover.Trigger>
          <Popover.Content class="w-auto p-0" align="start">
            <RangeCalendar
              bind:value={dateRange}
              bind:startValue
              placeholder={dateRange?.start}
              initialFocus
              numberOfMonths={2}
              maxValue={maxDate}
              minValue={minDate}
            />
          </Popover.Content>
        </Popover.Root>
      </div>
    {/if}
  </div>

  <Button
    variant="outline"
    size="sm"
    onclick={fetchPredictions}
    disabled={loading}
  >
    Refresh
  </Button>
</div>

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
          {:else}
            <!-- <div class="flex items-center justify-center">
              <span class="text-muted-foreground">Choose a range</span>
            </div> -->
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

  .error-message {
    color: red;
    padding: 1rem;
  }
</style>
