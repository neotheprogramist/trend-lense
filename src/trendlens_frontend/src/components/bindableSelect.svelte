<script lang="ts" generics="T">
	import * as Select from './shad/ui/select/index';

	interface IProps {
		placeholder: string;
		value: T;
		items: T[];
		onChange?: () => void;
	}

	let { value = $bindable(), items, placeholder, onChange }: IProps = $props();
</script>

<Select.Root 
	items={items.map((e) => {
		return { value: e };
	})}
	onSelectedChange={(e) => {
		if (e) {
			value = e.value;

			if (onChange) {
				onChange();
			}
		}
	}}
>
	<Select.Trigger class="w-100%">
		<Select.Value {placeholder} />
	</Select.Trigger>
	<Select.Content>
		{#each items as key}
			<Select.Item value={key}>{key}</Select.Item>
		{/each}
	</Select.Content>
</Select.Root>
