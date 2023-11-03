import { AnonymousIdentity, Identity } from '@dfinity/agent';
import { Principal } from '@dfinity/principal';
import { defineStore } from 'pinia';
import { logger } from '~/core';
import { icAgent } from '~/core/IcAgent';
import { Maybe } from '~/types';
import { defaultHomeRoute, defaultLoginRoute, redirectToKey, router, services } from '~/ui/modules';
import { useActiveBankStore, useBankStore } from '~/ui/stores';

export interface AuthStoreState {
  initialized: boolean;
  identity: Identity | null;
  _identities: string[];
  userId: string | null;
  userName: string | null;
}

export const useAuthStore = defineStore('auth', {
  state: (): AuthStoreState => ({
    initialized: false,
    identity: null,
    _identities: [],
    userId: null,
    userName: null,
  }),
  getters: {
    isAuthenticated(): boolean {
      return this.userId !== null;
    },
    principal(): string | undefined {
      return this.identity?.getPrincipal().toText();
    },
    identities(): Principal[] {
      return this._identities.map(identity => Principal.fromText(identity));
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
        const cachedIdentity = await authService.identity();

        if (!cachedIdentity) {
          icAgent.get().replaceIdentity(new AnonymousIdentity());
          this.initialized = true;
          return;
        }

        icAgent.get().replaceIdentity(cachedIdentity);
        const user = await controlPanelService.getCurrentUser();

        // loads information about the main bank and the list of banks for the user
        await this.initBanks();

        this.identity = cachedIdentity;
        this.userName = user.name.length ? user.name[0] : null;
        this.userId = user.id;
        this._identities = user.identities.map(identity => identity.identity.toText());
      } catch (error) {
        useBankStore().reset();
        this.resetIdentity();

        logger.error(`Application failed to initialize the state`, { error });
      } finally {
        this.initialized = true;
      }
    },
    async initBanks(): Promise<void> {
      const bankStore = useBankStore();
      // loads information about the main bank and the list of banks for the user
      await bankStore.init();
      if (bankStore.main !== null && !bankStore.main.isAnonymous()) {
        // this does not need to be awaited, it will be loaded in the background making the initial load faster
        await useActiveBankStore().load(bankStore.main);
      }
    },
    async signIn(): Promise<void> {
      const authService = services().auth;

      this.identity = await authService.login();
      icAgent.get().replaceIdentity(this.identity);

      try {
        const controlPanelService = services().controlPanel;
        const isRegistered = await controlPanelService.hasRegistration();

        if (isRegistered) {
          const user = await controlPanelService.getCurrentUser();
          // loads information about the main bank and the list of banks for the user
          await this.initBanks();

          this.userName = user.name.length ? user.name[0] : null;
          this.userId = user.id;
          this._identities = user.identities.map(identity => identity.identity.toText());
          return;
        }

        const user = await controlPanelService.registerWithSharedBank();

        // loads information about the main bank and the list of banks for the user
        await this.initBanks();

        this.userName = user.name.length ? user.name[0] : null;
        this.userId = user.id;
        this._identities = user.identities.map(identity => identity.identity.toText());
      } catch (error) {
        useBankStore().reset();
        this.resetIdentity();
        throw error;
      }
    },
    async signOut(): Promise<void> {
      const authService = services().auth;

      await authService.logout();
      this.resetIdentity();
      useBankStore().reset();
      this.redirectToLogin();
    },
    editUser(user: { name?: Maybe<string> }): void {
      if (user.name !== undefined) {
        this.userName = user.name;
      }
    },
    resetIdentity(): void {
      this.identity = null;
      this.userName = null;
      this.userId = null;
      this._identities = [];

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
