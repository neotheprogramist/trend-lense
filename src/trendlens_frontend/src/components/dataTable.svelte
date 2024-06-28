<script lang="ts">
	import { createTable, Render, Subscribe, createRender } from 'svelte-headless-table';
	import { readable } from 'svelte/store';
	import * as Table from '$components/shad/ui/table/index';
	import { Exchanges } from '$lib/exchange';
	import DataTableActions from './dataTableActions.svelte';

	type ApiData = {
		apiKey: string;
		exchange: Exchanges;
		status: 'active' | 'revoked';
	};

	const data: ApiData[] = [
		{
			apiKey: 'm5gr84i9',
			exchange: Exchanges.Okx,
			status: 'active'
		},
		{
			apiKey: 'm5gr84i9',
			exchange: Exchanges.Coinbase,
			status: 'active'
		}
	];

	const table = createTable(readable(data));

	const columns = table.createColumns([
		table.column({
			accessor: 'apiKey',
			header: 'Api key'
		}),
		table.column({
			accessor: 'exchange',
			header: 'Exchange'
		}),
		table.column({
			accessor: 'status',
			header: 'Status'
		}),
		table.column({
			accessor: ({ apiKey }) => apiKey,
			header: '',
			cell: ({ value }) => {
				return createRender(DataTableActions, { id: value });
			}
		})
	]);

	const { headerRows, pageRows, tableAttrs, tableBodyAttrs } = table.createViewModel(columns);
</script>

<div class="rounded-md border">
	<Table.Root {...$tableAttrs}>
		<Table.Header>
			{#each $headerRows as headerRow}
				<Subscribe rowAttrs={headerRow.attrs()}>
					<Table.Row>
						{#each headerRow.cells as cell (cell.id)}
							<Subscribe attrs={cell.attrs()} let:attrs props={cell.props()}>
								<Table.Head {...attrs}>
									<Render of={cell.render()} />
								</Table.Head>
							</Subscribe>
						{/each}
					</Table.Row>
				</Subscribe>
			{/each}
		</Table.Header>
		<Table.Body {...$tableBodyAttrs}>
			{#each $pageRows as row (row.id)}
				<Subscribe rowAttrs={row.attrs()} let:rowAttrs>
					<Table.Row {...rowAttrs}>
						{#each row.cells as cell (cell.id)}
							<Subscribe attrs={cell.attrs()} let:attrs>
								<Table.Cell {...attrs}>
									<Render of={cell.render()} />
								</Table.Cell>
							</Subscribe>
						{/each}
					</Table.Row>
				</Subscribe>
			{/each}
		</Table.Body>
	</Table.Root>
</div>
