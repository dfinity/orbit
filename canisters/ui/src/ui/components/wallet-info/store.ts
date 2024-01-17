import { Principal } from '@dfinity/principal';
import { defineStore } from 'pinia';
import { logger } from '~/core';
import { UserWallet } from '~/generated/control-panel/control_panel.did';
import { i18n, services } from '~/ui/modules';
import { useSessionStore } from '~/ui/stores/session';
import { FormValidationRules } from '~/ui/types';
import { maxLengthRule } from '~/ui/utils';

export interface FormFields {
  name: string | null;
  main: boolean;
}

export interface ValidationRules {
  walletName: FormValidationRules;
}

export interface StoreState {
  loading: boolean;
  editDialog: {
    open: boolean;
    loading: boolean;
    isValid: boolean;
    canisterId: string;
    unchangedVersion: string | null;
    form: FormFields;
  };
}

export const useStore = (sectionId: string = 'main') =>
  defineStore(`wallet-info-card-${sectionId}`, {
    state: (): StoreState => {
      return {
        loading: false,
        editDialog: {
          open: false,
          loading: false,
          isValid: true,
          canisterId: '',
          form: {
            name: null,
            main: false,
          },
          unchangedVersion: null,
        },
      };
    },
    getters: {
      hasChanges(state): boolean {
        return JSON.stringify(state.editDialog.form) !== state.editDialog.unchangedVersion;
      },
      canSave(state): boolean {
        return state.editDialog.isValid && this.hasChanges && !state.editDialog.loading;
      },
      validationRules(): ValidationRules {
        return {
          // TODO: the length of these fields should be sent from the backend
          walletName: [maxLengthRule(100, i18n.global.t('terms.wallet_name'))],
        };
      },
    },
    actions: {
      async openEditDialog(canisterId: Principal, form: FormFields): Promise<void> {
        this.$state.editDialog.form = form;
        this.$state.editDialog.unchangedVersion = JSON.stringify(form);
        this.$state.editDialog.open = true;
        this.$state.editDialog.canisterId = canisterId.toText();
      },
      closeEditDialog(): void {
        this.editDialog.open = false;
      },
      async saveChanges(): Promise<void> {
        if (!this.canSave) {
          logger.warn('Cannot save changes' + JSON.stringify(this.$state.editDialog));
          return;
        }

        try {
          this.$state.editDialog.loading = true;

          const session = useSessionStore();
          const controlPanel = services().controlPanel;
          const mainWallet = this.$state.editDialog.form.main
            ? Principal.fromText(this.$state.editDialog.canisterId)
            : session.mainWallet;

          const updatedWallets: UserWallet[] =
            session.data.wallets.map(wallet => {
              if (wallet.canisterId === this.$state.editDialog.canisterId) {
                return {
                  name: this.$state.editDialog.form.name ? [this.$state.editDialog.form.name] : [],
                  canister_id: Principal.fromText(this.$state.editDialog.canisterId),
                };
              }

              return {
                name: wallet.name ? [wallet.name] : [],
                canister_id: Principal.fromText(wallet.canisterId),
              };
            }) ?? [];

          const user = await controlPanel.editUser({
            main_wallet: mainWallet ? [mainWallet] : [],
            wallets: updatedWallets.length ? [updatedWallets] : [],
          });

          // uopdate the user to set the most recent data
          session.populateUser(user);

          this.$state.editDialog.open = false;
        } catch (e) {
          logger.error('Failed to save changes', e);
        } finally {
          this.$state.editDialog.loading = false;
        }
      },
    },
  })();
