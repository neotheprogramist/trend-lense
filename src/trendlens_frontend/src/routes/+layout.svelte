<script lang="ts">
	import Header from '$components/header.svelte';
	import '$styles/index.scss';
	import { ModeWatcher } from 'mode-watcher';
</script>

<ModeWatcher />

<div class="relative flex min-h-screen flex-col bg-background">
	<Header />
	<slot />
</div>
