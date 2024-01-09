<template>
  <PageLayout>
    <template #main-header>
      <VContainer class="pa-8" fluid>
        <VRow>
          <VCol cols="12">
            <h1 class="text-h4">{{ $t(`pages.overview.title`, { name: wallet.name }) }}</h1>
          </VCol>
        </VRow>
      </VContainer>
    </template>
    <template #main-body>
      <VContainer class="pl-8 pr-8" fluid>
        <VRow>
          <VCol v-if="!wallet.hasUser" cols="12" md="4">
            <VCard color="background" height="100%">
              <VCardTitle>{{ $t('wallets.no_wallet_user') }}</VCardTitle>
              <VCardSubtitle>{{ $t('wallets.please_register_to_continue') }}</VCardSubtitle>
              <VCardText class="text-center mt-6 mb-6">
                <VIcon :icon="mdiWallet" size="64" />
              </VCardText>
            </VCard>
          </VCol>
          <VCol v-if="wallet.hasUser" cols="12" md="4">
            <VCard color="background" height="100%" :loading="wallet.accounts.loading">
              <VCardTitle>{{ $t('terms.accounts') }}</VCardTitle>
              <VCardText class="text-center text-h3 pt-8 pb-16">
                {{ wallet.metrics.accounts }}
              </VCardText>
            </VCard>
          </VCol>
          <VCol v-if="wallet.hasUser" cols="12" md="4">
            <VCard color="background" height="100%" :loading="wallet.notifications.loading">
              <VCardTitle>{{ $t('wallets.pending_requests') }}</VCardTitle>
              <VCardText class="text-center text-h3 pt-8 pb-16">
                {{ wallet.metrics.notifications }}
              </VCardText>
            </VCard>
          </VCol>
        </VRow>
      </VContainer>
    </template>
  </PageLayout>
</template>

<script lang="ts" setup>
import { mdiWallet } from '@mdi/js';
import PageLayout from '~/ui/components/PageLayout.vue';
import { useWalletStore } from '~/ui/stores';

const wallet = useWalletStore();
</script>

<style scoped lang="scss">
.info-box {
  display: flex;
  justify-content: start;
  align-items: center;

  &__loading {
    width: 200px;
  }
}

.transfers {
  &__card {
    display: flex;
    justify-content: center;
    flex-direction: column;
    gap: var(--ds-bdu);

    &:first-child {
      border-right: var(--ds-border-width) var(--ds-border-style) rgb(var(--ds-background-border));
    }
  }
}

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
</style>
