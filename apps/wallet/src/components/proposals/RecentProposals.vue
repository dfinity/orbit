<template>
  <DataLoader
    v-model:force-reload="forceReload"
    :load="fetchRecentProposals"
    :error-msg="props.loadErrorMsg"
    :refresh-interval-ms="props.refreshIntervalMs"
    :disable-refresh="disablePolling"
  >
    <template #default="{ data, loading }">
      <VCard :loading="data && data.proposals.length > 0 ? false : loading" rounded v-bind="$attrs">
        <VCardText class="py-0">
          <div class="d-flex pt-4 ga-4 flex-wrap">
            <div class="text-body-1 font-weight-bold flex-grow-1">
              {{ title && title.length ? title : $t('terms.requests') }}
            </div>
            <div class="d-flex flex-grow-1 justify-end flex-column flex-md-row ga-2">
              <slot name="top-actions">
                <VBtn
                  v-if="props.seeAllLink"
                  color="secondary"
                  size="small"
                  variant="flat"
                  :to="props.seeAllLink"
                >
                  {{ $t('terms.see_all') }}
                </VBtn>
              </slot>
            </div>
          </div>
          <VList bg-color="transparent">
            <VDivider
              v-if="data && data.proposals.length > 0"
              class="mb-2 border-opacity-50"
              thickness="1"
            />
            <ProposalList
              v-if="data"
              :proposals="data.proposals"
              :privileges="data.privileges"
              :additionals="data.additional_info"
              :hide-not-found="props.hideNotFound"
              hide-headers
              :mode="app.isMobile ? 'list' : 'grid'"
              @voted="
                disablePolling = false;
                forceReload = true;
              "
              @opened="disablePolling = true"
              @closed="disablePolling = false"
            />
          </VList>
        </VCardText>
      </VCard>
    </template>
  </DataLoader>
</template>

<script lang="ts" setup>
import { ref } from 'vue';
import { RouteLocationRaw } from 'vue-router';
import { VBtn, VCard, VCardText, VDivider, VList } from 'vuetify/components';
import DataLoader from '~/components/DataLoader.vue';
import { ListProposalsOperationType } from '~/generated/wallet/wallet.did';
import { i18n } from '~/plugins/i18n.plugin';
import { useAppStore } from '~/stores/app.store';
import { useWalletStore } from '~/stores/wallet.store';
import { ListProposalsArgs } from '~/types/wallet.types';
import ProposalList from './ProposalList.vue';

const props = withDefaults(
  defineProps<{
    types: ListProposalsOperationType[];
    title?: string;
    limit?: number;
    sortBy?: ListProposalsArgs['sortBy'];
    seeAllLink?: RouteLocationRaw;
    refreshIntervalMs?: number;
    loadErrorMsg?: string;
    hideNotFound?: boolean;
  }>(),
  {
    title: undefined,
    limit: 3,
    sortBy: () => ({
      expirationDt: 'asc',
    }),
    refreshIntervalMs: 5000,
    seeAllLink: undefined,
    loadErrorMsg: i18n.global.t('app.data_load_error'),
    hideNotFound: false,
  },
);

const app = useAppStore();
const wallet = useWalletStore();
const forceReload = ref(false);
const disablePolling = ref(false);

let useVerifiedCall = false;

const fetchRecentProposals = async (): ReturnType<typeof wallet.service.listProposals> => {
  const result = await wallet.service.listProposals(
    {
      types: props.types,
      statuses: [{ Created: null }],
      limit: props.limit,
      sortBy: props.sortBy,
    },
    useVerifiedCall,
  );

  useVerifiedCall = true;

  return result;
};
</script>
