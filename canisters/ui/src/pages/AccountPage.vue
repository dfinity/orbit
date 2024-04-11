<template>
  <DataLoader
    :load="loadAccount"
    :refresh-interval-ms="5000"
    :disable-refresh="disableRefresh"
    @loading="loading = $event"
    @loaded="
      result => {
        account = result.account;
        privileges = result.privileges;
      }
    "
  >
    <template #error>
      <PageLayout>
        <template #main-header>
          <PageHeader
            :title="$t('pages.accounts.error_fetching_account')"
            :breadcrumbs="pageBreadcrumbs"
          />
        </template>
      </PageLayout>
    </template>
    <PageLayout>
      <template #main-header>
        <div v-if="loading" class="d-flex justify-center">
          <VProgressCircular indeterminate color="primary" class="ma-8" />
        </div>
        <div v-else-if="!account">
          <PageHeader :title="$t('pages.account.not_found')" :breadcrumbs="pageBreadcrumbs" />
        </div>
        <PageHeader v-else :title="pageTitle" :breadcrumbs="pageBreadcrumbs">
          <template #title-toolbar>
            <AccountSetupAction
              :account-id="account.id"
              class="px-1 mb-2"
              size="small"
              variant="tonal"
              :readonly="!privileges.can_edit"
              :append-icon="mdiTuneVariant"
            >
              {{ $t('terms.settings') }}
            </AccountSetupAction>
          </template>
          <template #subtitle>
            <small><TextOverflow :max-length="32" :text="account.address" /></small>
            <VBtn
              size="x-small"
              variant="text"
              :icon="mdiContentCopy"
              @click="
                copyToClipboard({
                  textToCopy: account.address,
                  sendNotification: true,
                })
              "
            />
          </template>
          <template v-if="privileges.can_transfer" #actions>
            <BatchTransfersActionBtn :account="account" variant="outlined" />
            <TransferBtn :account="account" color="primary">
              + {{ $t('pages.accounts.btn_new_transfer') }}
            </TransferBtn>
          </template>
        </PageHeader>
      </template>
      <template v-if="!loading" #main-body>
        <PageBody v-if="!account">{{ $t('pages.account.not_found_description') }}</PageBody>
        <PageBody v-else>
          <RecentProposals
            class="mb-4"
            :see-all-link="{
              name: Routes.Proposals,
              query: { group_by: ProposalDomains.Transfers },
            }"
            :types="[{ Transfer: [account.id] }]"
            hide-not-found
          />
          <VContainer fluid class="px-3">
            <VRow>
              <VCol
                cols="12"
                class="d-flex flex-column-reverse flex-md-row ga-4 px-0 align-md-start pt-0"
              >
                <div class="d-flex flex-column flex-grow-1 ga-4">
                  <DataLoader
                    v-slot="{ data, loading: loadingTransfers }"
                    v-model:force-reload="forceReload"
                    :load="loadTransfers"
                    :refresh-interval-ms="10000"
                  >
                    <VProgressCircular v-if="loadingTransfers" indeterminate color="primary" />
                    <VTable v-else-if="data" hover class="elevation-2 rounded">
                      <thead>
                        <tr>
                          <th class="w-50 font-weight-bold">{{ $t('terms.time') }}</th>
                          <th class="text-no-wrap font-weight-bold">
                            {{ $t('app.destination_source') }}
                          </th>
                          <th class="text-no-wrap text-right font-weight-bold">
                            {{ $t('app.amount_token', { token: account.symbol }) }}
                          </th>
                        </tr>
                      </thead>
                      <tbody>
                        <tr v-if="!data.length">
                          <td colspan="4">{{ $t('app.no_transfers') }}</td>
                        </tr>
                        <tr v-for="(transfer, idx) in data" :key="idx">
                          <td>
                            {{
                              `${transfer.created_at?.toLocaleDateString()} ${transfer.created_at?.toLocaleTimeString()}`
                            }}
                          </td>
                          <td>
                            <div class="d-flex flex-row align-center">
                              <TextOverflow
                                :text="isReceivedTransfer(transfer) ? transfer.from : transfer.to"
                              />
                              <VBtn
                                size="x-small"
                                variant="text"
                                :icon="mdiContentCopy"
                                @click="
                                  copyToClipboard({
                                    textToCopy: isReceivedTransfer(transfer)
                                      ? transfer.from
                                      : transfer.to,
                                    sendNotification: true,
                                  })
                                "
                              />
                            </div>
                          </td>
                          <td class="d-flex flex-row ga-2 align-center justify-end">
                            {{ formatBalance(transfer.amount, account.decimals) }}
                            <VChip
                              size="x-small"
                              :color="isReceivedTransfer(transfer) ? 'success' : 'error'"
                            >
                              <VIcon
                                size="default"
                                :icon="isReceivedTransfer(transfer) ? mdiArrowDown : mdiArrowUp"
                              />
                            </VChip>
                          </td>
                        </tr>
                      </tbody>
                    </VTable>
                  </DataLoader>
                </div>
                <VCard
                  min-height="200px"
                  min-width="272px"
                  :max-width="!app.isMobile ? `272px` : undefined"
                >
                  <VToolbar color="transparent" class="pr-4">
                    <VToolbarTitle>{{ $t('terms.filters') }}</VToolbarTitle>
                    <VIcon :icon="mdiFilter" />
                  </VToolbar>
                  <VCardText class="pt-2">
                    <DateRange
                      v-model="filters.created"
                      :label="$t('terms.created')"
                      :prepend-icon="mdiCalendar"
                    />
                    <VDivider thickness="2" class="my-2" />
                    <VBtn
                      density="comfortable"
                      block
                      color="primary-variant"
                      flat
                      size="small"
                      variant="tonal"
                      @click="filters = filterUtils.getDefaultFilters()"
                    >
                      {{ $t('terms.reset') }}
                    </VBtn>
                  </VCardText>
                </VCard>
              </VCol>
            </VRow>
          </VContainer>
        </PageBody>
      </template>
    </PageLayout>
  </DataLoader>
</template>

<script lang="ts" setup>
import {
  mdiArrowDown,
  mdiArrowUp,
  mdiCalendar,
  mdiContentCopy,
  mdiFilter,
  mdiTuneVariant,
} from '@mdi/js';
import { computed, ref, watch } from 'vue';
import { useRouter } from 'vue-router';
import {
  VBtn,
  VCard,
  VCardText,
  VChip,
  VCol,
  VContainer,
  VDivider,
  VIcon,
  VProgressCircular,
  VRow,
  VTable,
  VToolbar,
  VToolbarTitle,
} from 'vuetify/components';
import DataLoader from '~/components/DataLoader.vue';
import PageLayout from '~/components/PageLayout.vue';
import TextOverflow from '~/components/TextOverflow.vue';
import AccountSetupAction from '~/components/accounts/AccountSetupAction.vue';
import BatchTransfersActionBtn from '~/components/accounts/BatchTransfersActionBtn.vue';
import TransferBtn from '~/components/accounts/TransferBtn.vue';
import DateRange from '~/components/inputs/DateRange.vue';
import PageBody from '~/components/layouts/PageBody.vue';
import PageHeader from '~/components/layouts/PageHeader.vue';
import RecentProposals from '~/components/proposals/RecentProposals.vue';
import { useFilterUtils, useSavedFilters } from '~/composables/account.composable';
import { Routes } from '~/configs/routes.config';
import { Account, AccountCallerPrivileges } from '~/generated/wallet/wallet.did';
import { ChainApiFactory } from '~/services/chains';
import { useAppStore } from '~/stores/app.store';
import { useWalletStore } from '~/stores/wallet.store';
import type { PageProps } from '~/types/app.types';
import type { AccountIncomingTransfer } from '~/types/chain.types';
import { BreadCrumbItem } from '~/types/navigation.types';
import { ProposalDomains } from '~/types/wallet.types';
import { copyToClipboard } from '~/utils/app.utils';
import { convertDate } from '~/utils/date.utils';
import { formatBalance, throttle } from '~/utils/helper.utils';

const props = withDefaults(defineProps<PageProps>(), {
  title: undefined,
  breadcrumbs: () => [],
});
const router = useRouter();
const pageTitle = computed(() => {
  if (account.value && account.value.balance[0]) {
    return (
      formatBalance(account.value.balance[0].balance, account.value.balance[0].decimals) +
      ' ' +
      account.value.symbol
    );
  }

  return '-';
});
const forceReload = ref(false);
const disableRefresh = ref(false);
const account = ref<Account | null>(null);
const privileges = ref<AccountCallerPrivileges>({
  id: '',
  can_edit: false,
  can_transfer: false,
});
const loading = ref(false);
const app = useAppStore();
const wallet = useWalletStore();
const triggerSearch = throttle(() => (forceReload.value = true), 500);
const pageBreadcrumbs = computed<BreadCrumbItem[]>(() => {
  const breadcrumbs = [...props.breadcrumbs];

  if (account.value) {
    breadcrumbs.push({
      title: account.value.name,
    });
  }

  return breadcrumbs;
});
const filters = useSavedFilters();
const filterUtils = useFilterUtils();
const saveFilters = (): void => {
  router.replace({ query: filterUtils.getQuery(filters.value) });
};

watch(
  () => filters.value,
  () => {
    saveFilters();
    triggerSearch();
  },
  { deep: true },
);

const isReceivedTransfer = (transfer: AccountIncomingTransfer): boolean => {
  return transfer.to === account.value?.address;
};

const loadTransfers = async (): Promise<AccountIncomingTransfer[]> => {
  if (!account.value) {
    return [];
  }

  const chainApi = ChainApiFactory.create(account.value);
  const transfers = await chainApi.fetchTransfers({
    fromDt: convertDate(filters.value.created.from, {
      time: 'start-of-day',
      tz: 'local',
    }),
    toDt: convertDate(filters.value.created.to, {
      time: 'end-of-day',
      tz: 'local',
    }),
  });

  return transfers;
};

const loadAccount = async (): Promise<{
  account: Account;
  privileges: AccountCallerPrivileges;
}> => {
  const accountId = `${router.currentRoute.value.params.id}`;
  const result = await wallet.service.getAccount({ account_id: accountId });
  const account = result.account;

  if (!account.balance.length) {
    const balances = await wallet.service.fetchAccountBalances({
      account_ids: [accountId],
    });

    if (balances.length) {
      account.balance = [
        {
          balance: balances[0].balance,
          decimals: balances[0].decimals,
          last_update_timestamp: balances[0].last_update_timestamp,
        },
      ];
    }
  }

  wallet.trackAccountsBalance([account.id]);
  return { account, privileges: result.privileges };
};
</script>
