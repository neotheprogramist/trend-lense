<script lang="ts">
  import { handleExchange, type Exchanges } from "$lib/exchange";
  import {
    Instrument,
    Instruments,
    RequestPickState,
    RequestType,
    type ExchangeRequest,
    type InstrumentsRequest,
  } from "$lib/request";
  import RequestPicker from "./requestPicker.svelte";
  import { page } from "$app/stores";
  import { pushState } from "$app/navigation";
  import RequestForm, { type FormFields } from "./requestForm.svelte";
  import { keyStore } from "$lib/keystore.svelte";
  import { onMount } from "svelte";
  import { wallet } from "$lib/wallet.svelte";
  import type {
    Request as BackendRequest,
    InstrumentType,
  } from "../../../declarations/trendlens_backend/trendlens_backend.did";
  import Page from "../routes/+page.svelte";

  // right now i pass exchange as prop, but it could be store or context

  interface IProps {
    exchange: Exchanges;
  }

  let { exchange }: IProps = $props();
  let exchangeKey = keyStore.getByExchange(exchange);
  let request = $state<FormFields<ExchangeRequest | null>>(null);

  const getDefaultRequestOfType = (r: RequestType): ExchangeRequest => {
    switch (r) {
      case RequestType.Empty:
        throw new Error("empty request not allowed");
      case RequestType.Instruments:
        return {
          type: RequestType.Instruments,
          instrumentId: "",
          instrumentType: new Instruments(Instrument.Spot),
        };
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
    instrument: keyof typeof Instrument,
  ): InstrumentType => {
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
      case RequestType.Instruments:
        const req = r as InstrumentsRequest;
        const key = req.instrumentType.v as keyof typeof Instrument;

        return {
          Instruments: {
            instrument_id: [req.instrumentId],
            instrument_type: handleInstrumentType(key),
          },
        };
    }

    return {
      Empty: null,
    };
  };

  const sendRequest = async () => {
    const requestTransformed = handleRequest(request);
    const key = keyStore.getByExchange(exchange);

    if (!request) {
      throw new Error("No request");
    }

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
