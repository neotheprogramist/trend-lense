import { handleExchange, type Exchanges } from "./exchange";
import { keyStore } from "./keystore.svelte";
import { isBalanceResponse } from "./response";
import { extractOkValue } from "./result";
import { finishSignature } from "./signature";
import { wallet } from "./wallet.svelte";

export const getBalance = async (
  exchange: Exchanges,
  currencies: Array<string>,
) => {
  if (!wallet.actor) {
    throw new Error("wallet not connected");
  }

  const key = keyStore.getByExchange(exchange);

  if (!key) {
    throw new Error("missing key");
  }

  const requestNumber = await wallet.actor.add_instruction({
    api_key: key.apiKey,
    exchange: handleExchange(exchange),
    request: {
      Balances: {
        currency: currencies,
      },
    },
  });

  const requestSignatureData =
    await wallet.actor.get_signature_string(requestNumber);

  console.log(requestSignatureData);

  const isoTimestamp = new Date().toISOString();

  const signature = await finishSignature(
    requestSignatureData,
    key.secretKey,
    isoTimestamp,
  );

  const result = await wallet.actor!.run_request(
    requestNumber,
    signature,
    isoTimestamp,
  );

  try {
    const response = extractOkValue(result);

    if (isBalanceResponse(response)) {
      const balances = response.Balances;

      console.log(balances);
    } else {
      throw new Error("Response returned not type of balances");
    }
  } catch (err) {
    console.error(err);
  }
};
