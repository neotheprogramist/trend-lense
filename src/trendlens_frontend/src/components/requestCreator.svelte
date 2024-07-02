<script lang="ts">
  import type { Exchanges } from "$lib/exchange";
  import { RequestPickState, RequestType } from "$lib/request";
  import type {
    Pair,
    Request as CandidRequest,
    GetInstrumentsRequest,
  } from "../../../declarations/trendlens_backend/trendlens_backend.did";
  import RequestPicker from "./requestPicker.svelte";
  import { page } from "$app/stores";
  import { pushState } from "$app/navigation";
  import RequestForm, { type FormFields } from "./requestForm.svelte";
  import { keyStore } from "$lib/keystore.svelte";
  import { onMount } from "svelte";

  // right now i pass exchange as prop, but it could be store or context

  interface IProps {
    exchange: Exchanges;
  }

  let { exchange }: IProps = $props();

  let exchangeKey = keyStore.getByExchange(exchange);
  let request = $state<FormFields<GetInstrumentsRequest | null>>(null);

  const getDefaultRequestOfType = (r: RequestType): GetInstrumentsRequest => {
    switch (r) {
      case RequestType.Empty:
        throw new Error("empty request not allowed");
      case RequestType.Instruments:
        return {
          instId: [""],
          instType: {
            Spot: null,
          },
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
  <RequestForm bind:request />
{/if}
