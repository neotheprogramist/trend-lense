import { handleExchange, Exchanges } from "./exchange";
import { keyStore } from "./keystore.svelte";
import { isBalanceResponse } from "./response";
import { extractOkValue } from "./result";
import { finishSignature } from "./signature";
import { wallet } from "./wallet.svelte";

export const getBalance = async (
  exchange: Exchanges,
  currencies: Array<string> = [],
) => {
  if (!wallet.actor) {
    throw new Error("wallet not connected");
  }

  const key = keyStore.getByExchange(exchange);

  if (!key) {
    throw new Error("missing key");
  }

  let currenciesList: [] | [string[]] = [];
  if (currencies.length > 0) {
    currenciesList = [currencies];
  }

  const requestNumber = await wallet.actor.add_instruction({
    api_key: key.apiKey,
    exchange: handleExchange(exchange),
    request: {
      Balances: {
        currency: currenciesList,
      },
    },
  });

  const requestSignatureData =
    await wallet.actor.get_signatures_metadata(requestNumber);

  console.log(requestSignatureData);

  const timestamp = Math.round(Date.now() / 1000) - 1;
  const isoTimestamp = new Date().toISOString();

  const signature = await finishSignature(
    exchange,
    requestSignatureData[0],
    key.secretKey,
    exchange == Exchanges.Coinbase ? timestamp.toString() : isoTimestamp,
  );

  const result = await wallet.actor.run_transaction(
    requestNumber,
    [signature],
    isoTimestamp,
    BigInt(timestamp),
  );

  try {
    const response = extractOkValue(result)[0];

    if (isBalanceResponse(response)) {
      return response.Balances;
    } else {
      throw new Error("Response returned not type of balances");
    }
  } catch (err) {
    console.error(err);
    return [];
  }
};