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
        <AuthCheck :privileges="[Privilege.ListRequests]">
          <RecentRequests
            class="mb-4"
            :see-all-link="{
              name: Routes.Requests,
              query: { group_by: RequestDomains.Accounts },
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
            data-test-id="accounts-table"
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

            <template #item.symbol="{ item: account }">
              <div class="d-flex align-center text-no-wrap">
                <AccountAssetsCell :asset-ids="account.assets.map(a => a.asset_id)" />
              </div>
            </template>
            <template #item.name="{ item: account }">
              {{ account.name }}

              <VTooltip v-if="account.id == sourceCycleAccount" location="bottom">
                <template #activator="{ props: tooltipProps }">
                  <VIcon :icon="mdiCashSync" class="ml-2 pb-1" v-bind="tooltipProps"></VIcon>
                </template>
                {{ $t('pages.accounts.cycle_obtain_account') }}
              </VTooltip>
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
import { mdiCashSync, mdiChevronRight } from '@mdi/js';
import { computed, onMounted, ref } from 'vue';
import { useI18n } from 'vue-i18n';
import { VDataTable, VIcon, VPagination } from 'vuetify/components';
import AuthCheck from '~/components/AuthCheck.vue';
import DataLoader from '~/components/DataLoader.vue';
import PageLayout from '~/components/PageLayout.vue';
import AccountAssetsCell from '~/components/accounts/AccountAssetsCell.vue';
import AccountSetupAction from '~/components/accounts/AccountSetupAction.vue';
import PageBody from '~/components/layouts/PageBody.vue';
import PageHeader from '~/components/layouts/PageHeader.vue';
import RecentRequests from '~/components/requests/RecentRequests.vue';
import { useFetchList, usePagination } from '~/composables/lists.composable';
import { Routes } from '~/configs/routes.config';
import { UUID } from '~/generated/control-panel/control_panel.did';
import { Account, AccountCallerPrivileges } from '~/generated/station/station.did';
import { useAppStore } from '~/stores/app.store';
import { useStationStore } from '~/stores/station.store';
import type { PageProps, TableHeader } from '~/types/app.types';
import { Privilege } from '~/types/auth.types';
import { RequestDomains } from '~/types/station.types';
import { hasRequiredPrivilege } from '~/utils/auth.utils';
import { throttle, unreachable, variantIs } from '~/utils/helper.utils';

const props = withDefaults(defineProps<PageProps>(), { title: undefined, breadcrumbs: () => [] });
const i18n = useI18n();
const station = useStationStore();
const app = useAppStore();
const pageTitle = computed(() => props.title || i18n.t('pages.accounts.title'));
const pagination = usePagination();
const forceReload = ref(false);
const headers = computed<TableHeader[]>(() => {
  if (app.isMobile) {
    return [
      { title: i18n.t('terms.name'), key: 'name', sortable: false },
      { title: i18n.t('terms.token'), key: 'symbol', sortable: false },
      { title: '', key: 'actions', sortable: false, headerProps: { class: 'w-0' } },
    ];
  }

  return [
    { title: i18n.t('terms.name'), key: 'name', sortable: false },
    { title: i18n.t('terms.token'), key: 'symbol', sortable: false },
    { title: '', key: 'actions', sortable: false, headerProps: { class: 'w-0' } },
  ];
});

const sourceCycleAccount = ref<UUID | undefined>();

let useVerifiedCall = false;
const triggerSearch = throttle(() => (forceReload.value = true), 500);
const accounts = ref<Account[]>([]);
const privileges = ref<AccountCallerPrivileges[]>([]);
const fetchList = useFetchList(
  async (offset, limit) => {
    const results = await station.service.listAccounts(
      {
        offset,
        limit,
      },
      useVerifiedCall,
    );

    useVerifiedCall = true;

    station.trackAccountsBalance(results.accounts.map(account => account.id));
    return results;
  },
  {
    pagination,
    getTotal: res => Number(res.total),
  },
);

onMounted(async () => {
  if (hasRequiredPrivilege({ anyOf: [Privilege.SystemInfo] })) {
    try {
      const systemInfo = (await station.service.systemInfo()).system;

      if (variantIs(systemInfo.cycle_obtain_strategy, 'MintFromNativeToken')) {
        sourceCycleAccount.value = systemInfo.cycle_obtain_strategy.MintFromNativeToken.account_id;
      } else if (variantIs(systemInfo.cycle_obtain_strategy, 'WithdrawFromCyclesLedger')) {
        sourceCycleAccount.value =
          systemInfo.cycle_obtain_strategy.WithdrawFromCyclesLedger.account_id;
      } else if (variantIs(systemInfo.cycle_obtain_strategy, 'Disabled')) {
        // do nothing
      } else {
        unreachable(systemInfo.cycle_obtain_strategy);
      }
    } catch (e: unknown) {
      app.sendErrorNotification(e);
    }
  }
});
</script>
