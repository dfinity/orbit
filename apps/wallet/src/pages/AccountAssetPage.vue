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
          <template #subtitle>
            <div v-for="accountAddress in addresses" :key="accountAddress.address">
              <small
                ><VChip
                  size="x-small"
                  class="mr-2"
                  :text="
                    $t(`blockchains.${accountAddress.blockchain}.formats.${accountAddress.format}`)
                  "
                ></VChip>
                <ShortenedAddress
                  :address="accountAddress.address"
                  :format="accountAddress.format as AddressFormat"
                ></ShortenedAddress>
              </small>
              <VBtn
                size="x-small"
                variant="text"
                :icon="mdiContentCopy"
                @click="
                  copyToClipboard({
                    textToCopy: accountAddress.address,
                    sendNotification: true,
                  })
                "
              />
            </div>
          </template>
          <template v-if="asset" #actions>
            <BatchTransfersActionBtn
              v-if="privileges.can_transfer"
              :account="account"
              variant="outlined"
              :asset="asset"
            />
            <TransferBtn
              v-if="privileges.can_transfer"
              :account="account"
              color="primary"
              :asset="asset"
            >
              + {{ $t('pages.accounts.btn_new_transfer') }}
            </TransferBtn>

            <VMenu v-if="privileges.can_edit">
              <template #activator="{ props: activatorProps }">
                <VBtn
                  :icon="mdiDotsVertical"
                  color="primary-variant"
                  density="comfortable"
                  v-bind="activatorProps"
                >
                </VBtn>
              </template>
              <VList>
                <VListItem
                  v-if="privileges.can_edit"
                  :key="account.id"
                  color="primary"
                  variant="tonal"
                  link
                  :prepend-icon="mdiDelete"
                  @click="removeAssetDialog = true"
                >
                  <VListItemTitle>{{ $t('pages.account.remove_asset') }}</VListItemTitle>
                </VListItem>
              </VList>
            </VMenu>
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
              query: { group_by: RequestDomains.Transfers },
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
                    v-if="
                      chainApi && chainApi.getCapabilities().includes(ChainApiCapability.Transfers)
                    "
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
                            {{ $t('app.amount_token', { token: asset?.symbol || '' }) }}
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
                              <ShortenedAddress
                                v-if="asset"
                                :address="
                                  isReceivedTransfer(transfer) ? transfer.from : transfer.to
                                "
                                :format="transfer.format"
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
                            {{ asset ? formatBalance(transfer.amount, asset.decimals) : '' }}
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

                  <VAlert
                    type="warning"
                    variant="tonal"
                    density="compact"
                    class="mt-4"
                    v-else-if="
                      chainApi && !chainApi.getCapabilities().includes(ChainApiCapability.Transfers)
                    "
                  >
                    {{ $t('pages.account.transfers_not_supported') }}

                    <div v-if="asset?.blockchain === BlockchainType.InternetComputer">
                      {{ $t('pages.account.add_index_canister_to_see_transactions') }}
                    </div>
                  </VAlert>
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

  <RemoveAssetDialog
    v-if="account && asset"
    v-model:open="removeAssetDialog"
    :account="account"
    :asset="asset.id"
  ></RemoveAssetDialog>
</template>

<script lang="ts" setup>
import {
  mdiArrowDown,
  mdiArrowUp,
  mdiCalendar,
  mdiContentCopy,
  mdiDelete,
  mdiDotsVertical,
  mdiFilter,
} from '@mdi/js';
import { computed, ref, watch } from 'vue';
import { useRouter } from 'vue-router';
import {
  VAlert,
  VBtn,
  VChip,
  VCol,
  VContainer,
  VDivider,
  VIcon,
  VList,
  VListItem,
  VListItemTitle,
  VMenu,
  VProgressCircular,
  VRow,
  VSpacer,
  VTable,
} from 'vuetify/components';
import DataLoader from '~/components/DataLoader.vue';
import PageLayout from '~/components/PageLayout.vue';
import ShortenedAddress from '~/components/ShortenedAddress.vue';
import BatchTransfersActionBtn from '~/components/accounts/BatchTransfersActionBtn.vue';
import RemoveAssetDialog from '~/components/accounts/RemoveAssetDialog.vue';
import TransferBtn from '~/components/accounts/TransferBtn.vue';
import DateRange from '~/components/inputs/DateRange.vue';
import PageBody from '~/components/layouts/PageBody.vue';
import PageHeader from '~/components/layouts/PageHeader.vue';
import RecentRequests from '~/components/requests/RecentRequests.vue';
import FiltersCard from '~/components/ui/FiltersCard.vue';
import { useFilterUtils, useSavedFilters } from '~/composables/account.composable';
import { Routes } from '~/configs/routes.config';
import logger from '~/core/logger.core';
import {
  Account,
  AccountAsset,
  AccountCallerPrivileges,
  Asset,
} from '~/generated/station/station.did';
import { ChainApiFactory } from '~/services/chains';
import { useStationStore } from '~/stores/station.store';
import type { PageProps } from '~/types/app.types';
import {
  AddressFormat,
  BlockchainType,
  ChainApiCapability,
  type AccountIncomingTransfer,
} from '~/types/chain.types';
import { BreadCrumbItem } from '~/types/navigation.types';
import { RequestDomains } from '~/types/station.types';
import { copyToClipboard } from '~/utils/app.utils';
import { detectAddressFormat } from '~/utils/asset.utils';
import { convertDate } from '~/utils/date.utils';
import { formatBalance, throttle } from '~/utils/helper.utils';

const props = withDefaults(defineProps<PageProps>(), {
  title: undefined,
  breadcrumbs: () => [],
});
const router = useRouter();
const pageTitle = computed(() => {
  if (asset.value && accountAsset.value && accountAsset.value.balance[0]) {
    return (
      formatBalance(accountAsset.value.balance[0].balance, accountAsset.value.balance[0].decimals) +
      ' ' +
      asset.value.symbol
    );
  }

  return '-';
});
const forceReload = ref(false);
const disableRefresh = ref(false);
const account = ref<Account | null>(null);
const accountAsset = ref<AccountAsset | null>(null);
const asset = ref<Asset | null>(null);
const addresses = ref<
  | {
      address: string;
      standard: string;
      format: string;
      blockchain: string;
    }[]
  | null
>(null);
const privileges = ref<AccountCallerPrivileges>({
  id: '',
  can_edit: false,
  can_transfer: false,
});
const loading = ref(false);
const removeAssetDialog = ref(false);
const station = useStationStore();
const triggerSearch = throttle(() => (forceReload.value = true), 500);
const pageBreadcrumbs = computed<BreadCrumbItem[]>(() => {
  const breadcrumbs = [...props.breadcrumbs];

  if (account.value) {
    breadcrumbs.pop();
    breadcrumbs.push({
      title: account.value.name,
      to: { name: Routes.Account, params: { id: account.value.id } },
    });

    if (asset.value) {
      breadcrumbs.push({
        title: asset.value.name + ' (' + asset.value.symbol + ')',
      });
    }
  }

  return breadcrumbs;
});
const filters = useSavedFilters();
const filterUtils = useFilterUtils();
const saveFilters = (): void => {
  router.replace({ query: filterUtils.getQuery(filters.value) });
};

const chainApi = computed(() => {
  if (!account.value || !asset.value) {
    return null;
  }
  return ChainApiFactory.create(asset.value, account.value.addresses);
});

watch(
  () => filters.value,
  () => {
    saveFilters();
    triggerSearch();
  },
  { deep: true },
);

const isReceivedTransfer = (transfer: AccountIncomingTransfer): boolean => {
  return !!addresses.value?.some(address => address.address === transfer.to);

  // return transfer.to === account.value?.address;
};

const loadTransfers = async (): Promise<
  (AccountIncomingTransfer & { format: string | undefined })[]
> => {
  if (
    !account.value ||
    !accountAsset.value ||
    !asset.value ||
    !addresses.value ||
    !addresses.value[0] ||
    !chainApi.value
  ) {
    return [];
  }
  // const firstAddress = addresses.value[0];
  const transfers = await chainApi.value.fetchTransfers({
    fromDt: convertDate(filters.value.created.from, {
      time: 'start-of-day',
      tz: 'local',
    }),
    toDt: convertDate(filters.value.created.to, {
      time: 'end-of-day',
      tz: 'local',
    }),
  });

  return transfers.map(transfer => ({
    ...transfer,
    format: detectAddressFormat(
      asset.value!.blockchain,
      isReceivedTransfer(transfer) ? transfer.from : transfer.to,
    ),
  }));
};

let useVerifiedCall = false;

const loadAccount = async (): Promise<{
  account: Account;
  privileges: AccountCallerPrivileges;
}> => {
  const accountId = router.currentRoute.value.params.id.toString();
  const result = await station.service.getAccount({ account_id: accountId }, useVerifiedCall);
  useVerifiedCall = true;

  const account = result.account;

  const assetId = router.currentRoute.value.params.assetId.toString();
  const maybeAccountAsset = account.assets.find(accountAsset => accountAsset.asset_id === assetId);

  if (!maybeAccountAsset) {
    logger.error('Account asset not found', { accountId, assetId });
    throw new Error('Account asset not found');
  }

  accountAsset.value = maybeAccountAsset;

  const maybeAsset = station.configuration.details.supported_assets.find(
    token => token.id === assetId,
  );

  if (!maybeAsset) {
    logger.error('Asset not found', { assetId });
    throw new Error('Asset not found');
  }

  asset.value = maybeAsset;

  const supportedFormats = asset.value.standards
    .map(
      standard =>
        station.configuration.details.supported_blockchains
          .find(b => b.blockchain === maybeAsset.blockchain)
          ?.supported_standards.find(s => s.standard === standard)
          ?.supported_address_formats.map(format => ({
            format,
            standard,
          })) || [],
    )
    .flat();

  // collect all addresses
  const formats = asset.value.standards
    .map(standard => supportedFormats.find(sf => sf.standard === standard)!)
    .filter(Boolean);

  addresses.value = account.addresses
    .filter(account_address => formats.some(f => f.format == account_address.format))
    .map(account_address => ({
      address: account_address.address,
      standard: formats.find(f => f.format === account_address.format)!.standard,
      format: account_address.format,
      blockchain: maybeAsset.blockchain,
    }));

  if (!accountAsset.value.balance.length) {
    const balances = await station.service.fetchAccountBalances({
      account_ids: [accountId],
    });

    if (balances.length) {
      accountAsset.value = {
        ...accountAsset.value,
        balance: [
          {
            account_id: accountId,
            balance: balances[0].balance,
            decimals: balances[0].decimals,
            last_update_timestamp: balances[0].last_update_timestamp,
          },
        ],
      };
    }
  }

  station.trackAccountsBalance([account.id]);
  return { account, privileges: result.privileges };
};
</script>
