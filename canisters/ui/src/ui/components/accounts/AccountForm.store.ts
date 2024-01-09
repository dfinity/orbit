import { defineStore } from 'pinia';
import { logger } from '~/core';
import { UUID, Account, Proposal, AccountPolicies } from '~/generated/wallet/wallet.did';
import { i18n } from '~/ui/modules';
import { useWalletStore } from '~/ui/stores';
import { FormValidationRules } from '~/ui/types';
import { maxLengthRule, requiredRule, validPrincipalRule, validUuidV4Rule } from '~/ui/utils';

export interface AccountForm {
  name: string | null;
  owners: Array<UUID | null>;
  blockchain: string | null;
  blockchainStandard: string | null;
  policies: AccountPolicies;
}

export interface AccountFormStoreState {
  loading: boolean;
  isValid: boolean;
  multiCustody: boolean;
  alert: {
    show: boolean;
    type: 'success' | 'error' | 'warning' | 'info';
    message: string | null;
  };
  unchangedVersion: string | null;
  form: AccountForm;
}

export interface AccountFormValidationRules {
  name: FormValidationRules;
  blockchain: FormValidationRules;
  blockchainStandard: FormValidationRules;
  ownerUser: FormValidationRules;
  ownerIdentity: FormValidationRules;
}

const createFormId = (account?: Account): string => {
  if (account) {
    return `${account.id}`;
  }

  return 'new-' + Math.random().toString(8);
};

const initialStateForAccount = (account?: Account): AccountFormStoreState => {
  if (!account) {
    const activeWallet = useWalletStore();

    return {
      loading: false,
      isValid: true,
      unchangedVersion: null,
      multiCustody: false,
      alert: {
        show: false,
        type: 'success',
        message: null,
      },
      form: {
        name: null,
        owners: activeWallet.user ? [activeWallet.user.me.id] : [],
        blockchain: null,
        blockchainStandard: null,
        policies: {
          edit: [],
          transfer: [],
        },
      },
    };
  }

  return {
    loading: false,
    isValid: true,
    unchangedVersion: null,
    multiCustody: account.owners.length > 1,
    alert: {
      show: false,
      type: 'success',
      message: null,
    },
    form: {
      name: account.name,
      owners: account.owners,
      blockchain: account.blockchain,
      blockchainStandard: account.standard,
      policies: {
        edit: [],
        transfer: [],
      },
    },
  };
};

export const useAccountForm = (account?: Account) =>
  defineStore(`account-form-${createFormId(account)}`, {
    state: (): AccountFormStoreState => {
      return initialStateForAccount(account);
    },
    getters: {
      canAddOwner(state): boolean {
        // TODO: the length of the nr of max owners should be sent from the backend
        return state.form.owners.length < 10;
      },
      nrOfOwners(state): number {
        return state.form.owners.filter(id => id !== null).length;
      },
      hasChanges(state): boolean {
        return JSON.stringify(state.form) !== state.unchangedVersion;
      },
      canSave(state): boolean {
        return state.isValid && this.hasChanges;
      },
      validationRules(): AccountFormValidationRules {
        return {
          // TODO: the length of these fields should be sent from the backend
          name: [requiredRule, maxLengthRule(50, i18n.global.t('terms.name'))],
          blockchain: [requiredRule],
          blockchainStandard: [requiredRule],
          ownerUser: [requiredRule, validUuidV4Rule],
          ownerIdentity: [requiredRule, validPrincipalRule],
        };
      },
      supportedBlockchains(): Array<{ value: string; title: string }> {
        const activeWallet = useWalletStore();

        return activeWallet.supportedAssets.map(asset => ({
          value: asset.blockchain,
          title: `${asset.symbol}: ${asset.name}`,
        }));
      },
    },
    actions: {
      reset(): void {
        const reset = initialStateForAccount(account);
        this.loading = reset.loading;
        this.isValid = reset.isValid;
        this.form = reset.form;
        this.alert = reset.alert;
        this.multiCustody = reset.multiCustody;
      },
      clearAlert(): void {
        this.alert = {
          show: false,
          type: 'success',
          message: null,
        };
      },
      addOwner(owner: UUID | null): void {
        this.form.owners.push(owner);
      },
      isSelfOwnerEntry(ownerId: UUID | null): boolean {
        const activeWallet = useWalletStore();
        if (ownerId === null) {
          return false;
        }

        return activeWallet.user!.me.id === ownerId;
      },
      removeOwnerByIndex(index: number): void {
        this.form.owners.splice(index, 1);
      },
      supportedBlockchainStandards(): string[] {
        const activeWallet = useWalletStore();
        const supportedAsset = activeWallet.supportedAssets.find(
          asset => asset.blockchain === this.form.blockchain,
        );

        return supportedAsset?.standards ?? [];
      },
      async save(): Promise<Proposal | false> {
        try {
          if (!this.isValid) {
            return false;
          }
          this.clearAlert();
          this.loading = true;

          const wallet = useWalletStore().service;
          const owners: string[] = this.form.owners.filter(id => id !== null) as string[];

          if (account && account.id) {
            return wallet.createProposal({
              title: [],
              summary: [],
              operation: {
                EditAccount: {
                  account_id: account.id,
                  name: this.form.name ? [this.form.name] : [],
                  owners: owners.length ? [owners] : [],
                  policies: [
                    {
                      // TODO: add policies once we have the UI for it
                      edit: [{ AutoAdopted: null }],
                      transfer: [{ AutoAdopted: null }],
                    },
                  ],
                },
              },
              execution_plan: [],
            });
          }

          return wallet.createProposal({
            title: [],
            summary: [],
            operation: {
              AddAccount: {
                name: `${this.form.name}`,
                owners: owners,
                blockchain: `${this.form.blockchain}`,
                standard: `${this.form.blockchainStandard ?? 'native'}`,
                policies: {
                  // TODO: add policies once we have the UI for it
                  edit: [{ AutoAdopted: null }],
                  transfer: [{ AutoAdopted: null }],
                },
                metadata: [],
              },
            },
            execution_plan: [],
          });
        } catch (err) {
          logger.error('Failed to change account', { err });

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
  })();
