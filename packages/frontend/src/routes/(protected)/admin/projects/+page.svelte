<script lang="ts">
  import { onMount } from "svelte";
  import { projectsAPI, type Project } from "$lib/api/projects";
  import { Button } from "$lib/components/ui/button";
  import * as Card from "$lib/components/ui/card";
  import * as Dialog from "$lib/components/ui/dialog";
  import { Input } from "$lib/components/ui/input";
  import { Label } from "$lib/components/ui/label";
  import { Badge } from "$lib/components/ui/badge";
  import LoadingSpinner from "$lib/components/LoadingSpinner.svelte";
  import {
    FolderOpen,
    Search,
    Plus,
    Edit,
    Trash2,
    Calendar,
    User,
    FileText,
    Clock,
  } from "lucide-svelte";

  let projects = $state<Project[]>([]);
  let filteredProjects = $state<Project[]>([]);
  let isLoading = $state(true);
  let error = $state("");
  let searchTerm = $state("");
  let isCreateDialogOpen = $state(false);
  let isEditDialogOpen = $state(false);
  let selectedProject = $state<Project | null>(null);

  // Form states
  let createForm = $state({
    name: "",
    description: "",
  });

  let editForm = $state({
    name: "",
    description: "",
  });

  onMount(async () => {
    await loadProjects();
  });

  async function loadProjects() {
    isLoading = true;
    error = "";

    try {
      projects = await projectsAPI.listProjects();
      filteredProjects = projects;
    } catch (err) {
      error = err instanceof Error ? err.message : "Failed to load projects";
    } finally {
      isLoading = false;
    }
  }

  function filterProjects() {
    if (!searchTerm) {
      filteredProjects = projects;
      return;
    }

    const term = searchTerm.toLowerCase();
    filteredProjects = projects.filter(
      (project) =>
        project.name.toLowerCase().includes(term) ||
        project.description?.toLowerCase().includes(term) ||
        project.user_email.toLowerCase().includes(term)
    );
  }

  $effect(() => {
    filterProjects();
  });

  function openCreateDialog() {
    createForm = { name: "", description: "" };
    isCreateDialogOpen = true;
  }

  function openEditDialog(project: Project) {
    selectedProject = project;
    editForm = {
      name: project.name,
      description: project.description || "",
    };
    isEditDialogOpen = true;
  }

  async function handleCreateProject() {
    try {
      await projectsAPI.createProject(createForm);
      await loadProjects();
      isCreateDialogOpen = false;
    } catch (err) {
      error = err instanceof Error ? err.message : "Failed to create project";
    }
  }

  async function handleUpdateProject() {
    if (!selectedProject?.id) return;

    try {
      await projectsAPI.updateProject(selectedProject.id, editForm);
      await loadProjects();
      isEditDialogOpen = false;
    } catch (err) {
      error = err instanceof Error ? err.message : "Failed to update project";
    }
  }

  async function handleDeleteProject(project: Project) {
    if (!project.id || !confirm(`Are you sure you want to delete "${project.name}"?`)) return;

    try {
      await projectsAPI.deleteProject(project.id);
      await loadProjects();
    } catch (err) {
      error = err instanceof Error ? err.message : "Failed to delete project";
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

  function getProjectStatus(createdAt: string): { label: string; variant: "default" | "secondary" | "outline" } {
    const createdDate = new Date(createdAt);
    const now = new Date();
    const daysDiff = Math.floor((now.getTime() - createdDate.getTime()) / (1000 * 60 * 60 * 24));

    if (daysDiff < 7) {
      return { label: "New", variant: "default" };
    } else if (daysDiff < 30) {
      return { label: "Recent", variant: "secondary" };
    } else {
      return { label: "Active", variant: "outline" };
    }
  }
</script>

<div class="space-y-6">
  <!-- Page Header -->
  <div class="flex items-center justify-between">
    <div>
      <h2 class="text-3xl font-bold tracking-tight">Projects Management</h2>
      <p class="text-muted-foreground">
        Manage all projects across the platform
      </p>
    </div>
    <Button onclick={openCreateDialog}>
      <Plus class="mr-2 h-4 w-4" />
      Create Project
    </Button>
  </div>

  <!-- Search Bar -->
  <Card.Root>
    <Card.Content class="p-4">
      <div class="relative">
        <Search class="absolute left-3 top-1/2 transform -translate-y-1/2 text-muted-foreground h-4 w-4" />
        <Input
          type="text"
          placeholder="Search projects by name, description, or owner..."
          class="pl-10"
          bind:value={searchTerm}
        />
      </div>
    </Card.Content>
  </Card.Root>

  <!-- Projects Grid -->
  <div class="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
    {#if isLoading}
      <div class="col-span-full flex items-center justify-center py-12">
        <LoadingSpinner size={32} text="Loading projects..." />
      </div>
    {:else if error}
      <div class="col-span-full">
        <Card.Root>
          <Card.Content class="py-8">
            <div class="text-center space-y-4">
              <p class="text-destructive">{error}</p>
              <Button onclick={loadProjects} variant="outline">
                Try Again
              </Button>
            </div>
          </Card.Content>
        </Card.Root>
      </div>
    {:else if filteredProjects.length === 0}
      <div class="col-span-full">
        <Card.Root>
          <Card.Content class="py-12">
            <div class="text-center space-y-4">
              <FolderOpen class="h-12 w-12 mx-auto text-muted-foreground opacity-50" />
              <div>
                <h3 class="font-medium">No projects found</h3>
                <p class="text-sm text-muted-foreground">
                  {searchTerm ? "Try adjusting your search terms" : "Get started by creating your first project"}
                </p>
              </div>
              {#if !searchTerm}
                <Button onclick={openCreateDialog}>
                  <Plus class="mr-2 h-4 w-4" />
                  Create Project
                </Button>
              {/if}
            </div>
          </Card.Content>
        </Card.Root>
      </div>
    {:else}
      {#each filteredProjects as project (project.id)}
        <Card.Root class="hover:shadow-lg transition-shadow">
          <Card.Header>
            <div class="flex items-start justify-between">
              <div class="flex items-center space-x-3">
                <div class="h-10 w-10 rounded-full bg-primary/10 flex items-center justify-center">
                  <span class="text-sm font-medium text-primary">
                    {getInitials(project.name)}
                  </span>
                </div>
                <div class="space-y-1">
                  <Card.Title class="text-lg">{project.name}</Card.Title>
                  {#if project.description}
                    <Card.Description class="line-clamp-2">
                      {project.description}
                    </Card.Description>
                  {/if}
                </div>
              </div>
              <Badge variant={getProjectStatus(project.created_at || "").variant}>
                {getProjectStatus(project.created_at || "").label}
              </Badge>
            </div>
          </Card.Header>
          <Card.Content>
            <div class="space-y-3">
              <div class="flex items-center space-x-2 text-sm text-muted-foreground">
                <User class="h-4 w-4" />
                <span>{project.user_email}</span>
              </div>
              <div class="flex items-center space-x-2 text-sm text-muted-foreground">
                <Calendar class="h-4 w-4" />
                <span>Created {project.created_at ? formatDate(project.created_at) : "N/A"}</span>
              </div>
              {#if project.updated_at && project.updated_at !== project.created_at}
                <div class="flex items-center space-x-2 text-sm text-muted-foreground">
                  <Clock class="h-4 w-4" />
                  <span>Updated {formatDate(project.updated_at)}</span>
                </div>
              {/if}
            </div>
          </Card.Content>
          <Card.Content class="pt-0">
            <div class="flex items-center justify-end space-x-2">
              <Button
                variant="ghost"
                size="sm"
                onclick={() => openEditDialog(project)}
              >
                <Edit class="h-4 w-4 mr-1" />
                Edit
              </Button>
              <Button
                variant="ghost"
                size="sm"
                onclick={() => handleDeleteProject(project)}
                class="text-red-600 hover:text-red-700"
              >
                <Trash2 class="h-4 w-4 mr-1" />
                Delete
              </Button>
            </div>
          </Card.Content>
        </Card.Root>
      {/each}
    {/if}
  </div>
</div>

<!-- Create Project Dialog -->
<Dialog.Root bind:open={isCreateDialogOpen}>
  <Dialog.Content>
    <Dialog.Header>
      <Dialog.Title>Create New Project</Dialog.Title>
      <Dialog.Description>
        Create a new project for a user
      </Dialog.Description>
    </Dialog.Header>
    <div class="space-y-4 py-4">
      <div class="space-y-2">
        <Label for="create-name">Project Name</Label>
        <Input
          id="create-name"
          type="text"
          placeholder="My Awesome Project"
          bind:value={createForm.name}
        />
      </div>
      <div class="space-y-2">
        <Label for="create-description">Description</Label>
        <Input
          id="create-description"
          type="text"
          placeholder="Project description (optional)"
          bind:value={createForm.description}
        />
      </div>
    </div>
    <Dialog.Footer>
      <Button variant="outline" onclick={() => (isCreateDialogOpen = false)}>
        Cancel
      </Button>
      <Button onclick={handleCreateProject}>Create Project</Button>
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>

<!-- Edit Project Dialog -->
<Dialog.Root bind:open={isEditDialogOpen}>
  <Dialog.Content>
    <Dialog.Header>
      <Dialog.Title>Edit Project</Dialog.Title>
      <Dialog.Description>
        Update project information
      </Dialog.Description>
    </Dialog.Header>
    <div class="space-y-4 py-4">
      <div class="space-y-2">
        <Label for="edit-name">Project Name</Label>
        <Input
          id="edit-name"
          type="text"
          bind:value={editForm.name}
        />
      </div>
      <div class="space-y-2">
        <Label for="edit-description">Description</Label>
        <Input
          id="edit-description"
          type="text"
          bind:value={editForm.description}
        />
      </div>
    </div>
    <Dialog.Footer>
      <Button variant="outline" onclick={() => (isEditDialogOpen = false)}>
        Cancel
      </Button>
      <Button onclick={handleUpdateProject}>Save Changes</Button>
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>