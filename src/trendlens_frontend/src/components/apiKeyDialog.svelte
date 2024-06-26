<script lang="ts">
	import * as Dialog from '$components/shad/ui/dialog/index';
	import Button from './shad/ui/button/button.svelte';
	import { Label } from './shad/ui/label/index';
	import Input from './shad/ui/input/input.svelte';

	export let onUpload: (apiKey: string, secretKey: string, passphrase: string) => void;

	function handleClick() {
		const apiKey = 'text';
		const secretKey = 'test';
		const passphrase = '123';

		onUpload(apiKey, secretKey, passphrase);
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
				<Label for="api_key">Api key</Label>
				<Input id="api_key" class="col-span-3" placeholder="enter api key" />
			</div>
			<div class="grid grid-cols-4 items-center gap-4">
				<Label for="secret_key">Secret key</Label>
				<Input id="secret_key" class="col-span-3" placeholder="enter secret key" />
			</div>
			<div class="grid grid-cols-4 items-center gap-4">
				<Label for="passphrase" class="text-right">Passphrase</Label>
				<Input id="passphrase" class="col-span-3" placeholder="enter passphrase" />
			</div>
		</div>
		<Dialog.Footer>
			<Button type="submit" on:click={handleClick}>Upload</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>
