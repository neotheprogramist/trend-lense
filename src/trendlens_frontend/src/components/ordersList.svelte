<script lang="ts">
  import { X } from "lucide-svelte";
  import type { GlobalPendingOrder } from "../../../declarations/trendlens_backend/trendlens_backend.did";

  interface IProps {
    orders: GlobalPendingOrder[];
    onOrderClose?: (orderId: string) => void;
  }

  let { orders }: IProps = $props();
</script>

<div class="flex px-6 mt-5">
  <table class="w-full">
    <thead>
      <tr>
        <th></th>
        <th class="py-4">Instrument</th>
        <th>Instrument Type</th>
        <th>Order Total</th>
        <th>Order Price</th>
        <th>Trade Mode</th>
        <th>Order Type</th>
        <th>Order Filled</th>
        <th></th>
      </tr>
    </thead>
    <tbody>
      {#each [...orders, ...orders] as order}
        <tr class="text-center">
          <td>
            {#if order.side == "buy"}
              <p class="text-green-400 bg-green-900 rounded-xl px-1 py-0.5">
                Buy
              </p>
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
          <td><X class="cursor-pointer w-5 stroke-red-400" /></td>
        </tr>
      {/each}
    </tbody>
  </table>
</div>
