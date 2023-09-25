import { Principal } from '@dfinity/principal';
import { defineStore } from 'pinia';
import { logger } from '~/core';
import { Maybe } from '~/types';
import { i18n, services } from '~/ui/modules';
import { useSettingsStore } from '~/ui/stores';

export interface BankItem {
  name: Maybe<string>;
  canisterId: Principal;
}

export interface BankStoreState {
  loading: boolean;
  initialized: boolean;
  main: Maybe<Principal>;
  banks: BankItem[];
}

export const useBankStore = defineStore('bank', {
  state: (): BankStoreState => {
    return {
      loading: false,
      initialized: false,
      main: null,
      banks: [],
    };
  },
  getters: {
    hasBanks(): boolean {
      return !!this.banks.length;
    },
  },
  actions: {
    async init(): Promise<void> {
      if (this.initialized) {
        return;
      }

      await this.load().finally(() => {
        this.initialized = true;
      });
    },
    computedBankName(canisterId: Principal, notFoundName = '-'): string {
      const bankIdx = this.banks.findIndex(bank => bank.canisterId === canisterId);

      if (bankIdx === -1) {
        return notFoundName;
      }

      return this.banks[bankIdx].name ?? i18n.global.t('banks.bank_nr_title', { nr: bankIdx + 1 });
    },
    reset(): void {
      this.initialized = false;
      this.main = null;
      this.banks = [];
    },
    useBanks(banks: BankItem[]): void {
      this.banks = banks;
      if (
        this.main &&
        !banks.some(({ canisterId }) => canisterId.compareTo(this.main as Principal)) &&
        banks.length
      ) {
        this.main = banks[0].canisterId;
      }
    },
    async load(): Promise<void> {
      this.loading = true;
      const controlPanelService = services().controlPanel;
      const settings = useSettingsStore();
      await Promise.all([controlPanelService.getMainBank(), controlPanelService.listBanks()])
        .then(([mainBank, banks]) => {
          const main = mainBank ?? banks?.[0];
          const mainCanisterId = main?.canister_id ?? null;
          if (
            mainCanisterId &&
            banks.some(({ canister_id }) => canister_id.compareTo(mainCanisterId))
          ) {
            this.main = mainCanisterId;
          }
          this.banks = banks.map(bank => ({
            canisterId: bank.canister_id,
            name: bank.name?.[0] ?? null,
          }));
        })
        .catch(err => {
          logger.error(`Failed to load banks`, { err });
          settings.setNotification({
            show: true,
            type: 'error',
            message: i18n.global.t('banks.load_error'),
          });
        })
        .finally(() => {
          this.loading = false;
        });
    },
  },
});
