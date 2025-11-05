import { auth } from "$lib/stores/auth";
import { makeRequest } from "./request";

export interface LoginRequest {
  email: string;
  password: string;
}

export interface SignupRequest {
  email: string;
  password: string;
}

export interface User {
  id: string;
  email: string;
  fullname?: string;
  avatar_url?: string;
  is_active: boolean;
  created_at: string;
  updated_at: string;
}

export interface AuthResponse {
  user: User;
}

class AuthAPI {
  async login(credentials: LoginRequest): Promise<void> {
    auth.setLoading(true);

    try {
      const response = await makeRequest("/oauth/email/login", {
        method: "POST",
        body: JSON.stringify(credentials),
      });

      // Store email in localStorage for session persistence
      localStorage.setItem("user_email", credentials.email);

      // Update auth store - fetch current user data
      await this.getCurrentUser();
    } catch (error) {
      auth.setLoading(false);
      throw error;
    }
  }

  async signup(userData: SignupRequest): Promise<void> {
    await makeRequest("/oauth/email/register", {
      method: "POST",
      body: JSON.stringify(userData),
    });
  }

  async logout(): Promise<void> {
    try {
      await makeRequest("/api/v1/auth/logout", {
        method: "DELETE",
      });
    } catch (error) {
      // Even if logout fails on server, clear local state
      console.error("Logout error:", error);
    } finally {
      localStorage.removeItem("user_email");
      auth.clearUser();
    }
  }

  async getCurrentUser(): Promise<User | null> {
    try {
      const response = await makeRequest("/api/v1/auth/current-user", {
        method: "GET",
      });

      if (response && response.data) {
        const user = response.data as User;
        auth.setUser({ 
          id: user.id,
          email: user.email,
          fullname: user.fullname,
          avatar_url: user.avatar_url,
          is_active: user.is_active,
          created_at: user.created_at,
          updated_at: user.updated_at
        });
        return user;
      }
      return null;
    } catch (error) {
      console.error("Failed to get current user:", error);
      return null;
    }
  }

  async checkSession(): Promise<User | null> {
    return await this.getCurrentUser();
  }

  // OAuth functionality
  async getGoogleAuthUrl(): Promise<string> {
    const response = await makeRequest("/oauth/google/get-url", {
      method: "GET",
    });
    return response?.data || "";
  }

  async handleGoogleCallback(code: string): Promise<void> {
    auth.setLoading(true);
    try {
      await makeRequest(`/oauth/google/callback?code=${encodeURIComponent(code)}`, {
        method: "GET",
      });
      
      // Fetch current user after successful OAuth
      await this.getCurrentUser();
    } catch (error) {
      auth.setLoading(false);
      throw error;
    }
  }

  async refreshToken(): Promise<void> {
    try {
      await makeRequest("/oauth/refresh-token", {
        method: "GET",
      });
    } catch (error) {
      console.error("Token refresh failed:", error);
      throw error;
    }
  }
}

export const authAPI = new AuthAPI();
