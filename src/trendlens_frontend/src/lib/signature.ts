import { Buffer } from "buffer";

function base64ToArrayBuffer(base64: string) {
  return Buffer.from(base64, "base64");
}

function arrayBufferToBase64(buffer: ArrayBuffer) {
  return Buffer.from(buffer).toString("base64");
}

// this is not generic right now, but solely based on the requirements of the OKX api
export const finishSignature = async (
  signatureData: string,
  secret: string,
  timestamp: string,
): Promise<string> => {
  console.log(timestamp);
  const preHashString = timestamp + signatureData;
  console.log(preHashString);
  const keyData = base64ToArrayBuffer(btoa(secret));
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
