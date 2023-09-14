import { defineStore } from 'pinia';
import { Identity } from '@dfinity/agent';
import { services } from '~/ui/modules';

export interface AuthStoreState {
  identity: Identity | null;
  identityProvider: {
    domain: string;
  }
}

export const useAuthStore = defineStore('auth', {
  state: (): AuthStoreState => ({
    identity: null,
    identityProvider: {
      domain: 'https://identity.ic0.app',
    },
  }),
  getters: {
    isAuthenticated(): boolean {
      return this.identity !== null;
    },
  },
  actions: {
    async signIn(): Promise<void> {
      const authService = services().auth;

      await authService.login();
    },
    async signOut(): Promise<void> {
      const authService = services().auth;

      await authService.logout();
    },
  },
});
