<script lang="ts">
  import { handleExchange, type Exchanges } from "$lib/exchange";
  import {
    Instruments,
    InstrumentType,
    OrderSide,
    OrderSideType,
    OrderType,
    OrderTypeType,
    RequestPickState,
    RequestType,
    TradeMode,
    TradeModeType,
    type BalanceRequests,
    type ExchangeRequest,
    type InstrumentsRequest,
    type PostOrderRequest,
  } from "$lib/request";
  import RequestPicker from "./requestPicker.svelte";
  import { page } from "$app/stores";
  import { pushState } from "$app/navigation";
  import RequestForm, { type Fields } from "./requestForm.svelte";
  import { keyStore } from "$lib/keystore.svelte";
  import { onMount } from "svelte";
  import { wallet } from "$lib/wallet.svelte";
  import type {
    Request as BackendRequest,
    InstrumentType as BackendInstrumentType,
  } from "../../../declarations/trendlens_backend/trendlens_backend.did";
  import type { Field } from "bits-ui/dist/bits/date-picker";

  // right now i pass exchange as prop, but it could be store or context

  interface IProps {
    exchange: Exchanges;
  }

  let { exchange }: IProps = $props();
  let exchangeKey = keyStore.getByExchange(exchange);
  let request = $state<Fields<ExchangeRequest | null>>(null);

  const getDefaultRequestOfType = (r: RequestType): ExchangeRequest => {
    switch (r) {
      case RequestType.Empty:
        throw new Error("empty request not allowed");
      case RequestType.GetInstruments:
        return {
          type: RequestType.GetInstruments,
          instrumentId: null,
          instrumentType: new Instruments(InstrumentType.Spot),
        } as InstrumentsRequest;
      case RequestType.GetBalance:
        return {
          type: RequestType.GetBalance,
          currencies: [],
        } as BalanceRequests;
      case RequestType.PostOrder:
        return {
          type: RequestType.PostOrder,
          instrumentId: "",
          side: new OrderSide(OrderSideType.Buy),
          marginCurrency: null,
          size: 1,
          orderPrice: null,
          orderType: new OrderType(OrderTypeType.Market),
          positionSide: null,
          tradeMode: new TradeMode(TradeModeType.SpotIsolated),
        } as PostOrderRequest;
    }
  };

  const handleRequestPick = (r: RequestType) => {
    request = getDefaultRequestOfType(r);

    pushState("", {
      requestPickState: RequestPickState.RequestPicked,
      request: r,
    });
  };

  const handleInstrumentType = (
    instrument: keyof typeof InstrumentType,
  ): BackendInstrumentType => {
    switch (instrument) {
      case "Features":
        return {
          Futures: null,
        };
      case "Margin":
        return {
          Margin: null,
        };
      case "Spot":
        return {
          Spot: null,
        };
      case "Swap":
        return {
          Swap: null,
        };
    }
  };

  const handleRequest = (r: ExchangeRequest): BackendRequest => {
    switch (r.type) {
      case RequestType.GetInstruments:
        const req = r as InstrumentsRequest;
        const key = req.instrumentType.v as keyof typeof InstrumentType;

        return {
          Instruments: {
            instrument_id: req.instrumentId ? [req.instrumentId] : [],
            instrument_type: handleInstrumentType(key),
          },
        };
    }

    return {
      Empty: null,
    };
  };

  const sendRequest = async () => {
    if (!request) {
      throw new Error("No request");
    }

    const requestTransformed = handleRequest(request);
    const key = keyStore.getByExchange(exchange);

    if (!key) {
      throw new Error("No key for exchange");
    }

    const number = await wallet.actor?.initialize_request({
      request: requestTransformed,
      api_key: key.apiKey,
      exchange: handleExchange(exchange),
    });
  };

  onMount(() => {
    if (exchangeKey) {
      pushState("", {
        requestPickState: RequestPickState.ApiRegistered,
        request: null,
      });
    }

    keyStore.load();
  });
</script>

{#if $page.state.requestPickState == undefined}
  {#if !exchangeKey}
    Your api key was not setup, please set it up in appropriate place
  {/if}
{:else if $page.state.requestPickState === RequestPickState.ApiRegistered}
  <RequestPicker onRequestPick={handleRequestPick} />
{:else if $page.state.requestPickState === RequestPickState.RequestPicked}
  <RequestForm bind:request onSubmit={sendRequest} />
{/if}
