<script type="ts">
  import "../index.scss";
  import { anonymousBackend, connect } from '$lib/canisters';
	import { Principal } from '@dfinity/principal';
	import { onMount } from 'svelte';

  let greeting = $state("");

	onMount(async () => {
		// if (!connectedBackend) connectedBackend = await connect();
		if (!anonymousBackend) throw new Error('No backend connection');
		greeting = await anonymousBackend.greet('user');
	});
</script>

<main>
  <img src="/logo2.svg" alt="DFINITY logo" />
  <br />
  <br />
  <form action="#">
    <label for="name">Enter your name tomek: &nbsp;</label>
    <input id="name" alt="Name" type="text" />
    <button type="submit">Click Me!</button>
  </form>
  <section id="greeting">{greeting}</section>
</main>
