// See https://kit.svelte.dev/docs/types#app

import type { RequestPickState, RequestType } from "$lib/request";

// for information about these interfaces
declare global {
	namespace App {
		// interface Error {}
		// interface Locals {}
		// interface PageData {}
		interface PageState {
			requestPickState: RequestPickState | null;
			requestType: RequestType | null;
		}
		// interface Platform {}
	}
}

export {};
