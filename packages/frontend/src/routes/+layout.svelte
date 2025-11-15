<script lang="ts">
  import "../app.css";
  import UserMenu from "$lib/components/UserMenu.svelte";
  import { Button } from "$lib/components/ui/button";
  import { onMount } from "svelte";
  import { page } from "$app/stores";
  import { resolve } from "$app/paths";
  import { auth } from "$lib/stores/auth";

  let { children } = $props();

  // Pages that don't require authentication
  const publicRoutes = ["/login", "/signup", "/"];

  let isPublicRoute = $derived(publicRoutes.includes($page.url.pathname));

  // Fallback client-side initialization
  onMount(() => {
    auth.init();
  });
</script>

<svelte:head>
  <title>Nore</title>
  <meta name="description" content="A way to manage your projects" />
</svelte:head>

<div class="min-h-screen bg-background flex flex-col">
  <!-- Navigation Header -->
  <header
    class="sticky top-0 z-50 w-full border-b bg-background/95 backdrop-blur supports-backdrop-filter:bg-background/60 shrink-0"
  >
    <div class="container mx-auto flex h-14 items-center">
      <!-- Simple Logo -->
      <div class="mr-4">
        <a href={resolve("/")} class="mr-6 flex items-center space-x-2">
          <div
            class="h-6 w-6 rounded bg-primary flex items-center justify-center"
          >
            <span class="text-primary-foreground font-bold text-sm">S</span>
          </div>
          <span class="font-bold">Nore</span>
        </a>
      </div>

      <!-- Simple Navigation -->
      <nav class="flex items-center space-x-6 text-sm font-medium">
        {#if $auth.isAuthenticated}
          <a href={resolve("/dashboard")} class="hover:text-primary"
            >Dashboard</a
          >
          <a href={resolve("/projects")} class="hover:text-primary">Projects</a>
        {/if}
      </nav>

      <!-- Simple Right side -->
      <div class="flex flex-1 items-center justify-end space-x-2">
        <nav class="flex items-center space-x-2">
          {#if $auth.isLoading}
            <div class="h-8 w-8 rounded-full bg-muted animate-pulse"></div>
          {:else if $auth.isAuthenticated}
            <UserMenu />
          {:else}
            <div class="flex items-center space-x-2">
              <Button variant="ghost" href="/login" size="sm">Sign In</Button>
              <Button href="/signup" size="sm">Sign Up</Button>
            </div>
          {/if}
        </nav>
      </div>
    </div>
  </header>

  <!-- Main Content -->
  <main class="flex-1 flex flex-col">
    {@render children()}
  </main>

  <!-- Footer -->
  <footer class="border-t shrink-0">
    <div
      class="container mx-auto flex flex-col gap-4 px-8 h-10 items-center justify-center"
    >
      <p
        class="text-center text-sm leading-loose text-muted-foreground md:text-left"
      >
        Built with
        <a
          href="https://kit.sveltekit.dev"
          target="_blank"
          rel="noreferrer"
          class="font-medium underline underline-offset-4"
        >
          SvelteKit
        </a>
        and
        <a
          href="https://www.rust-lang.org"
          target="_blank"
          rel="noreferrer"
          class="font-medium underline underline-offset-4"
        >
          Rust
        </a>
      </p>
    </div>
  </footer>
</div>
