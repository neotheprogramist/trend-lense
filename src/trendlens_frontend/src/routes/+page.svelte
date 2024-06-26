<script lang="ts">
	import { anonymousBackend, connect } from '$lib/canisters';
	// import { Principal } from '@dfinity/principal';
	import { onMount } from 'svelte';
	import type {
		Candle,
		_SERVICE
	} from '../../../declarations/trendlens_backend/trendlens_backend.did.d.ts';

	let candles = $state<Candle[]>([]);

	onMount(async () => {
		// if (!connectedBackend) connectedBackend = await connect();
		if (!anonymousBackend) throw new Error('No backend connection');
	});

	const fetchCandles = async () => {
		const timestamp_now = BigInt(Math.floor(Date.now() / 1000));
		const timestamp_minus_5 = timestamp_now - BigInt(5 * 60);

		candles = await anonymousBackend.pull_candles(
			{ BtcUsd: null },
			{ Okx: null },
			timestamp_minus_5,
			timestamp_now
		);
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
