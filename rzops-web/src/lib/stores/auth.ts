import { writable } from 'svelte/store';
import type { UserInfo } from '$lib/types/auth';

interface AuthState {
  token: string | null;
  user: UserInfo | null;
  isAuthenticated: boolean;
}

function createAuthStore() {
  const { subscribe, set, update } = writable<AuthState>({
    token: null,
    user: null,
    isAuthenticated: false,
  });

  return {
    subscribe,
    login: (token: string, user: UserInfo) => {
      localStorage.setItem('token', token);
      localStorage.setItem('user', JSON.stringify(user));
      set({ token, user, isAuthenticated: true });
    },
    logout: () => {
      localStorage.removeItem('token');
      localStorage.removeItem('user');
      set({ token: null, user: null, isAuthenticated: false });
    },
    init: () => {
      const token = localStorage.getItem('token');
      if (token) {
        let user: UserInfo | null = null;
        try {
          const raw = localStorage.getItem('user');
          user = raw ? (JSON.parse(raw) as UserInfo) : null;
        } catch {
          user = null;
        }
        set({ token, user, isAuthenticated: true });
      }
    },
    setUser: (user: UserInfo) => {
      localStorage.setItem('user', JSON.stringify(user));
      update((state) => ({ ...state, user }));
    },
  };
}

export const auth = createAuthStore();
