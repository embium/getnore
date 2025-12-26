<script lang="ts">
  import { Button } from "$lib/components/ui/button";
  import * as Card from "$lib/components/ui/card";
  import { Input } from "$lib/components/ui/input";
  import { Label } from "$lib/components/ui/label";
  import { Separator } from "$lib/components/ui/separator";
  import {
    Settings,
    Save,
    RefreshCw,
    Shield,
    Bell,
    Mail,
    Database,
    AlertTriangle,
    DollarSign,
  } from "lucide-svelte";

  // Mock settings data
  let settings = $state({
    siteName: "SaaS Boilerplate",
    siteDescription: "A modern SaaS application built with SvelteKit",
    supportEmail: "support@example.com",
    adminEmail: "admin@example.com",
    enableRegistration: true,
    requireEmailVerification: true,
    enableBilling: true,
    maintenanceMode: false,
  });

  let isSaving = $state(false);
  let saveMessage = $state("");
  let showSaveSuccess = $state(false);

  async function handleSaveSettings() {
    isSaving = true;
    saveMessage = "";
    showSaveSuccess = false;

    try {
      // Simulate API call
      await new Promise(resolve => setTimeout(resolve, 1000));
      showSaveSuccess = true;
      saveMessage = "Settings saved successfully!";
      
      // Hide success message after 3 seconds
      setTimeout(() => {
        showSaveSuccess = false;
        saveMessage = "";
      }, 3000);
    } catch (error) {
      saveMessage = "Failed to save settings";
    } finally {
      isSaving = false;
    }
  }

  function handleResetSettings() {
    if (confirm("Are you sure you want to reset all settings to defaults?")) {
      settings = {
        siteName: "SaaS Boilerplate",
        siteDescription: "A modern SaaS application built with SvelteKit",
        supportEmail: "support@example.com",
        adminEmail: "admin@example.com",
        enableRegistration: true,
        requireEmailVerification: true,
        enableBilling: true,
        maintenanceMode: false,
      };
      saveMessage = "Settings reset to defaults";
      setTimeout(() => saveMessage = "", 3000);
    }
  }
</script>

<div class="space-y-6">
  <!-- Page Header -->
  <div>
    <h2 class="text-3xl font-bold tracking-tight">Admin Settings</h2>
    <p class="text-muted-foreground">
      Configure your application settings and preferences
    </p>
  </div>

  <!-- Save Status -->
  {#if saveMessage}
    <Card.Root class={showSaveSuccess ? "border-green-200 bg-green-50" : "border-red-200 bg-red-50"}>
      <Card.Content class="p-4">
        <div class="flex items-center space-x-2">
          {#if showSaveSuccess}
            <div class="h-2 w-2 bg-green-500 rounded-full"></div>
          {:else}
            <div class="h-2 w-2 bg-red-500 rounded-full"></div>
          {/if}
          <p class="text-sm font-medium">{saveMessage}</p>
        </div>
      </Card.Content>
    </Card.Root>
  {/if}

  <!-- General Settings -->
  <Card.Root>
    <Card.Header>
      <div class="flex items-center space-x-2">
        <Settings class="h-5 w-5" />
        <Card.Title>General Settings</Card.Title>
      </div>
      <Card.Description>
        Basic application configuration
      </Card.Description>
    </Card.Header>
    <Card.Content class="space-y-4">
      <div class="grid gap-4 md:grid-cols-2">
        <div class="space-y-2">
          <Label for="site-name">Site Name</Label>
          <Input
            id="site-name"
            type="text"
            bind:value={settings.siteName}
            placeholder="Your SaaS Name"
          />
        </div>
        <div class="space-y-2">
          <Label for="support-email">Support Email</Label>
          <Input
            id="support-email"
            type="email"
            bind:value={settings.supportEmail}
            placeholder="support@example.com"
          />
        </div>
      </div>
      <div class="space-y-2">
        <Label for="site-description">Site Description</Label>
        <Input
          id="site-description"
          type="text"
          bind:value={settings.siteDescription}
          placeholder="A brief description of your application"
        />
      </div>
      <div class="space-y-2">
        <Label for="admin-email">Admin Email</Label>
        <Input
          id="admin-email"
          type="email"
          bind:value={settings.adminEmail}
          placeholder="admin@example.com"
        />
      </div>
    </Card.Content>
  </Card.Root>

  <!-- User Management Settings -->
  <Card.Root>
    <Card.Header>
      <div class="flex items-center space-x-2">
        <Shield class="h-5 w-5" />
        <Card.Title>User Management</Card.Title>
      </div>
      <Card.Description>
        Control user registration and authentication
      </Card.Description>
    </Card.Header>
    <Card.Content class="space-y-4">
      <div class="flex items-center justify-between p-4 border rounded-lg">
        <div class="space-y-1">
          <Label for="enable-registration" class="text-base">Enable Registration</Label>
          <p class="text-sm text-muted-foreground">
            Allow new users to create accounts
          </p>
        </div>
        <input
          id="enable-registration"
          type="checkbox"
          bind:checked={settings.enableRegistration}
          class="rounded border-gray-300"
        />
      </div>
      <div class="flex items-center justify-between p-4 border rounded-lg">
        <div class="space-y-1">
          <Label for="require-email-verification" class="text-base">Require Email Verification</Label>
          <p class="text-sm text-muted-foreground">
            Users must verify their email address before accessing the application
          </p>
        </div>
        <input
          id="require-email-verification"
          type="checkbox"
          bind:checked={settings.requireEmailVerification}
          class="rounded border-gray-300"
        />
      </div>
    </Card.Content>
  </Card.Root>

  <!-- Billing Settings -->
  <Card.Root>
    <Card.Header>
      <div class="flex items-center space-x-2">
        <DollarSign class="h-5 w-5" />
        <Card.Title>Billing & Payments</Card.Title>
      </div>
      <Card.Description>
        Configure payment and subscription settings
      </Card.Description>
    </Card.Header>
    <Card.Content class="space-y-4">
      <div class="flex items-center justify-between p-4 border rounded-lg">
        <div class="space-y-1">
          <Label for="enable-billing" class="text-base">Enable Billing</Label>
          <p class="text-sm text-muted-foreground">
            Allow users to subscribe to paid plans
          </p>
        </div>
        <input
          id="enable-billing"
          type="checkbox"
          bind:checked={settings.enableBilling}
          class="rounded border-gray-300"
        />
      </div>
    </Card.Content>
  </Card.Root>

  <!-- System Settings -->
  <Card.Root>
    <Card.Header>
      <div class="flex items-center space-x-2">
        <Database class="h-5 w-5" />
        <Card.Title>System Settings</Card.Title>
      </div>
      <Card.Description>
        Advanced system configuration
      </Card.Description>
    </Card.Header>
    <Card.Content class="space-y-4">
      <div class="flex items-center justify-between p-4 border rounded-lg bg-yellow-50 border-yellow-200">
        <div class="space-y-1">
          <div class="flex items-center space-x-2">
            <AlertTriangle class="h-4 w-4 text-yellow-600" />
            <Label for="maintenance-mode" class="text-base text-yellow-800">Maintenance Mode</Label>
          </div>
          <p class="text-sm text-yellow-700">
            Enable maintenance mode to restrict access to the application
          </p>
        </div>
        <input
          id="maintenance-mode"
          type="checkbox"
          bind:checked={settings.maintenanceMode}
          class="rounded border-yellow-300"
        />
      </div>
    </Card.Content>
  </Card.Root>

  <!-- Action Buttons -->
  <div class="flex items-center justify-between">
    <Button
      variant="outline"
      onclick={handleResetSettings}
      class="text-red-600 hover:text-red-700"
    >
      <RefreshCw class="mr-2 h-4 w-4" />
      Reset to Defaults
    </Button>
    <Button onclick={handleSaveSettings} disabled={isSaving}>
      {#if isSaving}
        <RefreshCw class="mr-2 h-4 w-4 animate-spin" />
        Saving...
      {:else}
        <Save class="mr-2 h-4 w-4" />
        Save Settings
      {/if}
    </Button>
  </div>
</div>