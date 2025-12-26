import { makeRequest } from "./request";

export interface BillingPlan {
  id?: string;
  name: string;
  description?: string;
  price_id: string;
}

export interface CheckoutSessionRequest {
  price_id: string;
}

export class BillingAPI {
  cookieHeader?: string;
  calledFrom?: string;

  constructor(cookieHeader?: string, calledFrom?: string) {
    this.cookieHeader = cookieHeader;
    this.calledFrom = calledFrom;
  }

  async getAllBillingPlans(): Promise<BillingPlan[]> {
    const response = await makeRequest(
      `/v1/billing`,
      {
        method: "GET",
        headers: {
          Cookie: this.cookieHeader || "",
        },
      },
      this.calledFrom,
    );
    return response.data;
  }

  async getBillingPlan(id: string): Promise<BillingPlan> {
    const response = await makeRequest(
      `/v1/billing/${id}`,
      {
        method: "GET",
        headers: {
          Cookie: this.cookieHeader || "",
        },
      },
      this.calledFrom,
    );
    return response.data;
  }

  async checkoutSession(price_id: string): Promise<string> {
    const response = await makeRequest(
      `/v1/billing/checkout`,
      {
        method: "POST",
        body: JSON.stringify({ price_id }),
        headers: {
          Cookie: this.cookieHeader || "",
        },
      },
      this.calledFrom,
    );
    return response.data.session_url;
  }
}

export const billingAPI = new BillingAPI();
