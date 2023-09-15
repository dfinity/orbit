import { defineStore } from 'pinia';
import { Identity } from '@dfinity/agent';
import { router, services } from '~/ui/modules';


export interface AuthStoreState {
  identity: Identity | null;
  username: string | null;
  identityProvider: {
    domain: string;
  }
}

export const useAuthStore = defineStore('auth', {
  state: (): AuthStoreState => ({
    identity: null,
    username: null,
    identityProvider: {
      domain: 'https://identity.ic0.app',
    },
  }),
  getters: {
    isAuthenticated(): boolean {
      return this.identity !== null;
    },
    principal(): string | undefined {
      return this.identity?.getPrincipal().toText()
    }
  },
  actions: {
    async signIn(): Promise<void> {
      const authService = services().auth;

      await authService.login();
    },
    async signOut(): Promise<void> {
      const authService = services().auth;

      await authService.logout();
      this.resetIdentity();
      router.push({ name: 'login' });
    },
    resetIdentity(): void {
      this.identity = null;
      this.username = null;
    }
  },
});
