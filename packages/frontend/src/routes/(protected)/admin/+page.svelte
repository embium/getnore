<script lang="ts">
  import { onMount } from "svelte";
  import { projectsAPI, type Project } from "$lib/api/projects";
  import { adminAPI, type Plan } from "$lib/api/admin";
  import { Button } from "$lib/components/ui/button";
  import * as Card from "$lib/components/ui/card";
  import LoadingSpinner from "$lib/components/LoadingSpinner.svelte";
  import {
    Users,
    FolderOpen,
    CreditCard,
    TrendingUp,
    Activity,
    ArrowUpRight,
    ArrowDownRight,
  } from "lucide-svelte";

  let stats = $state({
    totalUsers: 0,
    totalProjects: 0,
    totalPlans: 0,
    activeProjects: 0,
  });

  let recentProjects = $state<Project[]>([]);
  let subscriptionPlans = $state<Plan[]>([]);
  let isLoading = $state(true);
  let error = $state("");

  onMount(async () => {
    await loadDashboardData();
  });

  async function loadDashboardData() {
    isLoading = true;
    error = "";

    try {
      // Load projects
      const projects = await projectsAPI.listProjects();
      recentProjects = projects.slice(0, 5);
      
      // Load subscription plans
      const plans = await adminAPI.listPlans();
      subscriptionPlans = plans.slice(0, 3);

      // Calculate stats
      stats = {
        totalUsers: Math.floor(Math.random() * 100) + 50, // Mock data - replace with actual API
        totalProjects: projects.length,
        totalPlans: plans.length,
        activeProjects: projects.filter(p => p.created_at && 
          new Date(p.created_at) > new Date(Date.now() - 30 * 24 * 60 * 60 * 1000)).length,
      };
    } catch (err) {
      error = err instanceof Error ? err.message : "Failed to load dashboard data";
    } finally {
      isLoading = false;
    }
  }

  function formatDate(dateString: string): string {
    return new Date(dateString).toLocaleDateString("en-US", {
      year: "numeric",
      month: "short",
      day: "numeric",
    });
  }

  function getInitials(name: string): string {
    return name
      .split(" ")
      .map((word) => word.charAt(0))
      .join("")
      .toUpperCase()
      .substring(0, 2);
  }
</script>

<div class="space-y-6">
  <!-- Page Header -->
  <div class="flex items-center justify-between">
    <div>
      <h2 class="text-3xl font-bold tracking-tight">Dashboard</h2>
      <p class="text-muted-foreground">
        Overview of your application metrics and activity
      </p>
    </div>
    <Button onclick={loadDashboardData} variant="outline" size="sm">
      <Activity class="mr-2 h-4 w-4" />
      Refresh
    </Button>
  </div>

  {#if isLoading}
    <div class="flex items-center justify-center py-12">
      <LoadingSpinner size={32} text="Loading dashboard..." />
    </div>
  {:else if error}
    <Card.Root>
      <Card.Content class="py-8">
        <div class="text-center space-y-4">
          <p class="text-destructive">{error}</p>
          <Button onclick={loadDashboardData} variant="outline">
            Try Again
          </Button>
        </div>
      </Card.Content>
    </Card.Root>
  {:else}
    <!-- Stats Grid -->
    <div class="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
      <Card.Root>
        <Card.Content class="p-6">
          <div class="flex items-center justify-between">
            <div class="space-y-2">
              <p class="text-sm font-medium text-muted-foreground">Total Users</p>
              <p class="text-2xl font-bold">{stats.totalUsers}</p>
              <p class="text-xs text-muted-foreground flex items-center">
                <ArrowUpRight class="h-3 w-3 mr-1 text-green-500" />
                +12% from last month
              </p>
            </div>
            <div class="h-12 w-12 bg-blue-100 rounded-lg flex items-center justify-center">
              <Users class="h-6 w-6 text-blue-600" />
            </div>
          </div>
        </Card.Content>
      </Card.Root>

      <Card.Root>
        <Card.Content class="p-6">
          <div class="flex items-center justify-between">
            <div class="space-y-2">
              <p class="text-sm font-medium text-muted-foreground">Total Projects</p>
              <p class="text-2xl font-bold">{stats.totalProjects}</p>
              <p class="text-xs text-muted-foreground flex items-center">
                <ArrowUpRight class="h-3 w-3 mr-1 text-green-500" />
                +8% from last month
              </p>
            </div>
            <div class="h-12 w-12 bg-green-100 rounded-lg flex items-center justify-center">
              <FolderOpen class="h-6 w-6 text-green-600" />
            </div>
          </div>
        </Card.Content>
      </Card.Root>

      <Card.Root>
        <Card.Content class="p-6">
          <div class="flex items-center justify-between">
            <div class="space-y-2">
              <p class="text-sm font-medium text-muted-foreground">Active Projects</p>
              <p class="text-2xl font-bold">{stats.activeProjects}</p>
              <p class="text-xs text-muted-foreground flex items-center">
                <ArrowDownRight class="h-3 w-3 mr-1 text-red-500" />
                -2% from last month
              </p>
            </div>
            <div class="h-12 w-12 bg-purple-100 rounded-lg flex items-center justify-center">
              <Activity class="h-6 w-6 text-purple-600" />
            </div>
          </div>
        </Card.Content>
      </Card.Root>

      <Card.Root>
        <Card.Content class="p-6">
          <div class="flex items-center justify-between">
            <div class="space-y-2">
              <p class="text-sm font-medium text-muted-foreground">Subscription Plans</p>
              <p class="text-2xl font-bold">{stats.totalPlans}</p>
              <p class="text-xs text-muted-foreground flex items-center">
                <TrendingUp class="h-3 w-3 mr-1 text-blue-500" />
                Available plans
              </p>
            </div>
            <div class="h-12 w-12 bg-orange-100 rounded-lg flex items-center justify-center">
              <CreditCard class="h-6 w-6 text-orange-600" />
            </div>
          </div>
        </Card.Content>
      </Card.Root>
    </div>

    <div class="grid gap-6 md:grid-cols-2">
      <!-- Recent Projects -->
      <Card.Root>
        <Card.Header>
          <div class="flex items-center justify-between">
            <div>
              <Card.Title>Recent Projects</Card.Title>
              <Card.Description>Latest projects created</Card.Description>
            </div>
            <Button href="/admin/projects" variant="outline" size="sm">
              View All
            </Button>
          </div>
        </Card.Header>
        <Card.Content>
          {#if recentProjects.length === 0}
            <div class="text-center py-8 text-muted-foreground">
              <FolderOpen class="h-12 w-12 mx-auto mb-4 opacity-50" />
              <p>No recent projects</p>
            </div>
          {:else}
            <div class="space-y-4">
              {#each recentProjects as project (project.id)}
                <div class="flex items-center space-x-4 p-3 rounded-lg hover:bg-muted/50 transition-colors">
                  <div class="h-10 w-10 rounded-full bg-primary/10 flex items-center justify-center">
                    <span class="text-sm font-medium text-primary">
                      {getInitials(project.name)}
                    </span>
                  </div>
                  <div class="flex-1 space-y-1">
                    <h4 class="font-medium">{project.name}</h4>
                    <p class="text-sm text-muted-foreground">
                      {project.user_email}
                    </p>
                  </div>
                  <div class="text-sm text-muted-foreground">
                    {project.created_at ? formatDate(project.created_at) : 'N/A'}
                  </div>
                </div>
              {/each}
            </div>
          {/if}
        </Card.Content>
      </Card.Root>

      <!-- Subscription Plans -->
      <Card.Root>
        <Card.Header>
          <div class="flex items-center justify-between">
            <div>
              <Card.Title>Subscription Plans</Card.Title>
              <Card.Description>Available subscription tiers</Card.Description>
            </div>
            <Button href="/admin/plans" variant="outline" size="sm">
              Manage
            </Button>
          </div>
        </Card.Header>
        <Card.Content>
          {#if subscriptionPlans.length === 0}
            <div class="text-center py-8 text-muted-foreground">
              <CreditCard class="h-12 w-12 mx-auto mb-4 opacity-50" />
              <p>No subscription plans</p>
            </div>
          {:else}
            <div class="space-y-4">
              {#each subscriptionPlans as plan (plan.id)}
                <div class="flex items-center justify-between p-3 rounded-lg border">
                  <div class="space-y-1">
                    <h4 class="font-medium">{plan.name}</h4>
                    {#if plan.description}
                      <p class="text-sm text-muted-foreground">{plan.description}</p>
                    {/if}
                  </div>
                  <div class="text-right">
                    <p class="text-lg font-bold">${plan.price}</p>
                    <p class="text-xs text-muted-foreground">/month</p>
                  </div>
                </div>
              {/each}
            </div>
          {/if}
        </Card.Content>
      </Card.Root>
    </div>
  {/if}
</div>