import { error, redirect } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";
import { BillingAPI } from "$lib/api/billing";

export const load: PageServerLoad = async ({ locals, params, cookies }) => {
  try {
    const billingAPI = new BillingAPI();
    const billing_plan = await billingAPI.getAllBillingPlans();
    return { plans: billing_plan };
  } catch (err) {
    console.error("Error loading billing plans:", err);

    // If it's a SvelteKit error, re-throw it
    if (err && typeof err === "object" && "status" in err) {
      throw err;
    }

    // Otherwise, throw a generic error
    throw error(500, "Failed to load billing plans");
  }
};
