<template>
  <DataLoader
    v-model:force-reload="forceReload"
    :load="fetchRecentRequests"
    :error-msg="props.loadErrorMsg ?? $t('app.data_load_error')"
    :refresh-interval-ms="props.refreshIntervalMs"
    :disable-refresh="disablePolling"
  >
    <template #default="{ data, loading }">
      <VCard :loading="data && data.requests.length > 0 ? false : loading" rounded v-bind="$attrs">
        <VCardTitle>
          <div class="d-flex pt-1 ga-4 flex-wrap">
            <div class="flex-grow-1">
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
        </VCardTitle>
        <template v-if="data && data.requests.length">
          <VDivider class="mb-2" />
          <VCardText class="py-0 px-0">
            <VList bg-color="transparent">
              <RequestList
                :requests="data.requests"
                :privileges="data.privileges"
                :additionals="data.additional_info"
                :hide-not-found="props.hideNotFound"
                hide-headers
                mode="grid"
                :show-items-title="props.showItemsTitle"
                @approved="
                  disablePolling = false;
                  forceReload = true;
                "
                @opened="disablePolling = true"
                @closed="disablePolling = false"
              />
            </VList>
          </VCardText>
        </template>
      </VCard>
    </template>
  </DataLoader>
</template>

<script lang="ts" setup>
import { ref } from 'vue';
import { RouteLocationRaw } from 'vue-router';
import { VBtn, VCard, VCardText, VDivider, VList } from 'vuetify/components';
import DataLoader from '~/components/DataLoader.vue';
import { ListRequestsOperationType, RequestStatusCode } from '~/generated/station/station.did';
import { useStationStore } from '~/stores/station.store';
import { ListRequestsArgs } from '~/types/station.types';
import RequestList from './RequestList.vue';

const props = withDefaults(
  defineProps<{
    types?: ListRequestsOperationType[];
    title?: string;
    limit?: number;
    sortBy?: ListRequestsArgs['sortBy'];
    seeAllLink?: RouteLocationRaw;
    refreshIntervalMs?: number;
    loadErrorMsg?: string;
    hideNotFound?: boolean;
    statuses?: RequestStatusCode[];
    showItemsTitle?: boolean;
  }>(),
  {
    title: undefined,
    types: undefined,
    limit: 4,
    sortBy: () => ({
      expirationDt: 'asc',
    }),
    refreshIntervalMs: 5000,
    seeAllLink: undefined,
    loadErrorMsg: undefined,
    hideNotFound: false,
    showItemsTitle: true,
    statuses: () => [{ Created: null }],
  },
);

const station = useStationStore();
const forceReload = ref(false);
const disablePolling = ref(false);

let useVerifiedCall = false;

const fetchRecentRequests = async (): ReturnType<typeof station.service.listRequests> => {
  const result = await station.service.listRequests(
    {
      types: props.types,
      statuses: props.statuses,
      limit: props.limit,
      sortBy: props.sortBy,
    },
    useVerifiedCall,
  );

  useVerifiedCall = true;

  return result;
};
</script>
