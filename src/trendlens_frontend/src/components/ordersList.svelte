<script lang="ts">
  import { X } from "lucide-svelte";
  import type { Order } from "../../../declarations/trendlens_backend/trendlens_backend.did";

  interface IProps {
    orders: Order[];
    onOrderClose?: (orderId: string) => void;
    withClose?: boolean;
  }

  let { orders, withClose = true }: IProps = $props();
</script>

<div class="mt-5 flex px-6">
  <table class="w-full">
    <thead>
      <tr>
        <th class="py-4"></th>
        <th>Instrument</th>
        <th>Instrument Type</th>
        <th>Order Total</th>
        <th>Order Price</th>
        <th>Trade Mode</th>
        <th>Order Type</th>
        <th>Order Filled</th>
        {#if withClose}
          <th></th>
        {/if}
      </tr>
    </thead>
    <tbody>
      {#each orders as order}
        <tr class="text-center">
          <td>
            {#if order.side == "buy"}
              <p class="rounded-xl bg-green-900 py-0.5 text-green-400">Buy</p>
            {:else}
              <p class="rounded-xl bg-red-900 py-0.5 text-red-400">Sell</p>
            {/if}
          </td>
          <td class="py-3">{order.instrument_id}</td>
          <td>{order.instrument_type}</td>
          <td>{order.size}</td>
          <td>{order.price ? order.price : "-"}</td>
          <td>{order.trade_mode == "" ? "-" : order.trade_mode}</td>
          <td class="uppercase">{order.order_type}</td>
          <td>{order.accumulated_fill_quantity}</td>
          {#if withClose}
            <td><X class="w-5 cursor-pointer stroke-red-400" /></td>
          {/if}
        </tr>
      {/each}
    </tbody>
  </table>
</div>
