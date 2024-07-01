<script lang="ts">
	import type { Exchanges } from '$lib/exchange';
	import type { RequestType } from '$lib/request';
	import type { Pair } from '../../../declarations/trendlens_backend/trendlens_backend.did';
	import RequestPicker from './requestPicker.svelte';
	// import { page } from '$app/stores';
	// import { pushState } from '$app/navigation';
	import ApiKeyPicker from './apiKeyPicker.svelte';

	// right now i pass exchange as prop, but it could be store or context

	interface IProps {
		exchange: Exchanges;
	}

	let { exchange }: IProps = $props();

	const handleRequestPick = (requestType: RequestType) => {
	};
</script>

<ApiKeyPicker exchange={exchange} onApiKeyPick={() => console.log('3l')} />
<RequestPicker onRequestPick={handleRequestPick} />
