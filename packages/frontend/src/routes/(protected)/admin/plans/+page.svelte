<script lang="ts">
  import { onMount } from "svelte";
  import { adminAPI, type Plan, type CreatePlan, type UpdatePlan } from "$lib/api/admin";
  import { Button } from "$lib/components/ui/button";
  import * as Card from "$lib/components/ui/card";
  import * as Dialog from "$lib/components/ui/dialog";
  import { Input } from "$lib/components/ui/input";
  import { Label } from "$lib/components/ui/label";
  import { Textarea } from "$lib/components/ui/textarea";
  import { Badge } from "$lib/components/ui/badge";
  import LoadingSpinner from "$lib/components/LoadingSpinner.svelte";
  import {
    CreditCard,
    Search,
    Plus,
    Edit,
    Trash2,
    DollarSign,
    Package,
    Clock,
    Star,
  } from "lucide-svelte";

  let plans = $state<Plan[]>([]);
  let filteredPlans = $state<Plan[]>([]);
  let isLoading = $state(true);
  let error = $state("");
  let searchTerm = $state("");
  let isCreateDialogOpen = $state(false);
  let isEditDialogOpen = $state(false);
  let selectedPlan = $state<Plan | null>(null);

  // Form states
  let createForm = $state<CreatePlan>({
    name: "",
    description: "",
    price: 0,
  });

  let editForm = $state<UpdatePlan>({
    name: "",
    description: "",
    price: undefined,
  });

  onMount(async () => {
    await loadPlans();
  });

  async function loadPlans() {
    isLoading = true;
    error = "";

    try {
      plans = await adminAPI.listPlans();
      filteredPlans = plans;
    } catch (err) {
      error = err instanceof Error ? err.message : "Failed to load subscription plans";
    } finally {
      isLoading = false;
    }
  }

  function filterPlans() {
    if (!searchTerm) {
      filteredPlans = plans;
      return;
    }

    const term = searchTerm.toLowerCase();
    filteredPlans = plans.filter(
      (plan) =>
        plan.name.toLowerCase().includes(term) ||
        plan.description?.toLowerCase().includes(term)
    );
  }

  $effect(() => {
    filterPlans();
  });

  function openCreateDialog() {
    createForm = { name: "", description: "", price: 0 };
    isCreateDialogOpen = true;
  }

  function openEditDialog(plan: Plan) {
    selectedPlan = plan;
    editForm = {
      name: plan.name,
      description: plan.description || "",
      price: plan.price,
    };
    isEditDialogOpen = true;
  }

  async function handleCreatePlan() {
    try {
      await adminAPI.createPlan(createForm);
      await loadPlans();
      isCreateDialogOpen = false;
    } catch (err) {
      error = err instanceof Error ? err.message : "Failed to create plan";
    }
  }

  async function handleUpdatePlan() {
    if (!selectedPlan?.id) return;

    try {
      await adminAPI.updatePlan(selectedPlan.id, editForm);
      await loadPlans();
      isEditDialogOpen = false;
    } catch (err) {
      error = err instanceof Error ? err.message : "Failed to update plan";
    }
  }

  async function handleDeletePlan(plan: Plan) {
    if (!plan.id || !confirm(`Are you sure you want to delete "${plan.name}"?`)) return;

    try {
      await adminAPI.deletePlan(plan.id);
      await loadPlans();
    } catch (err) {
      error = err instanceof Error ? err.message : "Failed to delete plan";
    }
  }

  function formatDate(dateString: string): string {
    return new Date(dateString).toLocaleDateString("en-US", {
      year: "numeric",
      month: "short",
      day: "numeric",
    });
  }

  function getPlanFeatures(plan: Plan): string[] {
    // Mock features - in a real app, these would come from the API
    const features = [
      "Basic support",
      "Core features",
      "Standard usage limits",
    ];

    if (plan.price > 50) {
      features.push("Priority support", "Advanced features");
    }
    if (plan.price > 100) {
      features.push("Unlimited usage", "Custom integrations");
    }

    return features;
  }

  function getPlanColor(plan: Plan): string {
    if (plan.price < 30) return "border-green-200 bg-green-50";
    if (plan.price < 80) return "border-blue-200 bg-blue-50";
    return "border-purple-200 bg-purple-50";
  }
</script>

<div class="space-y-6">
  <!-- Page Header -->
  <div class="flex items-center justify-between">
    <div>
      <h2 class="text-3xl font-bold tracking-tight">Subscription Plans</h2>
      <p class="text-muted-foreground">
        Manage subscription tiers and pricing
      </p>
    </div>
    <Button onclick={openCreateDialog}>
      <Plus class="mr-2 h-4 w-4" />
      Create Plan
    </Button>
  </div>

  <!-- Search Bar -->
  <Card.Root>
    <Card.Content class="p-4">
      <div class="relative">
        <Search class="absolute left-3 top-1/2 transform -translate-y-1/2 text-muted-foreground h-4 w-4" />
        <Input
          type="text"
          placeholder="Search plans by name or description..."
          class="pl-10"
          bind:value={searchTerm}
        />
      </div>
    </Card.Content>
  </Card.Root>

  <!-- Plans Grid -->
  {#if isLoading}
    <div class="flex items-center justify-center py-12">
      <LoadingSpinner size={32} text="Loading subscription plans..." />
    </div>
  {:else if error}
    <Card.Root>
      <Card.Content class="py-8">
        <div class="text-center space-y-4">
          <p class="text-destructive">{error}</p>
          <Button onclick={loadPlans} variant="outline">
            Try Again
          </Button>
        </div>
      </Card.Content>
    </Card.Root>
  {:else if filteredPlans.length === 0}
    <Card.Root>
      <Card.Content class="py-12">
        <div class="text-center space-y-4">
          <CreditCard class="h-12 w-12 mx-auto text-muted-foreground opacity-50" />
          <div>
            <h3 class="font-medium">No subscription plans</h3>
            <p class="text-sm text-muted-foreground">
              {searchTerm ? "Try adjusting your search terms" : "Get started by creating your first subscription plan"}
            </p>
          </div>
          {#if !searchTerm}
            <Button onclick={openCreateDialog}>
              <Plus class="mr-2 h-4 w-4" />
              Create Plan
            </Button>
          {/if}
        </div>
      </Card.Content>
    </Card.Root>
  {:else}
    <div class="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
      {#each filteredPlans as plan (plan.id)}
        <Card.Root class="{getPlanColor(plan)} border-2 hover:shadow-lg transition-all">
          <Card.Header>
            <div class="flex items-start justify-between">
              <div class="space-y-1">
                <Card.Title class="text-xl">{plan.name}</Card.Title>
                {#if plan.description}
                  <Card.Description>{plan.description}</Card.Description>
                {/if}
              </div>
              <Badge variant="outline" class="text-lg font-bold">
                ${plan.price}/mo
              </Badge>
            </div>
          </Card.Header>
          <Card.Content>
            <div class="space-y-4">
              <!-- Features -->
              <div class="space-y-2">
                <h4 class="font-medium text-sm">Features:</h4>
                <ul class="space-y-1">
                  {#each getPlanFeatures(plan) as feature}
                    <li class="flex items-center space-x-2 text-sm">
                      <Star class="h-3 w-3 text-yellow-500" />
                      <span>{feature}</span>
                    </li>
                  {/each}
                </ul>
              </div>

              <!-- Metadata -->
              <div class="pt-4 border-t space-y-2">
                <div class="flex items-center space-x-2 text-sm text-muted-foreground">
                  <Clock class="h-4 w-4" />
                  <span>Created {plan.created_at ? formatDate(plan.created_at) : "N/A"}</span>
                </div>
                {#if plan.updated_at && plan.updated_at !== plan.created_at}
                  <div class="flex items-center space-x-2 text-sm text-muted-foreground">
                    <Package class="h-4 w-4" />
                    <span>Updated {formatDate(plan.updated_at)}</span>
                  </div>
                {/if}
              </div>
            </div>
          </Card.Content>
          <Card.Content class="pt-0">
            <div class="flex items-center justify-end space-x-2">
              <Button
                variant="ghost"
                size="sm"
                onclick={() => openEditDialog(plan)}
              >
                <Edit class="h-4 w-4 mr-1" />
                Edit
              </Button>
              <Button
                variant="ghost"
                size="sm"
                onclick={() => handleDeletePlan(plan)}
                class="text-red-600 hover:text-red-700"
              >
                <Trash2 class="h-4 w-4 mr-1" />
                Delete
              </Button>
            </div>
          </Card.Content>
        </Card.Root>
      {/each}
    </div>
  {/if}
</div>

<!-- Create Plan Dialog -->
<Dialog.Root bind:open={isCreateDialogOpen}>
  <Dialog.Content>
    <Dialog.Header>
      <Dialog.Title>Create Subscription Plan</Dialog.Title>
      <Dialog.Description>
        Add a new subscription tier to your pricing
      </Dialog.Description>
    </Dialog.Header>
    <div class="space-y-4 py-4">
      <div class="space-y-2">
        <Label for="create-name">Plan Name</Label>
        <Input
          id="create-name"
          type="text"
          placeholder="Pro Plan"
          bind:value={createForm.name}
        />
      </div>
      <div class="space-y-2">
        <Label for="create-description">Description</Label>
        <Textarea
          id="create-description"
          placeholder="Perfect for growing teams..."
          bind:value={createForm.description}
          rows={3}
        />
      </div>
      <div class="space-y-2">
        <Label for="create-price">Monthly Price ($)</Label>
        <Input
          id="create-price"
          type="number"
          min="0"
          step="0.01"
          placeholder="29.99"
          bind:value={createForm.price}
        />
      </div>
    </div>
    <Dialog.Footer>
      <Button variant="outline" onclick={() => (isCreateDialogOpen = false)}>
        Cancel
      </Button>
      <Button onclick={handleCreatePlan}>Create Plan</Button>
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>

<!-- Edit Plan Dialog -->
<Dialog.Root bind:open={isEditDialogOpen}>
  <Dialog.Content>
    <Dialog.Header>
      <Dialog.Title>Edit Subscription Plan</Dialog.Title>
      <Dialog.Description>
        Update plan details
      </Dialog.Description>
    </Dialog.Header>
    <div class="space-y-4 py-4">
      <div class="space-y-2">
        <Label for="edit-name">Plan Name</Label>
        <Input
          id="edit-name"
          type="text"
          bind:value={editForm.name}
        />
      </div>
      <div class="space-y-2">
        <Label for="edit-description">Description</Label>
        <Textarea
          id="edit-description"
          bind:value={editForm.description}
          rows={3}
        />
      </div>
      <div class="space-y-2">
        <Label for="edit-price">Monthly Price ($)</Label>
        <Input
          id="edit-price"
          type="number"
          min="0"
          step="0.01"
          bind:value={editForm.price}
        />
      </div>
    </div>
    <Dialog.Footer>
      <Button variant="outline" onclick={() => (isEditDialogOpen = false)}>
        Cancel
      </Button>
      <Button onclick={handleUpdatePlan}>Save Changes</Button>
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>