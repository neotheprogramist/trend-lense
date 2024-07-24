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

<div class="flex px-6 mt-5">
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
              <p class="text-green-400 bg-green-900 rounded-xl py-0.5">Buy</p>
            {:else}
              <p class="text-red-400 bg-red-900 rounded-xl py-0.5">Sell</p>
            {/if}
          </td>
          <td class="py-3">{order.instrument_id}</td>
          <td>{order.instrument_type}</td>
          <td>{order.size}</td>
          <td>{order.price}</td>
          <td>{order.trade_mode}</td>
          <td class="uppercase">{order.order_type}</td>
          <td>{order.accumulated_fill_quantity}</td>
          {#if withClose}
            <td><X class="cursor-pointer w-5 stroke-red-400" /></td>
          {/if}
        </tr>
      {/each}
    </tbody>
  </table>
</div>
