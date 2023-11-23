<template>
  <PageLayout v-if="activeWallet.hasUser">
    <template #main-header>
      <VContainer class="pt-16 pb-16 pl-8 pr-8" fluid>
        <VRow>
          <VCol md="6" sm="12">
            <h1 class="text-h4">{{ $t('terms.accounts') }}</h1>
          </VCol>
          <VCol md="6" sm="12" class="header-actions">
            <AddAccountBtn />
            <NewTransferBtn />
          </VCol>
        </VRow>
      </VContainer>
    </template>
    <template #main-body>
      <VContainer class="pl-8 pr-8" fluid>
        <VRow v-if="activeWallet.accounts.items.length">
          <VCol v-for="(account, idx) in activeWallet.sortedAccounts" :key="idx" cols="12" md="6">
            <VCard density="compact" variant="elevated" class="account-card">
              <VCardTitle>
                <VIcon :icon="mdiWallet" size="x-small" class="mr-2" />
                {{ account.name }}
              </VCardTitle>
              <VCardSubtitle class="account-card__subtitle">
                <span>{{ account.symbol }}</span>
                <template v-if="account.address">
                  <span>:&nbsp;</span>
                  <span class="account-card__subtitle__address" :title="account.address">
                    {{ account.address }}
                  </span>
                  <VBtn
                    class="account-card__subtitle__copy"
                    size="x-small"
                    variant="text"
                    :icon="mdiContentCopy"
                    @click="copyAddressToClipboard(account.address)"
                  />
                </template>
              </VCardSubtitle>
              <VCardText class="pb-0">
                <p>
                  <span
                    v-if="account.balance?.[0]"
                    class="account-card__amount--available"
                    :title="account.balance?.[0]?.last_update_timestamp"
                  >
                    {{ formatBalance(account.balance[0].balance, account.balance[0].decimals) }}
                  </span>
                  <span v-else class="account-card__amount account-card__amount--unavailable"
                    >-</span
                  >
                  {{ account.symbol }}
                </p>
              </VCardText>
              <VCardActions>
                <VChip
                  size="x-small"
                  color="primary-variant"
                  variant="tonal"
                  :prepend-icon="account.owners.length > 1 ? mdiAccountGroup : mdiAccount"
                >
                  {{
                    account.owners.length > 1
                      ? $t('wallets.joint_account')
                      : $t('wallets.private_account')
                  }}
                </VChip>
                <VSpacer />
                <VBtn
                  size="small"
                  variant="tonal"
                  :append-icon="mdiOpenInApp"
                  :to="{ name: 'Account', params: { id: account.id } }"
                >
                  {{ $t('terms.open') }}
                </VBtn>
              </VCardActions>
            </VCard>
          </VCol>
        </VRow>
        <VRow v-else>
          <VCol cols="12">
            <p class="text-h5">{{ $t('wallets.no_accounts') }}</p>
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
            <h1 class="text-h4">{{ $t('wallets.no_wallet_user') }}</h1>
            <p class="text-subtitle">{{ $t('wallets.please_register_to_continue') }}</p>
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
import AddAccountBtn from '~/ui/components/accounts/AddAccountBtn.vue';
import PageLayout from '~/ui/components/PageLayout.vue';
import { i18n } from '~/ui/modules';
import { useActiveWalletStore, useSettingsStore } from '~/ui/stores';

const activeWallet = useActiveWalletStore();
const settings = useSettingsStore();

const copyAddressToClipboard = (address: string) => {
  navigator.clipboard.writeText(address);

  settings.setNotification({
    show: true,
    type: 'success',
    message: i18n.global.t('wallets.account_address_copied_to_clipboard'),
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

.account-card {
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
