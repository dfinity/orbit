import { defineStore } from 'pinia';
import { amountToBigInt, formatBalance, logger } from '~/core';
import { Transfer, Wallet, WalletId } from '~/generated/bank/bank.did';
import { useActiveBankStore } from '~/ui/stores';
import { FormValidationRules } from '~/ui/types';
import { requiredRule, validTokenAmount } from '~/ui/utils';

export interface TransferForm {
  walletId: WalletId | null;
  amount: string | null;
  to: string | null;
}

export interface TransferFormValidationRules {
  walletId: FormValidationRules;
  amount: FormValidationRules;
  to: FormValidationRules;
}

export interface TransferFormStoreState {
  loading: boolean;
  isValid: boolean;
  fixedWallet: boolean;
  alert: {
    show: boolean;
    type: 'success' | 'error' | 'warning' | 'info';
    message: string | null;
  };
  unchangedVersion: string | null;
  form: TransferForm;
}

const initialState: TransferFormStoreState = {
  loading: false,
  isValid: true,
  unchangedVersion: null,
  fixedWallet: false,
  alert: {
    show: false,
    type: 'success',
    message: null,
  },
  form: {
    walletId: null,
    amount: null,
    to: null,
  },
};

export const useTransferFormStore = defineStore('transferForm', {
  state: (): TransferFormStoreState => {
    return JSON.parse(JSON.stringify(initialState));
  },
  getters: {
    hasChanges(state): boolean {
      return JSON.stringify(state.form) !== state.unchangedVersion;
    },
    canSave(state): boolean {
      return state.isValid && this.hasChanges;
    },
    selectedWallet(state): Wallet | null {
      const activeBank = useActiveBankStore();

      return activeBank.wallets.items.find(wallet => wallet.id === state.form.walletId) ?? null;
    },
    validationRules(): TransferFormValidationRules {
      return {
        walletId: [requiredRule],
        to: [requiredRule],
        amount: [requiredRule, v => validTokenAmount(v, this.selectedWallet?.decimals ?? 0)],
      };
    },
    wallets(state): Array<{ value: string; title: string; balance: string }> {
      const activeBank = useActiveBankStore();

      return activeBank.wallets.items
        .filter(wallet => {
          if (!state.fixedWallet) {
            return true;
          }

          return wallet.id === state.form.walletId;
        })
        .map(wallet => {
          const balance = wallet.balance?.[0]
            ? `${formatBalance(wallet.balance[0].balance, wallet.balance[0].decimals)}`
            : '-';

          return {
            value: wallet.id,
            title: `${wallet.name}: ${wallet.id}`,
            balance: `${wallet.symbol}: ${balance}`,
          };
        });
    },
  },
  actions: {
    reset(): void {
      const reset = JSON.parse(JSON.stringify(initialState));
      this.loading = reset.loading;
      this.isValid = reset.isValid;
      this.form = reset.form;
      this.alert = reset.alert;
    },
    load(walletId?: WalletId): void {
      this.reset();

      if (walletId) {
        this.form.walletId = walletId;
        this.fixedWallet = true;
      }
    },
    clearAlert(): void {
      this.alert = {
        show: false,
        type: 'success',
        message: null,
      };
    },
    async save(): Promise<Transfer | false> {
      try {
        if (!this.isValid) {
          return false;
        }
        this.clearAlert();
        this.loading = true;

        const bank = useActiveBankStore().service;
        return await bank.createTransfer({
          from_wallet_id: `${this.form.walletId}`,
          to: `${this.form.to}`,
          amount: amountToBigInt(this.form.amount ?? '', this.selectedWallet?.decimals ?? 0),
          fee: [],
          execution_plan: [],
          expiration_dt: [],
          metadata: [],
          network: [],
        });
      } catch (err) {
        logger.error('Failed to send wallet', { err });

        const e = err as Error;
        this.alert = {
          show: true,
          type: 'error',
          message: e.message,
        };
      } finally {
        this.loading = false;
      }

      return false;
    },
  },
});
