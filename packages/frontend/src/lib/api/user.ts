import { makeRequest } from "./request";

export interface User {
  name: string;
  email: string;
  password: string;
}

export interface UserSettings {
  fullname: string | null;
  email: string;
  provider: string;
}

export interface UserUpdateRequest {
  name: string | null;
  email: string;
  current_password?: string;
  new_password?: string;
}

export class UsersAPI {
  cookieHeader?: string;
  calledFrom?: string;

  constructor(cookieHeader?: string, calledFrom?: string) {
    this.cookieHeader = cookieHeader;
    this.calledFrom = calledFrom;
  }

  async getUserSettings(): Promise<UserSettings> {
    const response = await makeRequest(
      "/v1/user/settings",
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

  async updateUserSettings(updateRequest: UserUpdateRequest): Promise<void> {
    await makeRequest(
      "/v1/user/settings",
      {
        method: "PUT",
        body: JSON.stringify(updateRequest),
        headers: {
          Cookie: this.cookieHeader || "",
        },
      },
      this.calledFrom,
    );
  }
}

export const usersAPI = new UsersAPI();
