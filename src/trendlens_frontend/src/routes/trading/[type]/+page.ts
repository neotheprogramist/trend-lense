import { Exchanges } from "$lib/exchange";
import { InstrumentType } from "$lib/request";
import type { PageLoad } from "./$types";

const capitalize = (el: string) => {
  return el.charAt(0).toUpperCase() + el.slice(1);
};

export const load: PageLoad = async ({ params }) => {
  // default is spot
  let instrumentType = InstrumentType.Spot;
  let fallbackToSpot = true;

  const capitalizedKey = capitalize(
    params.type.toLowerCase(),
  ) as keyof typeof InstrumentType;

  if (Object.keys(InstrumentType).includes(capitalizedKey)) {
    fallbackToSpot = false;
    instrumentType = InstrumentType[capitalizedKey];
  }

  return {
    exchange: Exchanges.Okx, // set default exchange
    instrumentType, // set instrument type from route params, fallback is spot
    fallbackToSpot, // indicate when fallback happened
  };
};
