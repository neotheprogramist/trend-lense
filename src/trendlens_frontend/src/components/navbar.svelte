<script lang="ts">
  import { cn } from "$components/utils";
</script>

<div class="hidden md:flex">
  <a href="/" class="mr-5 flex items-center">
    <div class="bg-primary">logo</div>
  </a>

  <nav class="flex items-center space-x-4 text-sm lg:space-x-6">
    <a
      href="/trading/spot"
      class={cn(
        "text-sm font-medium text-muted-foreground transition-colors hover:text-primary",
      )}
    >
      Trading
    </a>

    <a
      href="/userKeys"
      class={cn(
        "text-sm font-medium text-muted-foreground transition-colors hover:text-primary",
      )}
    >
      User keys
    </a>
  </nav>
</div>
