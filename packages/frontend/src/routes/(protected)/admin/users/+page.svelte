<script lang="ts">
  import { onMount } from "svelte";
  import { adminUsersAPI, type AdminUser } from "$lib/api/admin-users";
  import { Button } from "$lib/components/ui/button";
  import * as Card from "$lib/components/ui/card";
  import * as Dialog from "$lib/components/ui/dialog";
  import { Input } from "$lib/components/ui/input";
  import { Label } from "$lib/components/ui/label";
  import { Badge } from "$lib/components/ui/badge";
  import LoadingSpinner from "$lib/components/LoadingSpinner.svelte";
  import {
    Users,
    Search,
    Plus,
    Edit,
    Trash2,
    MoreVertical,
    Mail,
    Calendar,
    Shield,
    UserCheck,
    UserX,
  } from "lucide-svelte";

  let users = $state<AdminUser[]>([]);
  let filteredUsers = $state<AdminUser[]>([]);
  let isLoading = $state(true);
  let error = $state("");
  let searchTerm = $state("");
  let isCreateDialogOpen = $state(false);
  let isEditDialogOpen = $state(false);
  let selectedUser = $state<AdminUser | null>(null);

  // Form states
  let createForm = $state({
    email: "",
    name: "",
    role: "user",
  });

  let editForm = $state({
    name: "",
    role: "",
    is_active: true,
  });

  onMount(async () => {
    await loadUsers();
  });

  async function loadUsers() {
    isLoading = true;
    error = "";

    try {
      // Mock data since admin users endpoint doesn't exist yet
      users = [
        {
          id: "1",
          email: "admin@example.com",
          name: "Admin User",
          role: "admin",
          created_at: "2024-01-15T10:00:00Z",
          updated_at: "2024-01-15T10:00:00Z",
          last_login: "2024-01-16T14:30:00Z",
          is_active: true,
        },
        {
          id: "2",
          email: "john.doe@example.com",
          name: "John Doe",
          role: "user",
          created_at: "2024-01-10T08:00:00Z",
          updated_at: "2024-01-14T16:45:00Z",
          last_login: "2024-01-15T09:20:00Z",
          is_active: true,
        },
        {
          id: "3",
          email: "jane.smith@example.com",
          name: "Jane Smith",
          role: "user",
          created_at: "2024-01-08T14:20:00Z",
          updated_at: "2024-01-12T11:30:00Z",
          last_login: "2024-01-14T18:15:00Z",
          is_active: false,
        },
      ];
      filteredUsers = users;
    } catch (err) {
      error = err instanceof Error ? err.message : "Failed to load users";
    } finally {
      isLoading = false;
    }
  }

  function filterUsers() {
    if (!searchTerm) {
      filteredUsers = users;
      return;
    }

    const term = searchTerm.toLowerCase();
    filteredUsers = users.filter(
      (user) =>
        user.name.toLowerCase().includes(term) ||
        user.email.toLowerCase().includes(term) ||
        user.role.toLowerCase().includes(term)
    );
  }

  $effect(() => {
    filterUsers();
  });

  function openCreateDialog() {
    createForm = { email: "", name: "", role: "user" };
    isCreateDialogOpen = true;
  }

  function openEditDialog(user: AdminUser) {
    selectedUser = user;
    editForm = {
      name: user.name,
      role: user.role,
      is_active: user.is_active,
    };
    isEditDialogOpen = true;
  }

  async function handleCreateUser() {
    try {
      await adminUsersAPI.createUser(createForm);
      await loadUsers();
      isCreateDialogOpen = false;
    } catch (err) {
      error = err instanceof Error ? err.message : "Failed to create user";
    }
  }

  async function handleUpdateUser() {
    if (!selectedUser) return;

    try {
      await adminUsersAPI.updateUser(selectedUser.id, editForm);
      await loadUsers();
      isEditDialogOpen = false;
    } catch (err) {
      error = err instanceof Error ? err.message : "Failed to update user";
    }
  }

  async function handleDeleteUser(user: AdminUser) {
    if (!confirm(`Are you sure you want to delete ${user.name}?`)) return;

    try {
      await adminUsersAPI.deleteUser(user.id);
      await loadUsers();
    } catch (err) {
      error = err instanceof Error ? err.message : "Failed to delete user";
    }
  }

  function formatDate(dateString: string): string {
    return new Date(dateString).toLocaleDateString("en-US", {
      year: "numeric",
      month: "short",
      day: "numeric",
    });
  }

  function formatDateTime(dateString: string): string {
    return new Date(dateString).toLocaleString("en-US", {
      year: "numeric",
      month: "short",
      day: "numeric",
      hour: "2-digit",
      minute: "2-digit",
    });
  }
</script>

<div class="space-y-6">
  <!-- Page Header -->
  <div class="flex items-center justify-between">
    <div>
      <h2 class="text-3xl font-bold tracking-tight">Users Management</h2>
      <p class="text-muted-foreground">
        Manage application users and their roles
      </p>
    </div>
    <Button onclick={openCreateDialog}>
      <Plus class="mr-2 h-4 w-4" />
      Add User
    </Button>
  </div>

  <!-- Search Bar -->
  <Card.Root>
    <Card.Content class="p-4">
      <div class="relative">
        <Search class="absolute left-3 top-1/2 transform -translate-y-1/2 text-muted-foreground h-4 w-4" />
        <Input
          type="text"
          placeholder="Search users by name, email, or role..."
          class="pl-10"
          bind:value={searchTerm}
        />
      </div>
    </Card.Content>
  </Card.Root>

  <!-- Users Table -->
  <Card.Root>
    <Card.Header>
      <div class="flex items-center justify-between">
        <div>
          <Card.Title>All Users</Card.Title>
          <Card.Description>
            {filteredUsers.length} user{filteredUsers.length !== 1 ? 's' : ''} found
          </Card.Description>
        </div>
        <Button variant="outline" size="sm" onclick={loadUsers}>
          <Users class="mr-2 h-4 w-4" />
          Refresh
        </Button>
      </div>
    </Card.Header>
    <Card.Content>
      {#if isLoading}
        <div class="flex items-center justify-center py-12">
          <LoadingSpinner size={32} text="Loading users..." />
        </div>
      {:else if error}
        <div class="text-center py-8 space-y-4">
          <p class="text-destructive">{error}</p>
          <Button onclick={loadUsers} variant="outline">
            Try Again
          </Button>
        </div>
      {:else if filteredUsers.length === 0}
        <div class="text-center py-8 space-y-4">
          <Users class="h-12 w-12 mx-auto text-muted-foreground opacity-50" />
          <div>
            <h3 class="font-medium">No users found</h3>
            <p class="text-sm text-muted-foreground">
              {searchTerm ? "Try adjusting your search terms" : "Get started by adding your first user"}
            </p>
          </div>
          {#if !searchTerm}
            <Button onclick={openCreateDialog}>
              <Plus class="mr-2 h-4 w-4" />
              Add User
            </Button>
          {/if}
        </div>
      {:else}
        <div class="overflow-x-auto">
          <table class="w-full">
            <thead>
              <tr class="border-b">
                <th class="text-left p-4 font-medium">User</th>
                <th class="text-left p-4 font-medium">Role</th>
                <th class="text-left p-4 font-medium">Status</th>
                <th class="text-left p-4 font-medium">Created</th>
                <th class="text-left p-4 font-medium">Last Login</th>
                <th class="text-right p-4 font-medium">Actions</th>
              </tr>
            </thead>
            <tbody>
              {#each filteredUsers as user (user.id)}
                <tr class="border-b hover:bg-muted/50 transition-colors">
                  <td class="p-4">
                    <div class="flex items-center space-x-3">
                      <div class="h-10 w-10 rounded-full bg-primary/10 flex items-center justify-center">
                        <span class="text-sm font-medium text-primary">
                          {user.name.split(" ").map(n => n[0]).join("").toUpperCase()}
                        </span>
                      </div>
                      <div>
                        <p class="font-medium">{user.name}</p>
                        <p class="text-sm text-muted-foreground">{user.email}</p>
                      </div>
                    </div>
                  </td>
                  <td class="p-4">
                    <Badge variant={user.role === "admin" ? "default" : "secondary"}>
                      {user.role}
                    </Badge>
                  </td>
                  <td class="p-4">
                    <div class="flex items-center space-x-2">
                      {#if user.is_active}
                        <UserCheck class="h-4 w-4 text-green-500" />
                        <span class="text-sm text-green-600">Active</span>
                      {:else}
                        <UserX class="h-4 w-4 text-red-500" />
                        <span class="text-sm text-red-600">Inactive</span>
                      {/if}
                    </div>
                  </td>
                  <td class="p-4">
                    <p class="text-sm">{formatDate(user.created_at)}</p>
                  </td>
                  <td class="p-4">
                    <p class="text-sm text-muted-foreground">
                      {user.last_login ? formatDateTime(user.last_login) : "Never"}
                    </p>
                  </td>
                  <td class="p-4 text-right">
                    <div class="flex items-center justify-end space-x-2">
                      <Button
                        variant="ghost"
                        size="sm"
                        onclick={() => openEditDialog(user)}
                      >
                        <Edit class="h-4 w-4" />
                      </Button>
                      <Button
                        variant="ghost"
                        size="sm"
                        onclick={() => handleDeleteUser(user)}
                        class="text-red-600 hover:text-red-700"
                      >
                        <Trash2 class="h-4 w-4" />
                      </Button>
                    </div>
                  </td>
                </tr>
              {/each}
            </tbody>
          </table>
        </div>
      {/if}
    </Card.Content>
  </Card.Root>
</div>

<!-- Create User Dialog -->
<Dialog.Root bind:open={isCreateDialogOpen}>
  <Dialog.Content>
    <Dialog.Header>
      <Dialog.Title>Create New User</Dialog.Title>
      <Dialog.Description>
        Add a new user to the application
      </Dialog.Description>
    </Dialog.Header>
    <div class="space-y-4 py-4">
      <div class="space-y-2">
        <Label for="create-email">Email</Label>
        <Input
          id="create-email"
          type="email"
          placeholder="user@example.com"
          bind:value={createForm.email}
        />
      </div>
      <div class="space-y-2">
        <Label for="create-name">Full Name</Label>
        <Input
          id="create-name"
          type="text"
          placeholder="John Doe"
          bind:value={createForm.name}
        />
      </div>
      <div class="space-y-2">
        <Label for="create-role">Role</Label>
        <select
          id="create-role"
          class="w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2"
          bind:value={createForm.role}
        >
          <option value="user">User</option>
          <option value="admin">Admin</option>
        </select>
      </div>
    </div>
    <Dialog.Footer>
      <Button variant="outline" onclick={() => (isCreateDialogOpen = false)}>
        Cancel
      </Button>
      <Button onclick={handleCreateUser}>Create User</Button>
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>

<!-- Edit User Dialog -->
<Dialog.Root bind:open={isEditDialogOpen}>
  <Dialog.Content>
    <Dialog.Header>
      <Dialog.Title>Edit User</Dialog.Title>
      <Dialog.Description>
        Update user information
      </Dialog.Description>
    </Dialog.Header>
    <div class="space-y-4 py-4">
      <div class="space-y-2">
        <Label for="edit-name">Full Name</Label>
        <Input
          id="edit-name"
          type="text"
          bind:value={editForm.name}
        />
      </div>
      <div class="space-y-2">
        <Label for="edit-role">Role</Label>
        <select
          id="edit-role"
          class="w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2"
          bind:value={editForm.role}
        >
          <option value="user">User</option>
          <option value="admin">Admin</option>
        </select>
      </div>
      <div class="flex items-center space-x-2">
        <input
          type="checkbox"
          id="edit-active"
          bind:checked={editForm.is_active}
          class="rounded border-gray-300"
        />
        <Label for="edit-active">Active</Label>
      </div>
    </div>
    <Dialog.Footer>
      <Button variant="outline" onclick={() => (isEditDialogOpen = false)}>
        Cancel
      </Button>
      <Button onclick={handleUpdateUser}>Save Changes</Button>
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>