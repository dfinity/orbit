import { defineStore } from 'pinia';
import { logger } from '~/core';
import { UserId, AccountPolicy } from '~/generated/bank/bank.did';
import { i18n, router } from '~/ui/modules';
import { useActiveBankStore } from '~/ui/stores';
import { FormValidationRules } from '~/ui/types';
import { maxLengthRule, requiredRule, validPrincipalRule, validUuidV4Rule } from '~/ui/utils';

export interface CreateAccountForm {
  name: string | null;
  owners: Array<UserId | null>;
  blockchain: string | null;
  blockchainStandard: string | null;
  policies: Array<AccountPolicy | null>;
}

export interface CreateAccountFormStoreState {
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
  form: CreateAccountForm;
}

export interface CreateAccountFormValidationRules {
  name: FormValidationRules;
  blockchain: FormValidationRules;
  blockchainStandard: FormValidationRules;
  ownerUser: FormValidationRules;
  ownerIdentity: FormValidationRules;
}

const initialState: CreateAccountFormStoreState = {
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

export const useCreateAccountFormStore = defineStore('createAccountForm', {
  state: (): CreateAccountFormStoreState => {
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
      return state.form.owners.filter(id => id !== null).length;
    },
    hasChanges(state): boolean {
      return JSON.stringify(state.form) !== state.unchangedVersion;
    },
    canSave(state): boolean {
      return state.isValid && this.hasChanges;
    },
    validationRules(): CreateAccountFormValidationRules {
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
    addOwner(owner: UserId | null): void {
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
    isSelfOwnerEntry(ownerId: UserId | null): boolean {
      const activeBank = useActiveBankStore();
      if (ownerId === null) {
        return false;
      }

      return activeBank.user.id === ownerId;
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
      this.addOwner(activeBank.user.id);

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

        const policies: AccountPolicy[] = this.form.policies.filter(
          entry => entry !== null,
        ) as AccountPolicy[];

        const nrOfThresholdPolicies = policies.filter(
          policy => 'approval_threshold' in policy,
        ).length;
        if (nrOfThresholdPolicies > 1) {
          throw new Error(i18n.global.t('banks.policy_misconfigured'));
        }

        const activeBank = useActiveBankStore();
        const bankService = activeBank.service;

        await bankService
          .createAccount({
            name: this.form.name ? [this.form.name] : [],
            owners: this.form.owners.filter(id => id !== null) as UserId[],
            blockchain: `${this.form.blockchain}`,
            standard: !this.form.blockchainStandard ? 'native' : this.form.blockchainStandard,
            metadata: [],
            policies,
          })
          .then(result => {
            activeBank.loadAccountList();
            this.close();

            router.push({ name: 'Account', params: { id: result.id } });
          });
      } catch (err) {
        logger.error('Failed to create account', { err });

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
