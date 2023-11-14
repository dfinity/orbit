<template>
  <PageLayout>
    <template #main-header>
      <VContainer class="pt-16 pb-16 pl-8 pr-8" fluid>
        <VRow>
          <VCol md="6" sm="12">
            <h1 class="text-h4">{{ $t('home.welcome_back') }}</h1>
            <p v-if="activeWallet.hasUser" class="info-box">
              <VIcon :icon="mdiBellRing" size="18" class="mr-1" />
              <span>
                {{
                  activeWallet.metrics.notifications > 0
                    ? $t('home.notifications.some', { count: activeWallet.metrics.notifications })
                    : $t('home.notifications.none')
                }}
              </span>
            </p>
          </VCol>
          <VCol md="6" sm="12" class="header-actions">
            <NewTransferBtn />
            <VBtn
              rounded
              color="primary-variant"
              :prepend-icon="mdiCogs"
              :to="`/${$route.params.locale}/settings`"
            >
              {{ $t('terms.settings') }}
            </VBtn>
          </VCol>
        </VRow>
      </VContainer>
    </template>
    <template #main-body>
      <VContainer class="pl-8 pr-8" fluid>
        <VRow>
          <VCol v-if="!activeWallet.hasUser" cols="12" md="4">
            <VCard color="surface" height="100%">
              <VCardTitle>{{ $t('wallets.no_wallet_user') }}</VCardTitle>
              <VCardSubtitle>{{ $t('wallets.please_register_to_continue') }}</VCardSubtitle>
              <VCardText class="text-center mt-6 mb-6">
                <VIcon :icon="mdiWallet" size="64" />
              </VCardText>
            </VCard>
          </VCol>
          <VCol v-if="activeWallet.hasUser" cols="12" md="4">
            <VCard color="surface" height="100%" :loading="activeWallet.accounts.loading">
              <VCardTitle>{{ $t('terms.accounts') }}</VCardTitle>
              <VCardText class="text-center text-h3 pt-8 pb-16">
                {{ activeWallet.metrics.accounts }}
              </VCardText>
            </VCard>
          </VCol>
          <VCol v-if="activeWallet.hasUser" cols="12" md="4">
            <VCard color="surface" height="100%" :loading="activeWallet.notifications.loading">
              <VCardTitle>{{ $t('wallets.pending_proposals') }}</VCardTitle>
              <VCardText class="text-center text-h3 pt-8 pb-16">
                {{ activeWallet.metrics.notifications }}
              </VCardText>
            </VCard>
          </VCol>
        </VRow>
      </VContainer>
    </template>
  </PageLayout>
</template>

<script lang="ts" setup>
import { mdiWallet, mdiBellRing, mdiCogs } from '@mdi/js';
import PageLayout from '~/ui/components/PageLayout.vue';
import { useActiveWalletStore } from '~/ui/stores';
import NewTransferBtn from '~/ui/components/NewTransferBtn.vue';

const activeWallet = useActiveWalletStore();
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
