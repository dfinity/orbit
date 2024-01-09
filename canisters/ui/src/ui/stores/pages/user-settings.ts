import { Principal } from '@dfinity/principal';
import { defineStore } from 'pinia';
import { logger } from '~/core';
import { UserWallet } from '~/generated/control-panel/control_panel.did';
import { i18n, services } from '~/ui/modules';
import { useSessionStore } from '~/ui/stores';
import { FormValidationRules } from '~/ui/types';
import { maxLengthRule } from '~/ui/utils';

export interface WalletEditFieldsValidationRules {
  walletName: FormValidationRules;
}

export interface WalletEditFields {
  name: string | null;
  main: boolean;
  canisterId: string;
}

export interface UserSettingsPage {
  removeWalletDialog: {
    open: boolean;
    saving: boolean;
    canisterId: string;
  };
  walletEditDialog: {
    isValid: boolean;
    saving: boolean;
    open: boolean;
    unchangedVersion: string | null;
    fields: WalletEditFields;
  };
}

const initialState: UserSettingsPage = {
  removeWalletDialog: {
    open: false,
    saving: false,
    canisterId: '',
  },
  walletEditDialog: {
    saving: false,
    open: false,
    unchangedVersion: null,
    isValid: true,
    fields: {
      name: null,
      main: false,
      canisterId: '',
    },
  },
};

export const useUserSettingsPage = defineStore('userSettingsPage', {
  state: (): UserSettingsPage => {
    return JSON.parse(JSON.stringify(initialState));
  },
  getters: {
    walletHasChanges(state): boolean {
      return (
        JSON.stringify(state.walletEditDialog.fields) !== state.walletEditDialog.unchangedVersion
      );
    },
    walletValidationRules(): WalletEditFieldsValidationRules {
      return {
        // TODO: the length of these fields should be sent from the backend
        walletName: [maxLengthRule(100, i18n.global.t('terms.wallet_name'))],
      };
    },
  },
  actions: {
    async reset(): Promise<void> {
      const reset = JSON.parse(JSON.stringify(initialState)) as UserSettingsPage;
      this.walletEditDialog = reset.walletEditDialog;
      this.removeWalletDialog = reset.removeWalletDialog;
    },
    closeWalletEditDialog(): void {
      this.walletEditDialog.open = false;
    },
    closeRemoveWalletDialog(): void {
      this.removeWalletDialog.open = false;
    },
    async editWallet(wallet: WalletEditFields): Promise<void> {
      this.reset();

      this.walletEditDialog.fields = wallet;
      this.walletEditDialog.unchangedVersion = JSON.stringify(wallet);
      this.walletEditDialog.open = true;
    },
    async confirmRemoveWallet(canisterId: Principal): Promise<void> {
      this.reset();

      this.removeWalletDialog.canisterId = canisterId.toText();
      this.removeWalletDialog.open = true;
    },
    async removeWallet(): Promise<void> {
      if (this.removeWalletDialog.saving) {
        logger.warn('Cannot remove wallet' + JSON.stringify(this.removeWalletDialog));
        return;
      }

      try {
        this.removeWalletDialog.saving = true;

        const session = useSessionStore();
        const controlPanel = services().controlPanel;

        const updatedWallets: UserWallet[] =
          session.user.wallets
            .filter(wallet => wallet.canisterId !== this.removeWalletDialog.canisterId)
            .map(wallet => ({
              name: wallet.name ? [wallet.name] : [],
              canister_id: Principal.fromText(wallet.canisterId),
            }));

        await controlPanel.editUser({
          main_wallet: [],
          wallets: updatedWallets.length ? [updatedWallets] : [],
        });

        // reloads the user to get the most recent data
        await session.loadUser();

        this.removeWalletDialog.open = false;
      } catch (e) {
        logger.error('Failed to remove wallet', e);
      } finally {
        this.removeWalletDialog.saving = false;
      }
    },
    async saveWallet(): Promise<void> {
      if (
        !this.walletEditDialog.isValid ||
        !this.walletHasChanges ||
        this.walletEditDialog.saving
      ) {
        logger.warn('Cannot save wallet' + JSON.stringify(this.walletEditDialog));
        return;
      }

      try {
        this.walletEditDialog.saving = true;

        const session = useSessionStore();
        const controlPanel = services().controlPanel;
        const mainWallet = this.walletEditDialog.fields.main
          ? Principal.fromText(this.walletEditDialog.fields.canisterId)
          : session.mainWallet;

        const updatedWallets: UserWallet[] =
          session.user.wallets.map(wallet => {
            if (wallet.canisterId === this.walletEditDialog.fields.canisterId) {
              return {
                name: this.walletEditDialog.fields.name ? [this.walletEditDialog.fields.name] : [],
                canister_id: Principal.fromText(this.walletEditDialog.fields.canisterId),
              };
            }

            return {
              name: wallet.name ? [wallet.name] : [],
              canister_id: Principal.fromText(wallet.canisterId),
            };
          }) ?? [];

        await controlPanel.editUser({
          main_wallet: mainWallet ? [mainWallet as Principal] : [],
          wallets: updatedWallets.length ? [updatedWallets] : [],
        });

        // reloads the user to get the most recent data
        await session.loadUser();

        this.walletEditDialog.open = false;
      } catch (e) {
        logger.error('Failed to save wallet', e);
      } finally {
        this.walletEditDialog.saving = false;
      }
    },
  },
});
