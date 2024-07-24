<script lang="ts">
  import { Button } from "$components/shad/ui/button/index";
  import { wallet } from "$lib/wallet.svelte";
  import ThemeSwitch from "../../themeSwitch.svelte";
  import Navbar from "./navbar.svelte";
</script>

<header
  class="sticky top-0 z-50 w-full border-b border-border/40 bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60"
>
  <div class="flex items-center px-8 py-4">
    <Navbar />

    <div class="ml-auto flex justify-center space-x-5">
      <ThemeSwitch />
      {#if wallet.connected && wallet.identity}
        <Button on:click={async () => await wallet.disconnect()}>
          Identity: {wallet.identity.getPrincipal().toText().substring(0, 6)}...
        </Button>
      {:else}
        <Button on:click={async () => await wallet.connect()}
          >Connect wallet</Button
        >
      {/if}
    </div>
  </div>
</header>
