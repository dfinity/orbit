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
              color="default"
              variant="tonal"
              :readonly="!privileges.can_edit"
              :append-icon="mdiTuneVariant"
            >
              {{ $t('terms.settings') }}
            </AccountSetupAction>
          </template>
          <template #subtitle> </template>
          <template v-if="privileges.can_edit" #actions>
            <AddAccountAssetBtn :account="account" color="primary">
              + {{ $t('pages.account.add_asset') }}
            </AddAccountAssetBtn>
          </template>
        </PageHeader>
      </template>
      <template v-if="!loading" #main-body>
        <PageBody v-if="!account">{{ $t('pages.account.not_found_description') }}</PageBody>
        <PageBody v-else>
          <RecentRequests
            class="mb-4"
            :see-all-link="{
              name: Routes.Requests,
              query: { group_by: RequestDomains.Accounts },
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
                  <VDataTable
                    class="elevation-2 rounded"
                    :loading="loading"
                    :headers="headers"
                    :items="account.assets"
                    :items-per-page="-1"
                    :hover="true"
                    @click:row="
                      (_: unknown, { item }: any) => {
                        $router.push({
                          name: Routes.AccountAsset,
                          params: { assetId: item.asset_id },
                        });
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
                    <template #item.address="{ item: account_asset }">
                      <div
                        v-for="account_address in assetAddresses(account_asset.asset_id)"
                        :key="account_address.address"
                      >
                        <small>
                          <ShortenedAddress
                            :address="account_address.address"
                            :format="account_address.format"
                          />
                        </small>
                      </div>
                    </template>

                    <template #item.symbol="{ item: asset }">
                      <div class="d-flex align-center text-no-wrap">
                        {{ assetById(asset.asset_id)?.symbol || '-' }}
                      </div>
                    </template>
                    <template #item.name="{ item: asset }">
                      {{ assetById(asset.asset_id)?.name || 'Unknown asset' }}
                    </template>
                    <template #item.balance="{ item: asset }">
                      <div class="d-flex justify-end">
                        {{
                          asset.balance[0]
                            ? formatBalance(asset.balance[0].balance, asset.balance[0].decimals)
                            : ''
                        }}
                      </div>
                    </template>

                    <template #item.actions>
                      <div class="d-flex justify-end">
                        <VIcon :icon="mdiChevronRight" size="large" />
                      </div>
                    </template>
                  </VDataTable>
                </div>
                <FiltersCard :title="$t('terms.filters')" :icon="mdiFilter">
                  <DateRange
                    v-model="filters.created"
                    :label="$t('terms.created')"
                    :prepend-icon="mdiCalendar"
                  />
                  <VSpacer />
                  <VDivider thickness="2" class="my-2" />
                  <VBtn
                    density="default"
                    color="primary-variant"
                    flat
                    size="small"
                    variant="tonal"
                    @click="filters = filterUtils.getDefaultFilters()"
                  >
                    {{ $t('terms.reset') }}
                  </VBtn>
                </FiltersCard>
              </VCol>
            </VRow>
          </VContainer>
        </PageBody>
      </template>
    </PageLayout>
  </DataLoader>
</template>

<script lang="ts" setup>
import { mdiCalendar, mdiChevronRight, mdiFilter, mdiTuneVariant } from '@mdi/js';
import { computed, ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import { useRouter } from 'vue-router';
import {
  VBtn,
  VCol,
  VContainer,
  VDataTable,
  VDivider,
  VIcon,
  VProgressCircular,
  VRow,
  VSpacer,
} from 'vuetify/components';
import DataLoader from '~/components/DataLoader.vue';
import PageLayout from '~/components/PageLayout.vue';
import ShortenedAddress from '~/components/ShortenedAddress.vue';
import AccountSetupAction from '~/components/accounts/AccountSetupAction.vue';
import AddAccountAssetBtn from '~/components/accounts/AddAccountAssetBtn.vue';
import DateRange from '~/components/inputs/DateRange.vue';
import PageBody from '~/components/layouts/PageBody.vue';
import PageHeader from '~/components/layouts/PageHeader.vue';
import RecentRequests from '~/components/requests/RecentRequests.vue';
import FiltersCard from '~/components/ui/FiltersCard.vue';
import { useFilterUtils, useSavedFilters } from '~/composables/account.composable';
import { Routes } from '~/configs/routes.config';
import {
  Account,
  AccountAddress,
  AccountCallerPrivileges,
  Asset,
} from '~/generated/station/station.did';
import { useAppStore } from '~/stores/app.store';
import { useStationStore } from '~/stores/station.store';
import type { PageProps, TableHeader } from '~/types/app.types';
import { BreadCrumbItem } from '~/types/navigation.types';
import { RequestDomains } from '~/types/station.types';
import { formatBalance, throttle } from '~/utils/helper.utils';
const i18n = useI18n();
const app = useAppStore();
const headers = computed<TableHeader[]>(() => {
  if (app.isMobile) {
    return [
      { title: i18n.t('terms.name'), key: 'name', sortable: false },
      { title: i18n.t('terms.symbol'), key: 'symbol', sortable: false },
      { title: i18n.t('terms.balance'), key: 'balance', sortable: false },
      { title: '', key: 'actions', sortable: false, headerProps: { class: 'w-0' } },
    ];
  }

  return [
    { title: i18n.t('terms.name'), key: 'name', sortable: false },
    { title: i18n.t('terms.balance'), key: 'balance', sortable: false },
    { title: i18n.t('terms.symbol'), key: 'symbol', sortable: false },
    { title: i18n.t('terms.address'), key: 'address', sortable: false },
    { title: '', key: 'actions', sortable: false, headerProps: { class: 'w-0' } },
  ];
});

const props = withDefaults(defineProps<PageProps>(), {
  title: undefined,
  breadcrumbs: () => [],
});
const router = useRouter();
const pageTitle = computed(() => {
  return account.value?.name;
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
const station = useStationStore();
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

const assetById = (assetId: string): Asset | undefined => {
  return station.configuration.details.supported_assets.find(token => token.id === assetId);
};

const assetAddresses = (assetId: string): AccountAddress[] => {
  const asset = assetById(assetId);

  if (!asset || !account.value) {
    return [];
  }

  const supportedFormats = new Set(
    station.configuration.details.supported_blockchains
      .find(b => b.blockchain === asset.blockchain)
      ?.supported_standards.filter(s => asset.standards.includes(s.standard))
      .map(s => s.supported_address_formats)
      .flat() || [],
  );

  return account.value.addresses.filter(account_address =>
    supportedFormats.has(account_address.format),
  );
};

watch(
  () => filters.value,
  () => {
    saveFilters();
    triggerSearch();
  },
  { deep: true },
);

let useVerifiedCall = false;

const loadAccount = async (): Promise<{
  account: Account;
  privileges: AccountCallerPrivileges;
}> => {
  const accountId = `${router.currentRoute.value.params.id}`;
  const result = await station.service.getAccount({ account_id: accountId }, useVerifiedCall);
  useVerifiedCall = true;

  const account = result.account;

  station.trackAccountsBalance([account.id]);
  return { account, privileges: result.privileges };
};
</script>
