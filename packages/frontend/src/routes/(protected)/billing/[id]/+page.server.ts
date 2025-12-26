import { error, redirect } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";
import { BillingAPI } from "$lib/api/billing";

export const load: PageServerLoad = async ({ locals, params, cookies }) => {
  try {
    const cookieHeader = cookies
      .getAll()
      .map((cookie) => `${cookie.name}=${cookie.value}`)
      .join("; ");

    if (!locals.user) {
      return redirect(302, "/login");
    }

    const billingId = params.id;
    const billingAPI = new BillingAPI(cookieHeader);
    const billing_plan = await billingAPI.getBillingPlan(billingId);
    if (billing_plan && billing_plan.price_id) {
      const session_url = await billingAPI.checkoutSession(
        billing_plan.price_id,
      );
      if (session_url) {
        return redirect(302, session_url);
      }
    }
  } catch (err) {
    console.error("Error loading checkout:", err);

    // If it's a SvelteKit error, re-throw it
    if (err && typeof err === "object" && "status" in err) {
      throw err;
    }

    // Otherwise, throw a generic error
    throw error(500, "Failed to load checkout");
  }
};
