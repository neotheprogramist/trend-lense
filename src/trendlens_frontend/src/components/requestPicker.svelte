<script lang="ts">
	import { RequestType } from '$lib/request';
    import Button from './shad/ui/button/button.svelte';
    
	interface IProps {
		onRequestPick: (requestType: RequestType) => void;
	}

	let { onRequestPick }: IProps = $props();

	const descriptions = [
		{
			request: RequestType.Instruments,
			label: 'get instruments'
		}
	];
</script>

<ul class="flex gap-10">
	{#each descriptions as { label, request }}
		<li>
			<Button onclick={() => onRequestPick(request)}>{label}</Button>
		</li>
	{/each}
</ul>
