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
      set({ token, user, isAuthenticated: true });
    },
    logout: () => {
      localStorage.removeItem('token');
      set({ token: null, user: null, isAuthenticated: false });
    },
    init: () => {
      const token = localStorage.getItem('token');
      if (token) {
        set({ token, user: null, isAuthenticated: true });
      }
    },
    setUser: (user: UserInfo) => {
      update((state) => ({ ...state, user }));
    },
  };
}

export const auth = createAuthStore();
