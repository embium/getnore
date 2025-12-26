<script lang="ts">
  import { page } from "$app/state";
  import { Button } from "$lib/components/ui/button";
  import * as Card from "$lib/components/ui/card";
  import {
    LayoutDashboard,
    Users,
    FolderOpen,
    CreditCard,
    Settings,
    ChevronRight,
  } from "lucide-svelte";

  const navigation = [
    {
      name: "Dashboard",
      href: "/admin",
      icon: LayoutDashboard,
    },
    {
      name: "Users",
      href: "/admin/users",
      icon: Users,
    },
    {
      name: "Projects",
      href: "/admin/projects",
      icon: FolderOpen,
    },
    {
      name: "Subscription Plans",
      href: "/admin/plans",
      icon: CreditCard,
    },
    {
      name: "Settings",
      href: "/admin/settings",
      icon: Settings,
    },
  ];

  function isActive(href: string): boolean {
    return page.url.pathname === href || page.url.pathname.startsWith(href + "/");
  }
</script>

<svelte:head>
  <title>Admin Panel - SaaS Boilerplate</title>
</svelte:head>

<div class="flex h-screen bg-muted/30">
  <!-- Sidebar -->
  <div class="w-64 bg-background border-r hidden md:block">
    <div class="p-6">
      <div class="flex items-center space-x-2 mb-8">
        <div class="h-8 w-8 bg-primary rounded-lg flex items-center justify-center">
          <span class="text-primary-foreground font-bold text-sm">A</span>
        </div>
        <span class="text-xl font-bold">Admin</span>
      </div>

      <nav class="space-y-2">
        {#each navigation as item (item.href)}
          <Button
            variant={isActive(item.href) ? "secondary" : "ghost"}
            class="w-full justify-start"
            href={item.href}
          >
            <item.icon class="mr-3 h-4 w-4" />
            {item.name}
            {#if isActive(item.href)}
              <ChevronRight class="ml-auto h-4 w-4" />
            {/if}
          </Button>
        {/each}
      </nav>
    </div>
  </div>

  <!-- Main Content -->
  <div class="flex-1 overflow-auto">
    <header class="bg-background border-b px-6 py-4">
      <div class="flex items-center justify-between">
        <div>
          <h1 class="text-2xl font-bold">Admin Panel</h1>
          <p class="text-muted-foreground">Manage your application</p>
        </div>
        <div class="flex items-center space-x-4">
          <Button variant="outline" size="sm" href="/dashboard">
            Back to Dashboard
          </Button>
        </div>
      </div>
    </header>

    <main class="p-6">
      <slot />
    </main>
  </div>
</div>

<!-- Mobile Navigation -->
<div class="md:hidden fixed bottom-0 left-0 right-0 bg-background border-t">
  <nav class="flex justify-around py-2">
    {#each navigation as item (item.href)}
      <Button
        variant={isActive(item.href) ? "secondary" : "ghost"}
        size="sm"
        class="flex-col h-12 px-3"
        href={item.href}
      >
        <item.icon class="h-4 w-4 mb-1" />
        <span class="text-xs">{item.name}</span>
      </Button>
    {/each}
  </nav>
</div>