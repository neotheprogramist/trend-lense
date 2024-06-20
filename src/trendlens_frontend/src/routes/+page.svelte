<script lang="ts">
	import '../index.scss';
	import { anonymousBackend, connect } from '$lib/canisters';
	// import { Principal } from '@dfinity/principal';
	import { onMount } from 'svelte';
	import type { Candle, _SERVICE } from '../../../declarations/trendlens_backend/trendlens_backend.did.d.ts';

	let candles = $state<Candle[]>([]);

	onMount(async () => {
		// if (!connectedBackend) connectedBackend = await connect();
		if (!anonymousBackend) throw new Error('No backend connection');
	});

	const fetchCandles = async () => {
		candles = await anonymousBackend.fetch_from_api({ BtcUsd: null }, { Okx: null });
	};
</script>

<main>
	<img src="/logo2.svg" alt="DFINITY logo" />
	<br />
	<br />
	<form action="#" onsubmit={fetchCandles}>
		<button type="submit">fetch candles</button>
	</form>

	<section id="candle_data">
		<ol>
			{#each candles as candle}
				<li>
					<span>open price</span>
					<span> {candle.open_price}</span>
				</li>
			{/each}
		</ol>
	</section>
</main>
