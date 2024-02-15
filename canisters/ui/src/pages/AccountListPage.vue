<template>
  <PageLayout>
    <template #main-header>
      <PageHeader :title="$t('terms.accounts')" :breadcrumbs="props.breadcrumbs.value">
        <template #actions>
          <AuthCheck :privileges="[Privilege.AddAccount]">
            <AccountConfigBtn :text="$t('terms.new_account')" variant="outlined" />
          </AuthCheck>
        </template>
      </PageHeader>
    </template>
    <template #main-body>
      <PageBody>
        <DataLoader
          v-slot="{ loading }"
          :load="fetchAccounts"
          :refresh-interval-ms="5000"
          @loaded="results = $event"
        >
          <VDataTable
            :loading="loading"
            :headers="headers"
            :items="accounts"
            :items-per-page="-1"
            :hover="true"
          >
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
        </DataLoader>
        <VPagination
          v-model="pagination.selectedPage"
          class="mt-2"
          :length="pagination.totalPages"
          rounded
          density="comfortable"
          @update:model-value="triggerSearch"
        />
      </PageBody>
    </template>
  </PageLayout>
</template>

<script lang="ts" setup>
import { mdiContentCopy, mdiOpenInApp } from '@mdi/js';
import { toRefs } from 'vue';
import { computed, ref } from 'vue';
import { useI18n } from 'vue-i18n';
import AuthCheck from '~/components/AuthCheck.vue';
import DataLoader from '~/components/DataLoader.vue';
import PageLayout from '~/components/PageLayout.vue';
import AccountConfigBtn from '~/components/accounts/AccountConfigBtn.vue';
import PageBody from '~/components/layouts/PageBody.vue';
import PageHeader from '~/components/layouts/PageHeader.vue';
import { Routes } from '~/configs/routes.config';
import { Account } from '~/generated/wallet/wallet.did';
import { useWalletStore } from '~/stores/wallet.store';
import { Privilege } from '~/types/auth.types';
import { BreadCrumbItem } from '~/types/navigation.types';
import { copyToClipboard } from '~/utils/app.utils';
import { formatBalance, throttle } from '~/utils/helper.utils';

const input = withDefaults(
  defineProps<{
    breadcrumbs?: BreadCrumbItem[];
  }>(),
  {
    breadcrumbs: () => [],
  },
);
const props = toRefs(input);
const i18n = useI18n();
const wallet = useWalletStore();

const headers = ref<{ title: string; key: string; sortable?: boolean }[]>([
  { title: i18n.t('terms.name'), key: 'name', sortable: false },
  { title: i18n.t('terms.token'), key: 'token', sortable: false },
  { title: i18n.t('terms.address'), key: 'address', sortable: false },
  { title: i18n.t('terms.balance'), key: 'balance', sortable: false },
  { title: '', key: 'actions', sortable: false },
]);

const results = ref<Account[]>([]);

const accounts = computed(() => {
  return results.value.map(account => {
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

const forceReload = ref(false);
const pagination = ref<{
  limit: number;
  totalPages: number;
  selectedPage: number;
}>({
  limit: 25,
  totalPages: 1,
  selectedPage: 1,
});

const triggerSearch = throttle(() => {
  forceReload.value = true;
}, 500);

const fetchAccounts = async (): Promise<Account[]> => {
  const { limit, selectedPage } = pagination.value;
  const offset = (selectedPage - 1) * limit;
  const { accounts, total } = await wallet.service.listAccounts({ limit, offset });

  pagination.value = {
    ...pagination.value,
    totalPages: Math.ceil(Number(total) / limit),
  };

  wallet.trackAccountsBalance(accounts.map(account => account.id));
  return accounts;
};
</script>
