<template>
  <PageLayout v-if="activeBank.hasAccount">
    <template #main-header>
      <VContainer class="pt-16 pb-16 pl-8 pr-8" fluid>
        <VRow>
          <VCol md="6" sm="12">
            <h1 class="text-h4">{{ $t('terms.wallets') }}</h1>
          </VCol>
          <VCol md="6" sm="12" class="header-actions">
            <NewWalletBtn />
            <NewTransferBtn />
          </VCol>
        </VRow>
      </VContainer>
    </template>
    <template #main-body>
      <VContainer class="pl-8 pr-8" fluid>
        <VRow v-if="activeBank.wallets.items.length">
          <VCol v-for="(wallet, idx) in activeBank.wallets.items" :key="idx" cols="12" md="6">
            <VCard density="compact" variant="elevated" class="wallet-card">
              <VCardTitle>
                <VIcon :icon="mdiWallet" size="x-small" class="mr-2" />
                <template v-if="wallet.name?.length">
                  {{ wallet.name[0] }}
                </template>
              </VCardTitle>
              <VCardSubtitle class="wallet-card__subtitle">
                <span>{{ wallet.symbol }}</span>
                <template v-if="wallet.address">
                  <span>:&nbsp;</span>
                  <span class="wallet-card__subtitle__address" :title="wallet.address">
                    {{ wallet.address }}
                  </span>
                  <VBtn
                    class="wallet-card__subtitle__copy"
                    size="x-small"
                    variant="text"
                    :icon="mdiContentCopy"
                    @click="copyAddressToClipboard(wallet.address)"
                  />
                </template>
              </VCardSubtitle>
              <VCardText class="pb-0">
                <p>
                  <span
                    v-if="wallet.balance?.[0]"
                    class="wallet-card__amount--available"
                    :title="wallet.balance?.[0]?.last_update_timestamp"
                  >
                    {{ formatBalance(wallet.balance[0].balance, wallet.balance[0].decimals) }}
                  </span>
                  <span v-else class="wallet-card__amount wallet-card__amount--unavailable">-</span>
                  {{ wallet.symbol }}
                </p>
              </VCardText>
              <VCardActions>
                <VChip
                  size="x-small"
                  color="primary-variant"
                  variant="tonal"
                  :prepend-icon="wallet.owners.length > 1 ? mdiAccountGroup : mdiAccount"
                >
                  {{
                    wallet.owners.length > 1 ? $t('banks.joint_wallet') : $t('banks.private_wallet')
                  }}
                </VChip>
                <VSpacer />
                <VBtn
                  size="small"
                  variant="tonal"
                  :append-icon="mdiOpenInApp"
                  :to="{ name: 'WalletDetails', params: { id: wallet.id } }"
                >
                  {{ $t('terms.open') }}
                </VBtn>
              </VCardActions>
            </VCard>
          </VCol>
        </VRow>
        <VRow v-else>
          <VCol cols="12">
            <p class="text-h5">{{ $t('banks.no_wallets') }}</p>
          </VCol>
        </VRow>
      </VContainer>
    </template>
  </PageLayout>
  <PageLayout v-else>
    <template #main-header>
      <VContainer class="pt-16 pb-16 pl-8 pr-8" fluid>
        <VRow>
          <VCol sm="12">
            <h1 class="text-h4">{{ $t('banks.no_bank_account') }}</h1>
            <p class="text-subtitle">{{ $t('banks.please_register_to_continue') }}</p>
          </VCol>
        </VRow>
      </VContainer>
    </template>
  </PageLayout>
</template>

<script lang="ts" setup>
import { mdiAccount, mdiAccountGroup, mdiContentCopy, mdiOpenInApp, mdiWallet } from '@mdi/js';
import { formatBalance } from '~/core';
import NewTransferBtn from '~/ui/components/NewTransferBtn.vue';
import NewWalletBtn from '~/ui/components/NewWalletBtn.vue';
import PageLayout from '~/ui/components/PageLayout.vue';
import { i18n } from '~/ui/modules';
import { useActiveBankStore, useSettingsStore } from '~/ui/stores';

const activeBank = useActiveBankStore();
const settings = useSettingsStore();

const copyAddressToClipboard = (address: string) => {
  navigator.clipboard.writeText(address);

  settings.setNotification({
    show: true,
    type: 'success',
    message: i18n.global.t('banks.wallet_address_copied_to_clipboard'),
  });
};
</script>

<style scoped lang="scss">
.header-actions {
  display: flex;
  justify-content: end;
  align-items: center;
  gap: calc(var(--ds-bdu) * 2);

  :deep(.v-btn) {
    flex-grow: 1;
  }
}

.page-layout--mobile {
  .header-actions {
    justify-content: center;
  }
}

.wallet-card {
  &__subtitle {
    display: flex;
    flex-direction: row;
    overflow: visible;
    width: 100%;

    &__address {
      white-space: nowrap;
      overflow: hidden;
      text-overflow: ellipsis;
      max-width: calc(100% - calc(var(--ds-bdu) * 6));
    }

    &__copy {
      margin-top: -8px;
    }
  }
}
</style>
