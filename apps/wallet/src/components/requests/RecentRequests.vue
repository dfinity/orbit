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
              v-if="data && data.requests.length > 0"
              class="mb-2 border-opacity-50"
              thickness="1"
            />
            <RequestList
              v-if="data"
              :requests="data.requests"
              :privileges="data.privileges"
              :additionals="data.additional_info"
              :hide-not-found="props.hideNotFound"
              hide-headers
              :mode="app.isMobile ? 'list' : 'grid'"
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
import { useAppStore } from '~/stores/app.store';
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
    limit: 3,
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

const app = useAppStore();
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
