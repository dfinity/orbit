<template>
  <PageLayout>
    <template #main-header>
      <VContainer class="pa-8" fluid>
        <VRow>
          <VCol cols="12" md="6">
            <h1 class="text-h4">{{ $t('terms.accounts') }}</h1>
          </VCol>
          <VCol md="6" sm="12" class="header-actions">
            <AddAccountBtn @created="() => {}" />
          </VCol>
        </VRow>
      </VContainer>
    </template>
    <template #main-body>
      <VContainer class="pl-8 pr-8" fluid>
        <VRow>
          <VDataTable :headers="headers" :items="accounts" :items-per-page="-1" :hover="true">
            <!-- eslint-disable-next-line vue/valid-v-slot -->
            <template #item.actions="{ item }">
              <VBtn
                size="small"
                variant="tonal"
                :append-icon="mdiOpenInApp"
                :to="{ name: Routes.Account, params: { id: item.id } }"
              >
                {{ $t('terms.open') }}
              </VBtn>
            </template>
            <!-- eslint-disable-next-line vue/valid-v-slot -->
            <template #item.address="{ item }">
              <span>{{ item.address }}</span>
              <VBtn
                size="x-small"
                variant="text"
                :icon="mdiContentCopy"
                @click="
                  copyToClipboard({
                    textToCopy: item.address,
                    sendNotification: true,
                  })
                "
              />
            </template>
            <template #bottom>
              <!--this hides the footer as pagination is not required-->
            </template>
          </VDataTable>
        </VRow>
      </VContainer>
    </template>
  </PageLayout>
</template>

<script lang="ts" setup>
import { mdiContentCopy, mdiOpenInApp } from '@mdi/js';
import { computed, ref } from 'vue';
import { formatBalance } from '~/core';
import PageLayout from '~/ui/components/PageLayout.vue';
import AddAccountBtn from '~/ui/components/accounts/AddAccountBtn.vue';
import { i18n } from '~/ui/modules';
import { useWalletStore } from '~/ui/stores/wallet';
import { copyToClipboard } from '~/ui/utils';
import { Routes } from '~/ui/config/routes';

const wallet = useWalletStore();

const headers = ref<{ title: string; key: string }[]>([
  {
    title: i18n.global.t('terms.name'),
    key: 'name',
  },
  {
    title: i18n.global.t('terms.token'),
    key: 'token',
  },
  {
    title: i18n.global.t('terms.address'),
    key: 'address',
  },
  {
    title: i18n.global.t('terms.balance'),
    key: 'balance',
  },
  { title: '', key: 'actions' },
]);

const accounts = computed(() => {
  return wallet.accounts.items.map(account => {
    return {
      actions: null,
      id: account.id,
      name: account.name,
      address: account.address,
      token: account.symbol,
      balance: account.balance?.[0]
        ? `${formatBalance(account.balance[0].balance, account.balance[0].decimals)} ${
            account.symbol
          }`
        : '-',
    };
  });
});
</script>

<style scoped lang="scss">
.header-actions {
  display: flex;
  justify-content: end;
  align-items: center;
  gap: calc(var(--ds-bdu) * 2);
}

.page-layout--mobile {
  .header-actions {
    justify-content: center;

    :deep(.v-btn) {
      flex-grow: 1;
    }
  }
}
</style>
