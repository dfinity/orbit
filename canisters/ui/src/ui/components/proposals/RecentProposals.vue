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
      >
        <VCardTitle v-if="title && title.length" class="text-h6">
          {{ props.title }}
        </VCardTitle>
        <VCardText class="py-0">
          <VList bg-color="transparent">
            <VListItem class="px-1">
              <VListItemTitle class="text-body-2 font-weight-bold">
                {{ $t('terms.requests') }}
                <VBadge
                  v-if="data && Number(data.total) - data.proposals.length > 0"
                  :content="`+ ${Number(data.total) - data.proposals.length}`"
                  inline
                  color="secondary"
                />
              </VListItemTitle>
              <template #append>
                <VBtn variant="tonal" size="small">
                  {{ $t('terms.see_all') }}
                </VBtn>
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
              hide-headers
              @voted="forceReload = true"
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
import { ListProposalsOperationType } from '~/generated/wallet/wallet.did';
import { ListProposalsArgs } from '~/types';
import DataLoader from '~/ui/components/DataLoader.vue';
import { i18n } from '~/ui/modules';
import { useWalletStore } from '~/ui/stores/wallet';
import { ref } from 'vue';
import ProposalList from './ProposalList.vue';

const props = withDefaults(
  defineProps<{
    types: ListProposalsOperationType[];
    title?: string;
    limit?: number;
    sortBy?: ListProposalsArgs['sortBy'];
    refreshIntervalMs?: number;
    loadErrorMsg?: string;
  }>(),
  {
    title: undefined,
    limit: 3,
    sortBy: () => ({
      expirationDt: 'asc',
    }),
    refreshIntervalMs: 5000,
    loadErrorMsg: i18n.global.t('app.data_load_error'),
  },
);

const wallet = useWalletStore();
const forceReload = ref(false);
const disablePolling = ref(false);

const fetchRecentProposals = async () => {
  const result = await wallet.service.listProposals({
    types: [{ AddUserGroup: null }, { EditUserGroup: null }, { RemoveUserGroup: null }],
    statuses: [{ Created: null }],
    limit: props.limit,
    sortBy: props.sortBy,
  });

  return result;
};
</script>
