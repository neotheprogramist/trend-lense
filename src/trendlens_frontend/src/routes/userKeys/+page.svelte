<script lang="ts">
	import ApiKeyDialog from '$components/apiKeyDialog.svelte';
	import DataTable from '$components/dataTable.svelte';
	import { wallet } from '$lib/wallet.svelte';
	import { onMount } from 'svelte';
	import type { ApiData } from '../../../../declarations/trendlens_backend/trendlens_backend.did';

	let userKeys = $state<ApiData[]>([]);

	const addApiKey = async (apiKey: string, secretKey: string, passphrase: string) => {
		if (!wallet.connected || !wallet.actor) {
			console.log('Wallet not connected');
			return;
		}

		const newApiKey: ApiData = {
			api_key: apiKey,
			passphrase: [passphrase],
			exchange: { Okx: null }
		};

		await wallet.actor.register_api_key(newApiKey);
		userKeys.push(newApiKey);
	};

	const fetchApiKeys = async () => {
		if (!wallet.connected || !wallet.actor) {
			console.log('Wallet not connected');
			return;
		}

		userKeys = await wallet.actor.get_api_keys();
	};

	$effect(() => {
		if (wallet.connected) {
			fetchApiKeys();
		} else {
			userKeys = [];
		}
	});

	$inspect(userKeys);
	$inspect(wallet.connected);
</script>

<div class="container mx-auto py-10">
	<div class="flex items-center py-4">
		<ApiKeyDialog onUpload={addApiKey} />
	</div>

	<div class="rounded-md border">
		<DataTable bind:data={userKeys}>
			{#if wallet.connected}
				A list of your api keys
			{:else}
				Please connect your wallet
			{/if}
		</DataTable>
	</div>
</div>
