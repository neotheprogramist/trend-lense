<script lang="ts">
  import { Exchanges, handleExchange } from "$lib/exchange";
  import { handleInstrumentType } from "$lib/instrumentType";
  import { keyStore } from "$lib/keystore.svelte";
  import type { InstrumentType } from "$lib/request";
  import { isInstrumentsResponse } from "$lib/response";
  import { extractOkValue } from "$lib/result";
  import { finishSignature } from "$lib/signature";
  import { wallet } from "$lib/wallet.svelte";
  // import { Toaster } from "$components/shad/ui/sonner/index";
  // import { toast } from "svelte-sonner";
  import type {
    Instrument,
    Pair,
  } from "../../../declarations/trendlens_backend/trendlens_backend.did";
  import { onMount } from "svelte";
  import BindableSelect from "./bindableSelect.svelte";

  interface IProps {
    instrumentType: InstrumentType;
    onInstrumentsArrived: (
      exchange: Exchanges,
      instruments: Instrument[],
    ) => void;
  }

  let { instrumentType, onInstrumentsArrived }: IProps = $props();
  const userExchanges = keyStore.exchanges();
  let instrumentsNames = $state<Pair[]>([]);
  let currentInstrument = $state<Pair>();

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

    const requestNumber = await wallet.actor.add_instruction({
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
      await wallet.actor.get_signature_string(requestNumber);

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

    try {
      const response = extractOkValue(result);

      if (isInstrumentsResponse(response)) {
        return response.Instruments;
      } else {
        throw new Error("Response returned not type of instruments");
      }
    } catch (err) {
      console.error(err);
      return [];
    }
  };

  const handleInstruments = (
    exchange: Exchanges,
    instruments: Instrument[],
  ) => {
    console.log(`${exchange} fetch success`);

    instrumentsNames = [
      ...instrumentsNames,
      ...instruments.map((i) => i.instrument_id),
    ];

    onInstrumentsArrived(exchange, instruments);
  };

  const fetchAllInstruments = async () => {
    const promises = userExchanges.map(async (e) => {
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
    await fetchAllInstruments();
  });

  $inspect(instrumentsNames);
</script>

<BindableSelect
  bind:value={currentInstrument}
  items={instrumentsNames}
  placeholder="instruments"
  onChange={(v) => console.log(v)}
/>
