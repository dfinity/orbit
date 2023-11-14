import { Principal } from '@dfinity/principal';
import { defineStore } from 'pinia';
import { i18n, services } from '~/ui/modules';
import { useAuthStore, useWalletStore } from '~/ui/stores';
import { FormValidationRules } from '~/ui/types';
import { maxLengthRule, requiredRule, validPrincipalRule } from '~/ui/utils';

export interface SettingsFormWalletEntry {
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
  mainWallet: string | null;
  wallets: SettingsFormWalletEntry[];
  identities: SettingsFormIdentityEntry[];
}

export interface SettingsFormValidationRules {
  userName: FormValidationRules;
  walletName: FormValidationRules;
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
    mainWallet: null,
    wallets: [],
    identities: [],
  },
};

export const useSettingsFormStore = defineStore('settingsForm', {
  state: (): SettingsFormStoreState => {
    return JSON.parse(JSON.stringify(initialState));
  },
  getters: {
    canAddWallet(state): boolean {
      // TODO: the length of the nr of max wallets should be sent from the backend
      return state.form.wallets.length < 10;
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
        userName: [maxLengthRule(100, i18n.global.t('terms.user_name'))],
        identityName: [maxLengthRule(100, i18n.global.t('terms.identity_name'))],
        walletName: [maxLengthRule(100, i18n.global.t('terms.wallet_name'))],
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
      const user = await services().controlPanel.getCurrentUser();

      this.form.name = user.name?.[0] ?? null;
      this.form.mainWallet = user.main_wallet?.[0]?.toText() ?? null;
      user.wallets.forEach(wallet => {
        this.form.wallets.push({
          name: wallet.name?.[0] ?? null,
          canisterId: wallet.canister_id.toText(),
        });
      });
      user.identities.forEach(confirmed => {
        this.form.identities.push({
          name: confirmed.name?.[0] ?? null,
          principal: confirmed.identity.toText(),
          confirmed: true,
        });
      });
      user.unconfirmed_identities.forEach(unconfirmed => {
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
    removeWallet(index: number): void {
      const [removed] = this.form.wallets.splice(index, 1);

      if (this.form.mainWallet === removed.canisterId) {
        this.form.mainWallet =
          this.form.wallets.length > 0 ? this.form.wallets[0].canisterId : null;
      }
    },
    addIdentity(): void {
      this.form.identities.push({
        name: null,
        principal: '',
        confirmed: false,
      });
    },
    addWallet(): void {
      this.form.wallets.push({
        name: null,
        canisterId: '',
      });
    },
    toggleMainWallet(walletCanisterId: string): void {
      this.form.mainWallet = walletCanisterId;
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
        .editUser({
          name: this.form.name ? [this.form.name] : [],
          main_wallet: this.form.mainWallet ? [Principal.fromText(this.form.mainWallet)] : [],
          wallets: [
            this.form.wallets.map(wallet => ({
              name: wallet.name ? [wallet.name] : [],
              canister_id: Principal.fromText(wallet.canisterId),
            })),
          ],
          identities: [
            this.form.identities.map(identity => ({
              name: identity.name ? [identity.name] : [],
              identity: Principal.fromText(identity.principal),
            })),
          ],
        })
        .then(user => {
          this.alert = {
            show: true,
            type: 'success',
            message: i18n.global.t('settings.edit_success'),
          };

          const auth = useAuthStore();
          const wallet = useWalletStore();
          auth.editUser({ name: user.name?.[0] ?? null });
          wallet.useWallets(
            user.wallets.map(wallet => ({
              canisterId: wallet.canister_id.toText(),
              name: wallet.name?.[0] ?? null,
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
