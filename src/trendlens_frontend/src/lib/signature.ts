import { Buffer } from "buffer";
import { Exchanges } from "./exchange";

function base64ToArrayBuffer(base64: string) {
  return Buffer.from(base64, "base64");
}

function arrayBufferToBase64(buffer: ArrayBuffer) {
  return Buffer.from(buffer).toString("base64");
}

// this is not generic right now, but solely based on the requirements of the OKX api
export const finishSignature = async (
  exchange: Exchanges,
  signatureData: string,
  secret: string,
  timestamp: string,
): Promise<string> => {
  const preHashString = timestamp + signatureData;

  const keyData = base64ToArrayBuffer(
    exchange == Exchanges.Okx ? btoa(secret) : secret,
  );
  const cryptoKey = await crypto.subtle.importKey(
    "raw",
    keyData,
    { name: "HMAC", hash: "SHA-256" },
    false,
    ["sign"],
  );

  const messageBuffer = new TextEncoder().encode(preHashString);
  const signature = await crypto.subtle.sign("HMAC", cryptoKey, messageBuffer);

  return arrayBufferToBase64(signature);
};
