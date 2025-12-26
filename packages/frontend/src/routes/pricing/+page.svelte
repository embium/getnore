<script lang="ts">
  import { Button } from "$lib/components/ui/button";
  import * as Card from "$lib/components/ui/card";
  import { Check } from "lucide-svelte";
  import { type BillingPlan } from "$lib/api/billing";

  interface PageLoadProps {
    data: {
      plans: BillingPlan[];
    };
  }

  let { data }: PageLoadProps = $props();
</script>

<section class="py-20 lg:py-32">
  <div class="container mx-auto px-4">
    <div class="text-center mb-16">
      <h2 class="text-3xl font-bold tracking-tight sm:text-4xl mb-4">
        Simple, transparent pricing
      </h2>
      <p class="text-xl text-muted-foreground max-w-2xl mx-auto">
        Choose the plan that's right for you. Upgrade or downgrade at any time.
      </p>
    </div>

    <div class="grid gap-8 lg:grid-cols-3 max-w-5xl mx-auto">
      {#each data.plans as plan (plan.name)}
        <Card.Root
          class="relative {plan.popular ? 'border-primary shadow-lg' : ''}"
        >
          {#if plan.popular}
            <div class="absolute -top-3 left-1/2 -translate-x-1/2">
              <div
                class="inline-flex items-center rounded-full bg-primary px-3 py-1 text-sm font-medium text-primary-foreground"
              >
                Most Popular
              </div>
            </div>
          {/if}

          <Card.Content class="p-8">
            <div class="text-center mb-8">
              <h3 class="text-2xl font-bold mb-2">{plan.name}</h3>
              <p class="text-muted-foreground mb-4">{plan.description}</p>
              <div class="flex items-baseline justify-center">
                <span class="text-4xl font-bold">{plan.price}</span>
                <span class="text-muted-foreground ml-1">/{plan.period}</span>
              </div>
            </div>

            <ul class="space-y-3 mb-8">
              {#each plan.features as feature (feature)}
                <li class="flex items-center">
                  <Check class="h-4 w-4 text-primary mr-3 shrink-0" />
                  <span class="text-sm">{feature}</span>
                </li>
              {/each}
            </ul>

            <Button
              class="w-full"
              variant={plan.popular ? "default" : "outline"}
              size="lg"
              href="/signup"
            >
              {plan.name === "Enterprise" ? "Contact Sales" : "Get Started"}
            </Button>
          </Card.Content>
        </Card.Root>
      {/each}
    </div>
  </div>
</section>
