<template>
  <VCard color="background" variant="flat" :loading="store.loading">
    <VCardTitle class="font-weight-bold">{{
      $t(`app.wallet_info_card_title`, { name: wallet.name })
    }}</VCardTitle>
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
          <VListItemSubtitle>{{ wallet.name ?? '-' }}</VListItemSubtitle>
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
      <VBtn
        v-if="wallet.canisterId"
        :loading="store.editDialog.loading"
        variant="flat"
        color="primary-variant"
        size="small"
        @click="
          store.openEditDialog(Principal.fromText(wallet.canisterId), {
            name: wallet.name,
            main: isMainWallet,
          })
        "
      >
        {{ $t(`app.wallet_info_card_edit_btn`) }}
      </VBtn>
    </VCardActions>

    <WalletInfoEditDialog />
  </VCard>
</template>

<script lang="ts" setup>
import { Principal } from '@dfinity/principal';
import { mdiContentCopy } from '@mdi/js';
import { computed } from 'vue';
import { useSessionStore } from '~/ui/stores/session';
import { useWalletStore } from '~/ui/stores/wallet';
import { copyToClipboard } from '~/ui/utils';
import WalletInfoEditDialog from './WalletInfoEditDialog.vue';
import { useStore } from './store';

const wallet = useWalletStore();
const session = useSessionStore();
const store = useStore();

const isMainWallet = computed(() => wallet.canisterId === session.user.mainWallet);
</script>
