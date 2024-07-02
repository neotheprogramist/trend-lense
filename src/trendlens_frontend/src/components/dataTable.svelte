<script lang="ts">
  import {
    createTable,
    Render,
    Subscribe,
    createRender,
  } from "svelte-headless-table";
  import {  writable, type Writable } from "svelte/store";
  import * as Table from "$components/shad/ui/table/index";
  import DataTableActions from "./dataTableActions.svelte";
    import type { ApiData } from "$lib/keystore.svelte";


  interface IProps {
    children: any;
    data: ApiData[];
    removeCallback: (id: string) => void;
  }

  let { children, data, removeCallback }: IProps = $props();

  const table = createTable(writable(data));

  const columns = table.createColumns([
    table.column({
      accessor: "apiKey",
      header: "Api key",
    }),
    table.column({
      accessor: "exchange",
      header: "Exchange",
    }),
    table.column({
      accessor: ({ apiKey }) => apiKey,
      header: "",
      cell: ({ value }) => {
        return createRender(DataTableActions, { id: value, removeCallback });
      },
    }),
  ]);

  $effect(() => {
    console.log("running effect");
    let store = table.data as Writable<ApiData[]>;
    store.set(data);
  });

  const { headerRows, pageRows, tableAttrs, tableBodyAttrs } =
    table.createViewModel(columns);
</script>

<div class="rounded-md border">
  <Table.Root {...$tableAttrs}>
    <Table.Caption class="p-4">{@render children()}</Table.Caption>
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
