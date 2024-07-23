<script lang="ts">
  import Header from "$components/layout/Header/header.svelte";
  import "$styles/index.scss";
  import { ModeWatcher } from "mode-watcher";
</script>

<ModeWatcher />

<div class="relative flex min-h-screen flex-col bg-background">
  <Header />

  <div class="container max-w-[1800px]">
    <slot />
  </div>
</div>
