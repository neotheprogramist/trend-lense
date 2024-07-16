<script lang="ts">
  import { RequestType } from "$lib/request";
  import Button from "./shad/ui/button/button.svelte";

  interface IProps {
    onRequestPick: (requestType: RequestType) => void;
  }

  let { onRequestPick }: IProps = $props();

  const variants = Object.keys(RequestType).map(
    (k) => k as keyof typeof RequestType,
  ).filter(e => e !== 'Empty');

</script>

<ul class="flex flex-col gap-10">
  {#each variants as key}
    <li>
      <Button onclick={() => onRequestPick(RequestType[key])}>{RequestType[key]}</Button>
    </li>
  {/each}
</ul>
