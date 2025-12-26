import { makeRequest } from "./request";

export interface Plan {
  id?: string;
  name: string;
  description?: string;
  price: number;
  created_at?: string;
  updated_at?: string;
}

export interface CreatePlan {
  name: string;
  description?: string;
  price: number;
}

export interface UpdatePlan {
  name?: string;
  description?: string;
  price?: number;
}

export class AdminAPI {
  cookieHeader?: string;
  calledFrom?: string;

  constructor(cookieHeader?: string, calledFrom?: string) {
    this.cookieHeader = cookieHeader;
    this.calledFrom = calledFrom;
  }

  async createPlan(project: CreatePlan): Promise<void> {
    await makeRequest(
      "/v1/admin/billing",
      {
        method: "POST",
        body: JSON.stringify(project),
        headers: {
          Cookie: this.cookieHeader || "",
        },
      },
      this.calledFrom,
    );
  }

  async listPlans(): Promise<Plan[]> {
    const response = await makeRequest(
      "/v1/admin/billing",
      {
        method: "GET",
        headers: {
          Cookie: this.cookieHeader || "",
        },
      },
      this.calledFrom,
    );
    return response.data || [];
  }

  async getPlan(id: string): Promise<Plan> {
    const response = await makeRequest(
      `/v1/admin/billing/${id}`,
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

  async updatePlan(id: string, update: UpdatePlan): Promise<Plan> {
    const response = await makeRequest(
      `/v1/admin/billing/${id}`,
      {
        method: "PUT",
        body: JSON.stringify(update),
        headers: {
          Cookie: this.cookieHeader || "",
        },
      },
      this.calledFrom,
    );
    return response.data;
  }

  async deletePlan(id: string): Promise<void> {
    await makeRequest(
      `/v1/admin/billing/${id}`,
      {
        method: "DELETE",
        headers: {
          Cookie: this.cookieHeader || "",
        },
      },
      this.calledFrom,
    );
  }
}

export const adminAPI = new AdminAPI();
