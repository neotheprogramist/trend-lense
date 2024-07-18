<script lang="ts">
  import { pushState } from "$app/navigation";
  import { page } from "$app/stores";
  import { handleExchange, type Exchanges } from "$lib/exchange";
  import { handleInstrumentType } from "$lib/instrumentType";
  import { keyStore } from "$lib/keystore.svelte";
  import { pairFromString } from "$lib/pair";
  import {
    Instruments,
    InstrumentType,
    OptionalField,
    OrderSide,
    OrderSideType,
    OrderType,
    OrderTypeType,
    PositionSide,
    PositionSideType,
    RequestPickState,
    RequestType,
    TradeMode,
    TradeModeType,
    type BalanceRequests,
    type ExchangeRequest,
    type InstrumentsRequest,
    type PostOrderRequest,
  } from "$lib/request";
  import { wallet } from "$lib/wallet.svelte";
  import { onMount } from "svelte";
  import type { Request as BackendRequest } from "../../../declarations/trendlens_backend/trendlens_backend.did";
  import RequestForm, { type Fields } from "./requestForm.svelte";
  import RequestPicker from "./requestPicker.svelte";
  import Button from "./shad/ui/button/button.svelte";

  // right now i pass exchange as prop, but it could be store or context

  interface IProps {
    exchange: Exchanges;
    instrumentId: string;
  }

  let { exchange, instrumentId }: IProps = $props();
  let exchangeKey = keyStore.getByExchange(exchange);
  let request = $state<Fields<ExchangeRequest | null>>(null);

  const getDefaultRequestOfType = (r: RequestType): ExchangeRequest => {
    switch (r) {
      case RequestType.Empty:
        throw new Error("empty request not allowed");
      case RequestType.GetInstruments:
        return {
          type: RequestType.GetInstruments,
          instrumentId: new OptionalField<string>(null),
          instrumentType: new Instruments(InstrumentType.Spot),
        } as InstrumentsRequest;
      case RequestType.GetBalance:
        return {
          type: RequestType.GetBalance,
          currencies: new OptionalField<string>(null),
        } as BalanceRequests;
      case RequestType.PostOrder:
        return {
          type: RequestType.PostOrder,
          instrumentId,
          side: new OrderSide(OrderSideType.Buy),
          marginCurrency: new OptionalField<string>(null),
          size: 1,
          orderPrice: new OptionalField<number>(null),
          orderType: new OrderType(OrderTypeType.Market),
          positionSide: new OptionalField<PositionSide>(
            new PositionSide(PositionSideType.Long),
          ),
          tradeMode: new TradeMode(TradeModeType.SpotIsolated),
        } as PostOrderRequest;
    }
  };

  const handleRequestPick = (r: RequestType) => {
    console.log("picked", request);
    request = getDefaultRequestOfType(r);

    pushState("", {
      requestPickState: RequestPickState.RequestPicked,
      requestType: r,
    });
  };

  const handleRequest = (r: ExchangeRequest): BackendRequest => {
    switch (r.type) {
      case RequestType.GetInstruments:
        let req = r as InstrumentsRequest;

        return {
          Instruments: {
            instrument_id: req.instrumentId.value
              ? [pairFromString(req.instrumentId.value)]
              : [],
            instrument_type: handleInstrumentType(req.instrumentType.v),
          },
        };
      case RequestType.Empty:
        throw new Error("unsupported");
      case RequestType.GetBalance:
        let b_req = r as BalanceRequests;

        return {
          Balances: {
            currency: [[b_req.currencies.value ?? ""]],
          },
        };

      case RequestType.PostOrder:
    }

    return {
      Empty: null,
    };
  };

  const sendRequest = async () => {
    console.log(request);
    if (!request) {
      throw new Error("No request");
    }

    const requestTransformed = handleRequest(request);
    const key = keyStore.getByExchange(exchange);

    if (!key) {
      throw new Error("No key for exchange");
    }

    const number = await wallet.actor?.add_instruction({
      request: requestTransformed,
      api_key: key.apiKey,
      exchange: handleExchange(exchange),
    });
    console.log();
  };

  onMount(() => {
    if (exchangeKey) {
      pushState("", {
        requestPickState: RequestPickState.ApiRegistered,
        requestType: null,
      });
    }

    keyStore.load();
  });

  $inspect({ request }, "REQ");
</script>

{#if $page.state.requestPickState == undefined}
  {#if !exchangeKey}
    Your api key was not setup, please set it up in appropriate place
  {/if}
{:else if $page.state.requestPickState === RequestPickState.ApiRegistered}
  <RequestPicker onRequestPick={handleRequestPick} />
{:else if $page.state.requestPickState === RequestPickState.RequestPicked}
  <RequestForm bind:request />
  <Button class="mt-4" on:click={sendRequest}>Submit</Button>
{/if}
