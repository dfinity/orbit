import { defineStore } from 'pinia';
import { logger } from '~/core/logger.core';
import { Account, Proposal, UUID } from '~/generated/wallet/wallet.did';
import { useWalletStore } from '~/stores/wallet.store';
import { FormValidationRules } from '~/types/helper.types';
import { requiredRule, validTokenAmount } from '~/utils/form.utils';
import { amountToBigInt, formatBalance } from '~/utils/helper.utils';

export interface TransferForm {
  accountId: UUID | null;
  amount: string | null;
  to: string | null;
}

export interface TransferFormValidationRules {
  accountId: FormValidationRules;
  amount: FormValidationRules;
  to: FormValidationRules;
}

export interface TransferFormStoreState {
  loading: boolean;
  isValid: boolean;
  fixedAccount: boolean;
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
  fixedAccount: false,
  alert: {
    show: false,
    type: 'success',
    message: null,
  },
  form: {
    accountId: null,
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
    selectedAccount(state): Account | null {
      const activeWallet = useWalletStore();

      return (
        activeWallet.accounts.items.find(account => account.id === state.form.accountId) ?? null
      );
    },
    validationRules(): TransferFormValidationRules {
      return {
        accountId: [requiredRule],
        to: [requiredRule],
        amount: [requiredRule, v => validTokenAmount(v, this.selectedAccount?.decimals ?? 0)],
      };
    },
    accounts(state): Array<{ value: string; title: string; balance: string }> {
      const activeWallet = useWalletStore();

      return activeWallet.accounts.items
        .filter(account => {
          if (!state.fixedAccount) {
            return true;
          }

          return account.id === state.form.accountId;
        })
        .map(account => {
          const balance = account.balance?.[0]
            ? `${formatBalance(account.balance[0].balance, account.balance[0].decimals)}`
            : '-';

          return {
            value: account.id,
            title: `${account.name}: ${account.id}`,
            balance: `${account.symbol}: ${balance}`,
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
    load(accountId?: UUID): void {
      this.reset();

      if (accountId) {
        this.form.accountId = accountId;
        this.fixedAccount = true;
      }
    },
    clearAlert(): void {
      this.alert = {
        show: false,
        type: 'success',
        message: null,
      };
    },
    async save(): Promise<Proposal | false> {
      try {
        if (!this.isValid) {
          return false;
        }
        this.clearAlert();
        this.loading = true;

        const wallet = useWalletStore().service;

        return await wallet.createProposal({
          title: [],
          summary: [],
          operation: {
            Transfer: {
              from_account_id: `${this.form.accountId}`,
              to: `${this.form.to}`,
              amount: amountToBigInt(this.form.amount ?? '', this.selectedAccount?.decimals ?? 0),
              fee: [],
              metadata: [],
              network: [],
            },
          },
          execution_plan: [],
        });
      } catch (err) {
        logger.error('Failed to send account', { err });

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
