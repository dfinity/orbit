<template>
  <DataLoader
    v-model:force-reload="forceReload"
    :load="fetchRecentProposals"
    :error-msg="props.loadErrorMsg"
    :refresh-interval-ms="props.refreshIntervalMs"
    :disable-refresh="disablePolling"
  >
    <template #default="{ data, loading }">
      <VCard
        color="background"
        flat
        :loading="data && data.proposals.length > 0 ? false : loading"
        rounded
        v-bind="$attrs"
      >
        <VCardText class="py-0">
          <VList bg-color="transparent">
            <VListItem class="px-1">
              <VListItemTitle class="text-body-2 font-weight-bold">
                {{ title && title.length ? title : $t('terms.requests') }}
                <VBadge
                  v-if="data && Number(data.total) - data.proposals.length > 0"
                  :content="`+ ${Number(data.total) - data.proposals.length}`"
                  inline
                  color="secondary"
                />
              </VListItemTitle>
              <template #append>
                <slot name="top-actions">
                  <VBtn v-if="props.seeAllLink" variant="tonal" size="small" :to="props.seeAllLink">
                    {{ $t('terms.see_all') }}
                  </VBtn>
                </slot>
              </template>
            </VListItem>
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
import { ListProposalsOperationType } from '~/generated/wallet/wallet.did';
import { i18n } from '~/plugins/i18n.plugin';
import { useWalletStore } from '~/stores/wallet.store';
import { ListProposalsArgs } from '~/types/wallet.types';
import DataLoader from '~/components/DataLoader.vue';
import ProposalList from './ProposalList.vue';
import { useAppStore } from '~/stores/app.store';

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

const fetchRecentProposals = async (): ReturnType<typeof wallet.service.listProposals> => {
  const result = await wallet.service.listProposals({
    types: props.types,
    statuses: [{ Created: null }],
    limit: props.limit,
    sortBy: props.sortBy,
  });

  return result;
};
</script>
