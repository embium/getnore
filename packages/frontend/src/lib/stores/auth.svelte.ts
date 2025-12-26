import { browser } from '$app/environment';
import { authAPI, type User } from '$lib/api/auth';

type AuthState = {
  isAuthenticated: boolean;
  isLoading: boolean;
  user: User | null;
  lastCheckedAt?: number;
};

class AuthStore {
  public store: AuthState;
  private monitorId: ReturnType<typeof setInterval> | null = null;

  constructor() {
    this.store = $state({
      isAuthenticated: false,
      isLoading: false,
      user: null,
    });
  }

  init = async () => {
    if (!browser) return;
    this.setLoading(true);
    try {
      const user = await authAPI.getCurrentUser();
      if (user) {
        this.setUser(user);
        // Start periodic monitor once authenticated
        this.startMonitor();
      } else {
        this.clearUser();
      }
    } catch (err) {
      // Attempt refresh if access expired
      try {
        await authAPI.refreshToken();
        const user = await authAPI.getCurrentUser();
        if (user) {
          this.setUser(user);
          this.startMonitor();
        } else {
          this.clearUser();
        }
      } catch {
        this.clearUser();
      }
    } finally {
      this.setLoading(false);
    }
  };

  setUser = (user: User) => {
    this.store = {
      ...this.store,
      user,
      isAuthenticated: true,
      lastCheckedAt: Date.now(),
    };
  };

  clearUser = () => {
    this.stopMonitor();
    this.store = { isAuthenticated: false, isLoading: false, user: null };
  };

  setLoading = (loading: boolean) => {
    this.store = { ...this.store, isLoading: loading };
  };

  checkSession = async () => {
    if (!browser) return;
    try {
      const user = await authAPI.getCurrentUser();
      if (user) {
        this.setUser(user);
        return true;
      }
      // If unauthorized, try refresh then fetch
      await authAPI.refreshToken();
      const refreshedUser = await authAPI.getCurrentUser();
      if (refreshedUser) {
        this.setUser(refreshedUser);
        return true;
      }
    } catch (err) {
      // swallow; fallthrough to clear
    }
    this.clearUser();
    return false;
  };

  startMonitor = (intervalMs: number = 5 * 60 * 1000) => {
    if (!browser) return;
    if (this.monitorId) return; // already running
    this.monitorId = setInterval(async () => {
      await this.checkSession();
    }, intervalMs);
  };

  stopMonitor = () => {
    if (this.monitorId) {
      clearInterval(this.monitorId);
      this.monitorId = null;
    }
  };
}

export const auth = new AuthStore();
