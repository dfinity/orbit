<template>
  <PageLayout>
    <template #main-header>
      <PageHeader :title="pageTitle" :breadcrumbs="props.breadcrumbs">
        <template #actions>
          <AuthCheck :privileges="[Privilege.AddAccount]">
            <AccountSetupAction :text="$t('terms.new_account')" />
          </AuthCheck>
        </template>
      </PageHeader>
    </template>
    <template #main-body>
      <PageBody>
        <AuthCheck :privileges="[Privilege.ListProposals]">
          <RecentProposals
            class="mb-4"
            :see-all-link="{
              name: Routes.Proposals,
              query: { group_by: ProposalDomains.Accounts },
            }"
            :types="[{ AddAccount: null }, { EditAccount: null }]"
            hide-not-found
          />
        </AuthCheck>

        <DataLoader
          v-slot="{ loading }"
          v-model:force-reload="forceReload"
          :load="fetchList"
          :refresh-interval-ms="5000"
          @loaded="
            result => {
              accounts = result.accounts;
              privileges = result.privileges;
            }
          "
        >
          <VDataTable
            class="elevation-2 rounded"
            :loading="loading"
            :headers="headers"
            :items="accounts"
            :items-per-page="-1"
            :hover="true"
            @click:row="
              (_: unknown, { item }: any) => {
                $router.push({ name: Routes.Account, params: { id: item.id } });
              }
            "
          >
            <template #bottom>
              <!--this hides the footer as pagination is not required-->
            </template>
            <template #header.balance="{ column }">
              <div class="d-flex justify-end">
                {{ column.title }}
              </div>
            </template>
            <template #item.balance="{ item: account }">
              <div class="d-flex justify-end align-center text-no-wrap">
                {{
                  account.balance?.[0]
                    ? `${formatBalance(account.balance[0].balance, account.balance[0].decimals)} ${
                        account.symbol
                      }`
                    : '-'
                }}
              </div>
            </template>
            <template #item.address="{ item: account }">
              <div class="d-flex align-center flex-no-wrap">
                <TextOverflow :max-length="app.isMobile ? 16 : 32" :text="account.address" />
                <VBtn
                  size="x-small"
                  variant="text"
                  :icon="mdiContentCopy"
                  @click.stop="
                    copyToClipboard({
                      textToCopy: account.address,
                      sendNotification: true,
                    })
                  "
                />
              </div>
            </template>
            <template #item.actions>
              <div class="d-flex justify-end">
                <VIcon :icon="mdiChevronRight" size="large" />
              </div>
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
import { mdiChevronRight, mdiContentCopy } from '@mdi/js';
import { computed, ref } from 'vue';
import { useI18n } from 'vue-i18n';
import { VBtn, VDataTable, VIcon, VPagination } from 'vuetify/components';
import AuthCheck from '~/components/AuthCheck.vue';
import DataLoader from '~/components/DataLoader.vue';
import PageLayout from '~/components/PageLayout.vue';
import TextOverflow from '~/components/TextOverflow.vue';
import AccountSetupAction from '~/components/accounts/AccountSetupAction.vue';
import PageBody from '~/components/layouts/PageBody.vue';
import PageHeader from '~/components/layouts/PageHeader.vue';
import RecentProposals from '~/components/proposals/RecentProposals.vue';
import { useFetchList, usePagination } from '~/composables/lists.composable';
import { Routes } from '~/configs/routes.config';
import { Account, AccountCallerPrivileges } from '~/generated/station/station.did';
import { useAppStore } from '~/stores/app.store';
import { useWalletStore } from '~/stores/wallet.store';
import type { PageProps, TableHeader } from '~/types/app.types';
import { Privilege } from '~/types/auth.types';
import { ProposalDomains } from '~/types/wallet.types';
import { copyToClipboard } from '~/utils/app.utils';
import { formatBalance, throttle } from '~/utils/helper.utils';

const props = withDefaults(defineProps<PageProps>(), { title: undefined, breadcrumbs: () => [] });
const i18n = useI18n();
const wallet = useWalletStore();
const app = useAppStore();
const pageTitle = computed(() => props.title || i18n.t('pages.accounts.title'));
const pagination = usePagination();
const forceReload = ref(false);
const headers = computed<TableHeader[]>(() => {
  if (app.isMobile) {
    return [
      { title: i18n.t('terms.name'), key: 'name', sortable: false },
      { title: i18n.t('terms.token'), key: 'symbol', sortable: false },
      { title: i18n.t('terms.balance'), key: 'balance', sortable: false },
      { title: '', key: 'actions', sortable: false, headerProps: { class: 'w-0' } },
    ];
  }

  return [
    { title: i18n.t('terms.name'), key: 'name', sortable: false },
    { title: i18n.t('terms.token'), key: 'symbol', sortable: false },
    { title: i18n.t('terms.address'), key: 'address', sortable: false },
    { title: i18n.t('terms.balance'), key: 'balance', sortable: false },
    { title: '', key: 'actions', sortable: false, headerProps: { class: 'w-0' } },
  ];
});

let useVerifiedCall = false;
const triggerSearch = throttle(() => (forceReload.value = true), 500);
const accounts = ref<Account[]>([]);
const privileges = ref<AccountCallerPrivileges[]>([]);
const fetchList = useFetchList(
  async (offset, limit) => {
    const results = await wallet.service.listAccounts(
      {
        offset,
        limit,
      },
      useVerifiedCall,
    );

    useVerifiedCall = true;

    wallet.trackAccountsBalance(results.accounts.map(account => account.id));
    return results;
  },
  {
    pagination,
    getTotal: res => Number(res.total),
  },
);
</script>
