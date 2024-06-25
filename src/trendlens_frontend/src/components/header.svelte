<script lang="ts">
	import Navbar from './navbar.svelte';
	import ThemeSwitch from './themeSwitch.svelte';
</script>

<header
	class="sticky top-0 z-50 w-full border-b border-border/40 bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60"
>
	<div class="container flex h-14 max-w-screen-2xl items-center">
		<Navbar />

		<div class="ml-auto flex justify-center">
			<ThemeSwitch />
		</div>
	</div>
</header>
