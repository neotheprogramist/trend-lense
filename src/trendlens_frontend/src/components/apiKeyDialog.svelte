<script lang="ts">
	import * as Dialog from '$components/shad/ui/dialog/index';
	import Button from './shad/ui/button/button.svelte';
	import { Label } from './shad/ui/label/index';
	import Input from './shad/ui/input/input.svelte';
	import * as Select from './shad/ui/select/index';
	import { Exchanges, type ExchangeKey } from '$lib/exchange';

	interface IProps {
		onUpload: (exchange: Exchanges, apiKey: string, secretKey: string, passphrase: string) => void;
	}

	let { onUpload }: IProps = $props();
	let apiKey = $state<string>('');
	let secretKey = $state<string>('');
	let passphrase = $state<string>('');
	let exchange = $state<Exchanges>(Exchanges.Okx);

	const exchangeKeys = Object.keys(Exchanges).map((e) => e as ExchangeKey);

	function checkInputs(): boolean {
		if (!exchange) {
			return false;
		}

		if (apiKey.length == 0 || secretKey.length == 0 || passphrase.length == 0) {
			return false;
		}

		return true;
	}

	function handleClick() {
		if (!checkInputs()) return;

		onUpload(exchange!, apiKey, secretKey, passphrase);
	}
</script>

<Dialog.Root>
	<Button><Dialog.Trigger>Add</Dialog.Trigger></Button>

	<Dialog.Content class="sm:max-w-[425px]">
		<Dialog.Header>
			<Dialog.Title>Upload new key</Dialog.Title>
			<Dialog.Description>Enter api key, secret key and passphrase.</Dialog.Description>
		</Dialog.Header>

		<div class="grid gap-3 py-4">
			<div class="grid grid-cols-4 items-center gap-4">
				<Label for="api_key">Exchange</Label>
				<div class="col-span-3 w-100">
					<Select.Root
						selected={{ value: exchange }}
						items={exchangeKeys.map((e) => {
							return {
								value: e as ExchangeKey
							};
						})}
						onSelectedChange={(e) => {
							if (e) {
								exchange = Exchanges[e.value];
							}
						}}
					>
						<Select.Trigger class="w-100">
							<Select.Value placeholder="Exchange" />
						</Select.Trigger>
						<Select.Content>
							{#each exchangeKeys as key}
								<Select.Item value={key}>{key}</Select.Item>
							{/each}
						</Select.Content>
					</Select.Root>
				</div>
			</div>

			<div class="grid grid-cols-4 items-center gap-4">
				<Label for="api_key">Api key</Label>
				<Input id="api_key" class="col-span-3" placeholder="enter api key" bind:value={apiKey} />
			</div>
			<div class="grid grid-cols-4 items-center gap-4">
				<Label for="secret_key">Secret key</Label>
				<Input
					id="secret_key"
					class="col-span-3"
					placeholder="enter secret key"
					bind:value={secretKey}
				/>
			</div>
			<div class="grid grid-cols-4 items-center gap-4">
				<Label for="passphrase" class="text-right">Passphrase</Label>
				<Input
					id="passphrase"
					class="col-span-3"
					placeholder="enter passphrase"
					bind:value={passphrase}
				/>
			</div>
		</div>
		<Dialog.Footer>
			<Button type="submit" on:click={handleClick}>Upload</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>
