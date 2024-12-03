<template>
  <PageLayout>
    <template #main-header>
      <PageHeader :title="pageTitle" :breadcrumbs="props.breadcrumbs" />
    </template>
    <template #main-body>
      <PageBody>
        <AuthCheck :privileges="[Privilege.ListRequests]">
          <RecentRequests
            class="mb-4"
            :see-all-link="{
              name: Routes.Requests,
              query: { group_by: RequestDomains.All },
            }"
            hide-not-found
          />
        </AuthCheck>

        <AuthCheck :privileges="[Privilege.ListAccounts]">
          <VCard>
            <VCardTitle class="flex-grow-1 pt-2 pt-3">
              {{ $t('pages.dashboard.available_assets') }}
            </VCardTitle>
            <VDivider />

            <VCardText class="px-0">
              <DataLoader
                v-model:force-reload="forceReload"
                :load="fetchList"
                :refresh-interval-ms="5000"
                @loaded="
                  result => {
                    assets = result.assets;
                    privileges = result.privileges;
                  }
                "
              >
                <VExpansionPanels flat>
                  <VExpansionPanel v-for="asset in assets" :key="asset.id">
                    <VExpansionPanelTitle>
                      <VRow>
                        <VCol :xs="6" :sm="4">
                          {{ asset.symbol }}
                          <div>
                            <small class="text-medium-emphasis">{{ asset.name }}</small>
                          </div>
                        </VCol>
                        <VCol :xs="6" :sm="4" class="d-none d-sm-flex align-center">
                          <VChip :size="'x-small'">
                            {{ $t(`blockchains.${asset.blockchain}.name`) }}
                          </VChip>
                        </VCol>
                        <VCol
                          :xs="6"
                          :sm="4"
                          class="text-right pr-8 d-flex align-center justify-end"
                        >
                          {{ formatBalance(asset.totalBalance, asset.decimals) }} {{ asset.symbol }}
                        </VCol>
                      </VRow>
                    </VExpansionPanelTitle>
                    <VExpansionPanelText>
                      <VDataTable
                        class="mt-2"
                        :headers="assetTableHeaders"
                        :items="asset.accountAssets"
                        :hover="true"
                        hide-default-footer
                        density="compact"
                        @click:row="
                          (_event: unknown, item: DatatableSlotItem) => {
                            $router.push({
                              name: Routes.AccountAsset,
                              params: {
                                id: item.item.account.id,
                                assetId: item.item.asset_id,
                              },
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
                        <template #item.balance="{ item: accountAsset }">
                          <div class="d-flex justify-end">
                            {{
                              formatBalance(
                                accountAsset.balance[0] ? accountAsset.balance[0].balance : 0n,
                                asset.decimals,
                              )
                            }}
                            {{ asset.symbol }}
                          </div>
                        </template>
                        <template #item.actions="{ item: accountAsset }">
                          <div class="d-none d-sm-flex justify-end">
                            <TransferBtn
                              v-if="
                                privileges.some(
                                  p => p.id === accountAsset.account.id && p.can_transfer,
                                )
                              "
                              :account="accountAsset.account"
                              :asset="asset"
                              size="x-small"
                              @click.stop=""
                            >
                              {{ $t('pages.accounts.btn_new_transfer') }}
                            </TransferBtn>
                          </div>
                        </template>
                      </VDataTable>
                    </VExpansionPanelText>
                  </VExpansionPanel>
                </VExpansionPanels>
              </DataLoader>
            </VCardText>
          </VCard>
        </AuthCheck>
      </PageBody>
    </template>
  </PageLayout>
</template>

<script lang="ts" setup>
import { computed, ref } from 'vue';
import { useI18n } from 'vue-i18n';
import { useDisplay } from 'vuetify';
import { VDataTable } from 'vuetify/components';
import AuthCheck from '~/components/AuthCheck.vue';
import DataLoader from '~/components/DataLoader.vue';
import PageLayout from '~/components/PageLayout.vue';
import TransferBtn from '~/components/accounts/TransferBtn.vue';
import PageBody from '~/components/layouts/PageBody.vue';
import PageHeader from '~/components/layouts/PageHeader.vue';
import RecentRequests from '~/components/requests/RecentRequests.vue';
import { Routes } from '~/configs/routes.config';
import { UUID } from '~/generated/control-panel/control_panel.did';
import {
  Account,
  AccountAsset,
  AccountCallerPrivileges,
  Asset,
} from '~/generated/station/station.did';
import { useStationStore } from '~/stores/station.store';
import type { PageProps } from '~/types/app.types';
import { Privilege } from '~/types/auth.types';
import { RequestDomains } from '~/types/station.types';
import { formatBalance } from '~/utils/helper.utils';

const props = withDefaults(defineProps<PageProps>(), { title: undefined, breadcrumbs: () => [] });
const i18n = useI18n();
const station = useStationStore();
const pageTitle = computed(() => props.title || i18n.t('pages.dashboard.title'));
const forceReload = ref(false);
const { xs } = useDisplay();

const assetTableHeaders = computed(() => [
  { title: i18n.t('terms.account'), key: 'account.name', sortable: false },
  { title: i18n.t('terms.balance'), key: 'balance', sortable: false },
  ...(xs.value
    ? []
    : [
        {
          title: '',
          key: 'actions',
          sortable: false,
          headerProps: { class: 'w-0' },
        },
      ]),
]);

// Vuetify does not expose this type, simplified version from the source code
type DatatableSlotItem = {
  item: AggregatedAsset['accountAssets'][0];
};

type AggregatedAsset = Asset & {
  totalBalance: bigint;
  accountAssets: (AccountAsset & {
    account: Account;
  })[];
};

let useVerifiedCall = false;
const assets = ref<AggregatedAsset[]>([]);
const privileges = ref<AccountCallerPrivileges[]>([]);
const fetchList = async () => {
  const results = await station.service.listAllAccounts(useVerifiedCall);

  useVerifiedCall = true;

  station.trackAccountsBalance(results.accounts.map(account => account.id));

  const assets: Record<UUID, AggregatedAsset> = {};

  for (const account of results.accounts) {
    for (const accountAsset of account.assets) {
      const asset = station.configuration.details.supported_assets.find(
        a => a.id == accountAsset.asset_id,
      );

      if (!asset) {
        continue;
      }

      if (!assets[asset.id]) {
        assets[asset.id] = {
          ...asset,
          totalBalance: 0n,
          accountAssets: [],
        };
      }
      assets[asset.id].accountAssets.push({ ...accountAsset, account: account });
      assets[asset.id].totalBalance += accountAsset.balance[0]
        ? accountAsset.balance[0].balance
        : 0n;
    }
  }

  return {
    privileges: results.privileges,
    assets: Object.keys(assets).map(assetId => assets[assetId]),
  };
};
</script>
