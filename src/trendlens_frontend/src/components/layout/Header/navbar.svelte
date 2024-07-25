<script lang="ts">
  import { cn } from "$components/utils";

  let navData = [
    {
      title: "Home",
      href: "/",
      isActive: false,
    },
    {
      title: "Trading",
      href: "/trading/spot",
      isActive: false,
    },
    {
      title: "User keys",
      href: "/userKeys",
      isActive: false,
    },
  ];

  let nav = $state(navData);
</script>

<div class="hidden md:flex">
  <nav class="flex items-center space-x-4 text-sm lg:space-x-6">
    {#each nav as { title, href, isActive }, i}
      <a
        {href}
        class={cn(
          "text-sm font-medium text-muted-foreground transition-colors hover:text-primary",
          isActive && "text-orange-700",
        )}
      >
        {title}
      </a>
    {/each}
  </nav>
</div>
