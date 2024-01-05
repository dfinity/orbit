import { AnonymousIdentity, Identity } from '@dfinity/agent';
import { Principal } from '@dfinity/principal';
import { defineStore } from 'pinia';
import { logger } from '~/core';
import { icAgent } from '~/core/ic-agent';
import { defaultHomeRoute, defaultLoginRoute, redirectToKey, router, services } from '~/ui/modules';
import { useSessionStore } from '~/ui/stores/session';

export interface AuthStoreState {
  initialized: boolean;
  userId: Principal | null;
  identity: Identity | null;
}

export const useAuthStore = defineStore('auth', {
  state: (): AuthStoreState => ({
    initialized: false,
    identity: null,
    userId: null,
  }),
  getters: {
    isAuthenticated(): boolean {
      return this.userId !== null;
    },
    principal(): string | undefined {
      return this.identity?.getPrincipal().toText();
    },
  },
  actions: {
    async initialize(): Promise<void> {
      try {
        if (this.initialized) {
          return;
        }
        const authService = services().auth;
        const controlPanelService = services().controlPanel;
        const sessionStore = useSessionStore();
        const cachedIdentity = await authService.identity();

        if (!cachedIdentity) {
          icAgent.get().replaceIdentity(new AnonymousIdentity());
          this.initialized = true;
          return;
        }

        icAgent.get().replaceIdentity(cachedIdentity);
        const user = await controlPanelService.getCurrentUser();

        // loads information about the authenticated user
        await sessionStore.load();

        this.identity = cachedIdentity;
        this.userId = user.id;
      } catch (error) {
        this.reset();

        logger.error(`Application failed to initialize the state`, { error });
      } finally {
        this.initialized = true;
      }
    },
    async signIn(): Promise<void> {
      const authService = services().auth;
      const sessionStore = useSessionStore();

      try {
        this.identity = await authService.login();
        icAgent.get().replaceIdentity(this.identity);

        const controlPanelService = services().controlPanel;
        const isRegistered = await controlPanelService.hasRegistration();

        if (isRegistered) {
          const user = await controlPanelService.getCurrentUser();

          // loads information about the authenticated user
          await sessionStore.load();

          this.userId = user.id;
          return;
        }

        const user = await controlPanelService.register({
          // a new user is created with an empty list of wallets, they can add them later
          wallet_id: [],
        });

        // loads information about the authenticated user
        await sessionStore.load();

        this.userId = user.id;
      } catch (error) {
        this.reset();
        throw error;
      }
    },
    async signOut(): Promise<void> {
      const authService = services().auth;

      await authService.logout();

      this.reset();
      this.redirectToLogin();
    },
    reset(): void {
      this.identity = null;
      this.userId = null;

      useSessionStore().reset();
      icAgent.get().invalidateIdentity();
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
