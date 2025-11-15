import { makeRequest } from "./request";

export interface Project {
  id?: string;
  user_email: string;
  name: string;
  description?: string;
  created_at?: string;
  updated_at?: string;
}

export interface CreateProject {
  name: string;
  description?: string;
}

export interface UpdateProject {
  name?: string;
  description?: string;
}

export class ProjectsAPI {
  cookieHeader?: string;
  calledFrom?: string;

  constructor(cookieHeader?: string, calledFrom?: string) {
    this.cookieHeader = cookieHeader;
    this.calledFrom = calledFrom;
  }

  async createProject(project: CreateProject): Promise<void> {
    await makeRequest(
      "/v1/projects",
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

  async listProjects(): Promise<Project[]> {
    const response = await makeRequest(
      "/v1/projects",
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

  async getProject(id: string): Promise<Project> {
    const response = await makeRequest(
      `/v1/projects/${id}`,
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

  async updateProject(id: string, update: UpdateProject): Promise<Project> {
    const response = await makeRequest(
      `/v1/projects/${id}`,
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

  async deleteProject(id: string): Promise<void> {
    await makeRequest(
      `/v1/projects/${id}`,
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

export const projectsAPI = new ProjectsAPI();
