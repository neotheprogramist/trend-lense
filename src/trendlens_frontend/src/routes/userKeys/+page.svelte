<svelte:options runes={false} />

<script lang="ts">
	import ApiKeyDialog from '$components/apiKeyDialog.svelte';
	import DataTable from '$components/dataTable.svelte';
	import { wallet, type IWallet } from '$lib/wallet';

	$: typedWallet = $wallet as IWallet;

	const addApiKey = async (apiKey: string, secretKey: string, passphrase: string) => {
		console.log(apiKey, secretKey, passphrase);
		let res = await typedWallet.actor.register_api_key(
			{ Okx: null },
			{ api_key: apiKey, passphrase: [passphrase] }
		);

		console.log(res)
	};
</script>

<div class="container mx-auto py-10">
	<div class="flex items-center py-4">
		<ApiKeyDialog onUpload={addApiKey} />
	</div>

	<div class="rounded-md border">
		<DataTable />
	</div>
</div>
