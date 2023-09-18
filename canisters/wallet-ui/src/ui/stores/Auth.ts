import { Identity } from '@dfinity/agent';
import { defineStore } from 'pinia';
import { defaultHomeRoute, defaultLoginRoute, redirectToKey, router, services } from '~/ui/modules';

export interface AuthStoreState {
  initialized: boolean;
  identity: Identity | null;
  username: string | null;
}

export const useAuthStore = defineStore('auth', {
  state: (): AuthStoreState => ({
    initialized: false,
    identity: null,
    username: null,
  }),
  getters: {
    isAuthenticated(): boolean {
      return this.identity !== null;
    },
    principal(): string | undefined {
      return this.identity?.getPrincipal().toText();
    },
  },
  actions: {
    async initialize(): Promise<void> {
      if (this.initialized) {
        return;
      }

      const authService = services().auth;

      this.identity = await authService.identity();
      this.initialized = true;
    },
    async signIn(): Promise<void> {
      const authService = services().auth;

      this.identity = await authService.login();
    },
    async signOut(): Promise<void> {
      const authService = services().auth;

      await authService.logout();
      this.resetIdentity();
      this.redirectToLogin();
    },
    resetIdentity(): void {
      this.identity = null;
      this.username = null;
    },
    redirectToLogin(): void {
      router.push({ name: defaultLoginRoute });
    },
    afterLoginRedirect(): void {
      const lastRequestedPage = window?.sessionStorage.getItem(redirectToKey);
      if (lastRequestedPage) {
        window?.sessionStorage.removeItem(redirectToKey);
        router.push(lastRequestedPage);
        return;
      }

      router.push({ name: defaultHomeRoute });
    },
  },
});
