<script lang="ts">
  import { Exchanges, handleExchange } from "$lib/exchange";
  import { handleInstrumentType } from "$lib/instrumentType";
  import { keyStore } from "$lib/keystore.svelte";
  import type { InstrumentType } from "$lib/request";
  import { isInstrumentsResponse } from "$lib/response";
  import { isOk } from "$lib/result";
  import { finishSignature } from "$lib/signature";
  import { wallet } from "$lib/wallet.svelte";
  // import { Toaster } from "$components/shad/ui/sonner/index";
  // import { toast } from "svelte-sonner";
  import type { Instrument } from "../../../declarations/trendlens_backend/trendlens_backend.did";
  import { onMount } from "svelte";

  interface IProps {
    instrumentType: InstrumentType;
    onInstrumentsArrived: (
      exchange: Exchanges,
      instruments: Instrument[],
    ) => void;
  }

  let { instrumentType, onInstrumentsArrived }: IProps = $props();
  const userExchanges = keyStore.exchanges();

  const loadInstruments = async (
    instrumentType: InstrumentType,
    exchange: Exchanges,
  ) => {
    if (!wallet.actor) {
      throw new Error("wallet actor not defined");
    }

    const key = keyStore.getByExchange(exchange);

    if (!key) {
      throw new Error("api key not exist in local storage");
    }

    const requestNumber = await wallet.actor.initialize_request({
      api_key: key.apiKey,
      exchange: handleExchange(exchange),
      request: {
        Instruments: {
          instrument_id: [], // we want all instruments,
          instrument_type: handleInstrumentType(instrumentType),
        },
      },
    });

    const requestSignatureData =
      await wallet.actor.get_request_signature_string(requestNumber);

    const isoTimestamp = new Date().toISOString();

    const signature = await finishSignature(
      requestSignatureData,
      key.secretKey,
      isoTimestamp,
    );

    const result = await wallet.actor.run_request(
      requestNumber,
      signature,
      isoTimestamp,
    );

    if (isOk(result)) {
      const response = result.Ok;

      if (isInstrumentsResponse(response)) {
        return response.Instruments;
      } else {
        throw new Error("Response returned not type of instruments");
      }
    } else {
      throw new Error("Instruments request failed with:" + result.Err);
    }
  };

  const handleInstruments = (
    exchange: Exchanges,
    instruments: Instrument[],
  ) => {
    console.log(`${exchange} fetch success`);
    // toast.success(`${exchange} fetch success`);
    onInstrumentsArrived(exchange, instruments);
  };

  const fetchAllInstruments = async () => {
    console.log("running requests");
    const promises = userExchanges.map(async (e) => {
      console.log("running one");
      const instruments = await loadInstruments(instrumentType, e);

      handleInstruments(e, instruments);
      return {
        exchange: e,
        instruments,
      };
    });

    return await Promise.all(promises);
  };

  onMount(async () => {
    console.log("rrrr");
    await fetchAllInstruments();
  });
</script>

<!-- <Toaster /> -->
