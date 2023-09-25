import { AnonymousIdentity, Identity } from '@dfinity/agent';
import { defineStore } from 'pinia';
import { icAgent } from '~/core/IcAgent';
import { Maybe } from '~/types';
import { defaultHomeRoute, defaultLoginRoute, redirectToKey, router, services } from '~/ui/modules';
import { useBankStore } from '~/ui/stores';

export interface AuthStoreState {
  initialized: boolean;
  identity: Identity | null;
  accountId: string | null;
  accountName: string | null;
}

export const useAuthStore = defineStore('auth', {
  state: (): AuthStoreState => ({
    initialized: false,
    identity: null,
    accountId: null,
    accountName: null,
  }),
  getters: {
    isAuthenticated(): boolean {
      return this.accountId !== null;
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
        const cachedIdentity = await authService.identity();

        if (!cachedIdentity) {
          icAgent.get().replaceIdentity(new AnonymousIdentity());
          this.initialized = true;
          return;
        }

        icAgent.get().replaceIdentity(cachedIdentity);
        const account_details = await controlPanelService.get_account_details();

        if (!account_details) {
          throw new Error('Account not found');
        }

        // loads information about the main bank and the list of banks for the account
        await useBankStore().init();

        this.identity = cachedIdentity;
        this.accountName = account_details.name.length ? account_details.name[0] : null;
        this.accountId = account_details.id;
      } catch (error) {
        useBankStore().reset();
        this.resetIdentity();
        throw error;
      } finally {
        this.initialized = true;
      }
    },
    async signIn(): Promise<void> {
      const authService = services().auth;

      this.identity = await authService.login();
      icAgent.get().replaceIdentity(this.identity);

      try {
        const controlPanelService = services().controlPanel;
        const accountDetails = await controlPanelService.get_account_details();

        if (accountDetails) {
          // loads information about the main bank and the list of banks for the account
          await useBankStore().init();

          this.accountName = accountDetails.name.length ? accountDetails.name[0] : null;
          this.accountId = accountDetails.id;
          return;
        }

        const account = await controlPanelService.register_with_shared_bank();

        // loads information about the main bank and the list of banks for the account
        await useBankStore().init();

        this.accountName = account.name.length ? account.name[0] : null;
        this.accountId = account.id;
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
    editAccount(account: { name?: Maybe<string> }): void {
      if (account.name !== undefined) {
        this.accountName = account.name;
      }
    },
    resetIdentity(): void {
      this.identity = null;
      this.accountName = null;
      this.accountId = null;

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
