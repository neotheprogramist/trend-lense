<script lang="ts">
  import Header from "$components/layout/Header/header.svelte";
  import Modal from "$components/modal.svelte";
  import { Button } from "$components/shad/ui/button";
  import "$styles/index.scss";
  import { ModeWatcher } from "mode-watcher";
  import { onMount } from "svelte";

  let showModal = $state(false);

  onMount(() => {
    showModal = true;
  });
</script>

<ModeWatcher />

<Modal bind:showModal>
  <h1 slot="header" class="text-center text-4xl font-bold text-orange-600">
    Disclaimer
  </h1>

  <div class="mt-6 flex flex-col space-y-10">
    <p class="text-center">
      This application is for testing purposes only. Do not use real private
      keys or sensitive information from exchanges. Using genuine data may lead
      to unintended exposure of sensitive information. We are not responsible
      for any consequences resulting from the use of this application. Please be
      aware that the information provided may be inaccurate or incorrect.
    </p>

    <Button
      on:click={() => {
        showModal = false;
      }}>Continue</Button
    >
  </div>
</Modal>

<div class="relative flex min-h-screen flex-col bg-background">
  <Header />

  <div class="container max-w-[1800px]">
    <slot />
  </div>
</div>
