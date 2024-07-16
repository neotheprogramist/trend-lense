<script lang="ts">
	import { cn } from '$components/utils';
</script>

<div class="hidden md:flex">
	<a href="/" class="flex items-center mr-5">
		<div class="bg-primary">logo</div>
	</a>

	<nav class="flex items-center space-x-4 lg:space-x-6 text-sm">
		<a
			href="/trading/spot"
			class={cn('text-sm font-medium text-muted-foreground transition-colors hover:text-primary')}
		>
			Trading
		</a>

		<a
			href="/userKeys"
			class={cn('text-sm font-medium text-muted-foreground transition-colors hover:text-primary')}
		>
			User keys
		</a>
	</nav>
</div>
