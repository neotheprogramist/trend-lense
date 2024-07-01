<script lang="ts">
	import { RequestType } from '$lib/request';
    
	interface IProps {
		onRequestPick: (requestType: RequestType) => void;
	}

	let { onRequestPick }: IProps = $props();

	const descriptions = [
		{ request: RequestType.Empty, label: 'mock request' },
		{
			request: RequestType.Instruments,
			label: 'get instruments'
		}
	];
</script>

<ul>
	{#each descriptions as { label, request }}
		<li>
			<button onclick={() => onRequestPick(request)}>{label}</button>
		</li>
	{/each}
</ul>
