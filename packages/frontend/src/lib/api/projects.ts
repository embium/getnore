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

class ProjectsAPI {
  async createProject(project: CreateProject): Promise<void> {
    await makeRequest("/api/v1/projects", {
      method: "POST",
      body: JSON.stringify(project),
    });
  }

  async listProjects(): Promise<Project[]> {
    const response = await makeRequest("/api/v1/projects", {
      method: "GET",
    });
    return response.data || [];
  }

  async getProject(id: string): Promise<Project> {
    const response = await makeRequest(`/api/v1/projects/${id}`, {
      method: "GET",
    });
    return response.data;
  }

  async updateProject(id: string, update: UpdateProject): Promise<Project> {
    const response = await makeRequest(`/api/v1/projects/${id}`, {
      method: "PUT",
      body: JSON.stringify(update),
    });
    return response.data;
  }

  async deleteProject(id: string): Promise<void> {
    await makeRequest(`/api/v1/projects/${id}`, {
      method: "DELETE",
    });
  }
}

export const projectsAPI = new ProjectsAPI();
