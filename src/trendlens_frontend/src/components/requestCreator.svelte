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
  import ApiKeyPicker from "./apiKeyPicker.svelte";
  import RequestForm, { type FormFields } from "./requestForm.svelte";

  // right now i pass exchange as prop, but it could be store or context

  interface IProps {
    exchange: Exchanges;
  }

  let { exchange }: IProps = $props();

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

  const handleApiPick = () => {
    pushState("", {
      requestPickState: RequestPickState.ApiPicked,
      request: null,
    });
  };

  $inspect($page.state.requestPickState);
</script>

{#if $page.state.requestPickState == undefined}
  <ApiKeyPicker {exchange} onApiKeyPick={handleApiPick} />
{:else if $page.state.requestPickState === RequestPickState.ApiPicked}
  <RequestPicker onRequestPick={handleRequestPick} />
{:else if $page.state.requestPickState === RequestPickState.RequestPicked}
  <RequestForm {request} />
{/if}
