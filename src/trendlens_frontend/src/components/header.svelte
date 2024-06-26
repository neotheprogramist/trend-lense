<script lang="ts">
	import Navbar from './navbar.svelte';
	import ThemeSwitch from './themeSwitch.svelte';
	import { Button } from '$components/shad/ui/button/index';
	import { wallet } from '$lib/wallet';

	async function connect() {
		if (!$wallet.connected) {
			await wallet.connect();
		}
	}

	async function disconnect() {
		if ($wallet.connected) {
			wallet.disconnect();
		}
	}
</script>

<header
	class="sticky top-0 z-50 w-full border-b border-border/40 bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60"
>
	<div class="container flex h-14 max-w-screen-2xl items-center">
		<Navbar />

		<div class="ml-auto flex justify-center space-x-5">
			<ThemeSwitch />
			{#if $wallet.connected}
				<Button on:click={disconnect}>
					Identity: {$wallet.identity.getPrincipal().toText().substring(0, 6)}...
				</Button>
			{:else}
				<Button on:click={connect}>Connect</Button>
			{/if}
		</div>
	</div>
</header>
