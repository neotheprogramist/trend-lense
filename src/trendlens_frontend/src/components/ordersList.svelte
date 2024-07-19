<script lang="ts">
  import type { GlobalPendingOrder } from "../../../declarations/trendlens_backend/trendlens_backend.did";

  interface IProps {
    orders: GlobalPendingOrder[];
  }

  let { orders }: IProps = $props();
</script>

<ul>
  {#each orders as order}
    <li>
      <p>Order ID: {order.order_id}</p>
      <p>Order Status: {order.side}</p>
      <p>Order fill: {order.accumulated_fill_quantity}</p>
      <p>Order Total: {order.size}</p>
      <p>Order Price: {order.price}</p>
    </li>
  {/each}
</ul>
