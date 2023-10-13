import { Principal } from '@dfinity/principal';
import { defineStore } from 'pinia';
import { i18n, services } from '~/ui/modules';
import { useAuthStore, useBankStore } from '~/ui/stores';
import { FormValidationRules } from '~/ui/types';
import { maxLengthRule, requiredRule, validPrincipalRule } from '~/ui/utils';

export interface SettingsFormBankEntry {
  name: string | null;
  canisterId: string;
}

export interface SettingsFormIdentityEntry {
  name: string | null;
  principal: string;
  confirmed: boolean;
}

export interface SettingsForm {
  name: string | null;
  mainBank: string | null;
  banks: SettingsFormBankEntry[];
  identities: SettingsFormIdentityEntry[];
}

export interface SettingsFormValidationRules {
  accountName: FormValidationRules;
  bankName: FormValidationRules;
  identityName: FormValidationRules;
  validPrincipal: FormValidationRules;
}

export interface SettingsFormStoreState {
  isLoading: boolean;
  isValid: boolean;
  alert: {
    show: boolean;
    type: 'success' | 'error' | 'warning' | 'info';
    message: string | null;
  };
  unchangedVersion: string | null;
  form: SettingsForm;
}

const initialState: SettingsFormStoreState = {
  isLoading: false,
  isValid: true,
  unchangedVersion: null,
  alert: {
    show: false,
    type: 'success',
    message: null,
  },
  form: {
    name: null,
    mainBank: null,
    banks: [],
    identities: [],
  },
};

export const useSettingsFormStore = defineStore('settingsForm', {
  state: (): SettingsFormStoreState => {
    return JSON.parse(JSON.stringify(initialState));
  },
  getters: {
    canAddBank(state): boolean {
      // TODO: the length of the nr of max banks should be sent from the backend
      return state.form.banks.length < 10;
    },
    canAddIdentity(state): boolean {
      // TODO: the length of the nr of max identities should be sent from the backend
      return state.form.identities.length < 10;
    },
    hasChanges(state): boolean {
      return JSON.stringify(state.form) !== state.unchangedVersion;
    },
    canSave(state): boolean {
      return state.isValid && this.hasChanges;
    },
    failedToLoad(state): boolean {
      return state.unchangedVersion === null;
    },
    validationRules(): SettingsFormValidationRules {
      return {
        // TODO: the length of these fields should be sent from the backend
        accountName: [maxLengthRule(100, i18n.global.t('terms.account_name'))],
        identityName: [maxLengthRule(100, i18n.global.t('terms.identity_name'))],
        bankName: [maxLengthRule(100, i18n.global.t('terms.bank_name'))],
        validPrincipal: [requiredRule, validPrincipalRule],
      };
    },
  },
  actions: {
    async reset(): Promise<void> {
      const reset = JSON.parse(JSON.stringify(initialState));
      this.isLoading = reset.isLoading;
      this.isValid = reset.isValid;
      this.form = reset.form;
      this.alert = reset.alert;
    },
    async initialize(): Promise<void> {
      this.reset();
      const account = await services().controlPanel.fetchAccount();

      this.form.name = account.name?.[0] ?? null;
      this.form.mainBank = account.main_bank?.[0]?.toText() ?? null;
      account.banks.forEach(bank => {
        this.form.banks.push({
          name: bank.name?.[0] ?? null,
          canisterId: bank.canister_id.toText(),
        });
      });
      account.identities.forEach(confirmed => {
        this.form.identities.push({
          name: confirmed.name?.[0] ?? null,
          principal: confirmed.identity.toText(),
          confirmed: true,
        });
      });
      account.unconfirmed_identities.forEach(unconfirmed => {
        this.form.identities.push({
          name: unconfirmed.name?.[0] ?? null,
          principal: unconfirmed.identity.toText(),
          confirmed: false,
        });
      });

      this.unchangedVersion = JSON.stringify(this.form);
    },
    removeIdentity(index: number): void {
      this.form.identities.splice(index, 1);
    },
    removeBank(index: number): void {
      const [removed] = this.form.banks.splice(index, 1);

      if (this.form.mainBank === removed.canisterId) {
        this.form.mainBank = this.form.banks.length > 0 ? this.form.banks[0].canisterId : null;
      }
    },
    addIdentity(): void {
      this.form.identities.push({
        name: null,
        principal: '',
        confirmed: false,
      });
    },
    addBank(): void {
      this.form.banks.push({
        name: null,
        canisterId: '',
      });
    },
    toggleMainBank(bankCanisterId: string): void {
      this.form.mainBank = bankCanisterId;
    },
    async load(): Promise<void> {
      this.isLoading = true;
      this.initialize()
        .catch(err => {
          this.alert = {
            show: true,
            type: 'error',
            message: err.message,
          };
        })
        .finally(() => {
          this.isLoading = false;
        });
    },
    async save(): Promise<void> {
      this.isLoading = true;

      const controlPanelService = services().controlPanel;
      await controlPanelService
        .editAccount({
          name: this.form.name ? [this.form.name] : [],
          main_bank: this.form.mainBank ? [Principal.fromText(this.form.mainBank)] : [],
          banks: [
            this.form.banks.map(bank => ({
              name: bank.name ? [bank.name] : [],
              canister_id: Principal.fromText(bank.canisterId),
            })),
          ],
          identities: [
            this.form.identities.map(identity => ({
              name: identity.name ? [identity.name] : [],
              identity: Principal.fromText(identity.principal),
            })),
          ],
        })
        .then(account => {
          this.alert = {
            show: true,
            type: 'success',
            message: i18n.global.t('settings.edit_success'),
          };

          const auth = useAuthStore();
          const bank = useBankStore();
          auth.editAccount({ name: account.name?.[0] ?? null });
          bank.useBanks(
            account.banks.map(bank => ({
              canisterId: bank.canister_id.toText(),
              name: bank.name?.[0] ?? null,
            })),
          );
          this.unchangedVersion = JSON.stringify(this.form);
        })
        .catch(err => {
          this.alert = {
            show: true,
            type: 'error',
            message: err.message,
          };
        })
        .finally(() => {
          this.isLoading = false;
        });
    },
  },
});
