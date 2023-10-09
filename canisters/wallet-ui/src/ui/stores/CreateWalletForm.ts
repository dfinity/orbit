import { Principal } from '@dfinity/principal';
import { defineStore } from 'pinia';
import { logger } from '~/core';
import { WalletPolicy } from '~/generated/bank/bank.did';
import { i18n, router } from '~/ui/modules';
import { useActiveBankStore } from '~/ui/stores';
import { FormValidationRules } from '~/ui/types';
import { maxLengthRule, requiredRule, validPrincipalRule, validUuidV4Rule } from '~/ui/utils';

export interface WalletOwnerEntry {
  type: 'account' | 'principal';
  id: string | null;
}

export interface CreateWalletForm {
  name: string | null;
  owners: WalletOwnerEntry[];
  blockchain: string | null;
  blockchainStandard: string | null;
  policies: Array<WalletPolicy | null>;
}

export interface CreateWalletFormStoreState {
  show: boolean;
  loading: boolean;
  isValid: boolean;
  multiCustody: boolean;
  alert: {
    show: boolean;
    type: 'success' | 'error' | 'warning' | 'info';
    message: string | null;
  };
  unchangedVersion: string | null;
  form: CreateWalletForm;
}

export interface CreateWalletFormValidationRules {
  name: FormValidationRules;
  blockchain: FormValidationRules;
  blockchainStandard: FormValidationRules;
  ownerAccount: FormValidationRules;
  ownerIdentity: FormValidationRules;
}

const initialState: CreateWalletFormStoreState = {
  show: false,
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
    owners: [],
    blockchain: null,
    blockchainStandard: null,
    policies: [],
  },
};

export const useCreateWalletFormStore = defineStore('createWalletForm', {
  state: (): CreateWalletFormStoreState => {
    return JSON.parse(JSON.stringify(initialState));
  },
  getters: {
    canAddOwner(state): boolean {
      // TODO: the length of the nr of max banks should be sent from the backend
      return state.form.owners.length < 10;
    },
    canAddPolicy(state): boolean {
      // TODO: the length of the nr of max banks should be sent from the backend
      return state.form.policies.length < 10;
    },
    nrOfOwners(state): number {
      return state.form.owners.filter(owner => owner.id !== null).length;
    },
    hasChanges(state): boolean {
      return JSON.stringify(state.form) !== state.unchangedVersion;
    },
    canSave(state): boolean {
      return state.isValid && this.hasChanges;
    },
    validationRules(): CreateWalletFormValidationRules {
      return {
        // TODO: the length of these fields should be sent from the backend
        name: [requiredRule, maxLengthRule(50, i18n.global.t('terms.name'))],
        blockchain: [requiredRule],
        blockchainStandard: [requiredRule],
        ownerAccount: [requiredRule, validUuidV4Rule],
        ownerIdentity: [requiredRule, validPrincipalRule],
      };
    },
    supportedBlockchains(): Array<{ value: string; title: string }> {
      const activeBank = useActiveBankStore();

      return activeBank.supportedAssets.map(asset => ({
        value: asset.blockchain,
        title: `${asset.symbol}: ${asset.name}`,
      }));
    },
  },
  actions: {
    reset(): void {
      const reset = JSON.parse(JSON.stringify(initialState));
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
    close(): void {
      this.reset();
      this.show = false;
    },
    addOwner(owner: WalletOwnerEntry): void {
      this.form.owners.push(owner);
    },
    addNewPolicy(): void {
      if (this.form.policies.length === 0) {
        this.form.policies.push({
          approval_threshold: {
            FixedThreshold: 1,
          },
        });

        return;
      }

      this.form.policies.push(null);
    },
    isSelfOwnerEntry(ownerEntry: WalletOwnerEntry): boolean {
      const activeBank = useActiveBankStore();
      if (ownerEntry.id === null) {
        return false;
      }

      if (ownerEntry.type === 'account') {
        return activeBank.account.id === ownerEntry.id;
      }

      if (ownerEntry.type === 'principal') {
        return activeBank.account.identities.map(i => i.toText()).includes(ownerEntry.id);
      }

      return false;
    },
    removeOwnerByIndex(index: number): void {
      this.form.owners.splice(index, 1);
    },
    removePolicyByIndex(index: number): void {
      this.form.policies.splice(index, 1);
    },
    open(): void {
      this.reset();
      const activeBank = useActiveBankStore();
      this.addOwner({ type: 'account', id: activeBank.account.id });

      this.show = true;
    },
    supportedBlockchainStandards(): string[] {
      const activeBank = useActiveBankStore();
      const supportedAsset = activeBank.supportedAssets.find(
        asset => asset.blockchain === this.form.blockchain,
      );

      return supportedAsset?.standards ?? [];
    },
    async save(): Promise<void> {
      try {
        if (!this.isValid) {
          return;
        }
        this.clearAlert();
        this.loading = true;

        const policies: WalletPolicy[] = this.form.policies.filter(
          entry => entry !== null,
        ) as WalletPolicy[];

        const nrOfThresholdPolicies = policies.filter(
          policy => 'approval_threshold' in policy,
        ).length;
        if (nrOfThresholdPolicies > 1) {
          throw new Error(i18n.global.t('banks.policy_misconfigured'));
        }

        const activeBank = useActiveBankStore();
        const bankService = activeBank.service;

        await bankService
          .createWallet({
            name: this.form.name ? [this.form.name] : [],
            owners: this.form.owners.map(owner => {
              if (owner.type === 'account') {
                return { AccountID: `${owner.id}` };
              }

              return { Principal: Principal.fromText(`${owner.id}`) };
            }),
            blockchain: `${this.form.blockchain}`,
            standard: !this.form.blockchainStandard ? 'native' : this.form.blockchainStandard,
            metadata: [],
            policies,
          })
          .then(result => {
            activeBank.loadWalletList();
            this.close();

            router.push({ name: 'WalletDetails', params: { id: result.id } });
          });
      } catch (err) {
        logger.error('Failed to create wallet', { err });

        const e = err as Error;
        this.alert = {
          show: true,
          type: 'error',
          message: e.message,
        };
      } finally {
        this.loading = false;
      }
    },
  },
});
