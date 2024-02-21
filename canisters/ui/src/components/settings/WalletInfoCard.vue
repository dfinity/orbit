<template>
  <VCard color="background" variant="flat">
    <VCardTitle>
      {{ $t(`app.wallet_info_card_title`, { name: wallet.name }) }}
    </VCardTitle>
    <VCardText class="pb-0">
      <VList lines="two" class="bg-background">
        <VListItem v-if="wallet.canisterId" class="px-0">
          <VListItemTitle class="font-weight-bold">{{ $t(`terms.canister_id`) }}</VListItemTitle>
          <VListItemSubtitle>
            <span>
              {{ wallet.canisterId }}
              <VBtn
                size="x-small"
                variant="text"
                :icon="mdiContentCopy"
                @click="
                  copyToClipboard({
                    textToCopy: wallet.canisterId,
                    sendNotification: true,
                  })
                "
              />
            </span>
          </VListItemSubtitle>
        </VListItem>
        <VListItem class="px-0">
          <VListItemTitle class="font-weight-bold">{{ $t(`terms.wallet_name`) }}</VListItemTitle>
          <VListItemSubtitle data-test-id="wallet-name">{{ wallet.name ?? '-' }}</VListItemSubtitle>
        </VListItem>
        <VListItem class="px-0">
          <VListItemTitle class="font-weight-bold">{{ $t(`terms.main`) }}</VListItemTitle>
          <VListItemSubtitle>{{
            isMainWallet ? $t(`terms.yes`) : $t(`terms.no`)
          }}</VListItemSubtitle>
        </VListItem>
      </VList>
    </VCardText>
    <VCardActions class="px-4 pb-4">
      <ActionBtn
        v-model="walletConfigInput"
        :text="$t(`app.wallet_info_card_edit_btn`)"
        :title="$t(`app.wallet_info_card_edit_btn`)"
        color="primary-variant"
        :submit="save"
        size="small"
        variant="flat"
        data-test-id="update-wallet-details-btn"
        @failed="onFailedOperation"
        @submitted="onSuccessfulOperation"
      >
        <template #default="{ model: elem, submit }">
          <WalletInfoForm
            v-model="elem.value.model"
            @valid="isValid => (elem.value.valid = isValid)"
            @submit="submit"
          />
        </template>
        <template #actions="{ submit, loading: saving, model: elem }">
          <VSpacer />
          <VBtn
            :loading="saving"
            :disabled="!elem.value.valid"
            color="primary"
            variant="flat"
            @click="submit"
          >
            {{ $t('terms.save') }}
          </VBtn>
        </template>
      </ActionBtn>
      <ActionBtn
        data-test-id="remove-wallet-btn"
        :text="$t(`app.wallet_info_card_remove_btn`)"
        :title="$t(`app.wallet_info_card_remove_btn`)"
        :content="$t(`app.wallet_info_card_remove_btn_confirm`)"
        variant="text"
        :submit="removeWallet"
        :disabled="!isWalletRemovable"
      >
      </ActionBtn>
    </VCardActions>
  </VCard>
</template>

<script lang="ts" setup>
import { Principal } from '@dfinity/principal';
import { mdiContentCopy } from '@mdi/js';
import { computed, ref } from 'vue';
import ActionBtn from '~/components/buttons/ActionBtn.vue';
import { UserWallet } from '~/generated/control-panel/control_panel.did';
import { sessionUserWalletToUserWallet } from '~/mappers/wallets.mapper';
import { i18n } from '~/plugins/i18n.plugin';
import { services } from '~/plugins/services.plugin';
import { useAppStore } from '~/stores/app.store';
import { useSessionStore } from '~/stores/session.store';
import { useWalletStore } from '~/stores/wallet.store';
import { copyToClipboard } from '~/utils/app.utils';
import WalletInfoForm, { WalletInfoModel } from './WalletInfoForm.vue';

const wallet = useWalletStore();
const session = useSessionStore();
const app = useAppStore();
const isMainWallet = computed(() => wallet.canisterId === session.mainWallet?.toText());
const isWalletRemovable = computed(() => !isMainWallet.value && session.data.wallets.length > 1);
const controlPanelService = services().controlPanel;

async function removeWallet(): Promise<void> {
  if (!isWalletRemovable.value) {
    return;
  }

  const updatedUser = await services().controlPanel.editUser({
    main_wallet: [], // do not change the main wallet
    wallets: [
      session.data.wallets
        .filter(w => w.canisterId !== wallet.canisterId)
        .map(w => sessionUserWalletToUserWallet(w)),
    ],
  });

  session.populateUser(updatedUser);

  let maybeWalletToRedirect = session.mainWallet;
  if (!maybeWalletToRedirect && session.data.wallets[0]?.canisterId) {
    maybeWalletToRedirect = Principal.fromText(session.data.wallets[0].canisterId);
  }

  if (maybeWalletToRedirect) {
    await session.connectWallet(maybeWalletToRedirect);
  } else {
    session.disconnectWallet();
  }
}

const onFailedOperation = (): void => {
  app.sendNotification({
    type: 'error',
    message: i18n.global.t('app.request_failed_message'),
  });

  walletConfigInput.value = initialCreateInput();
};

const onSuccessfulOperation = (): void => {
  app.sendNotification({
    type: 'success',
    message: i18n.global.t('app.request_completed_message'),
  });

  walletConfigInput.value = initialCreateInput();
};

const save = async ({ model }: { valid: boolean; model: WalletInfoModel }): Promise<void> => {
  const mainWallet = model.main ? Principal.fromText(wallet.canisterId) : session.mainWallet;
  const updatedWallets: UserWallet[] =
    session.data.wallets.map(entry => {
      if (entry.canisterId === wallet.canisterId) {
        return {
          name: model.name ? [model.name] : [],
          canister_id: Principal.fromText(entry.canisterId),
        };
      }

      return {
        name: entry.name ? [entry.name] : [],
        canister_id: Principal.fromText(entry.canisterId),
      };
    }) ?? [];

  const user = await controlPanelService.editUser({
    main_wallet: mainWallet ? [mainWallet] : [],
    wallets: updatedWallets.length ? [updatedWallets] : [],
  });

  session.populateUser(user);
};

const initialCreateInput = (): {
  valid: boolean;
  model: WalletInfoModel;
} => ({
  valid: false,
  model: {
    name: wallet.name,
    main: session.mainWallet?.toText() === wallet.canisterId,
  },
});

const walletConfigInput = ref<{
  valid: boolean;
  model: WalletInfoModel;
}>(initialCreateInput());
</script>
