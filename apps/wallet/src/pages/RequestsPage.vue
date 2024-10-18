<template>
  <PageLayout>
    <template #main-header>
      <PageHeader :title="pageTitle" :breadcrumbs="props.breadcrumbs">
        <template #actions>
          <ExportCsvActionBtn :filters="filters" :domains="shownRequestDomains" />
        </template>
      </PageHeader>
    </template>

    <template #main-body>
      <PageBody>
        <DataLoader
          v-slot="{ loading }"
          v-model:force-reload="forceReload"
          :disable-refresh="disableRefresh"
          :load="fetchList"
          :refresh-interval-ms="5000"
          @loaded="
            result => {
              requests = result.requests;
              privileges = result.privileges;
              additionals = result.additional_info;
            }
          "
        >
          <VContainer class="pa-0" fluid>
            <VRow v-if="shownRequestDomains.length > 1">
              <VCol cols="12">
                <VSlideGroup v-model="slideGroupIdIndex" show-arrows>
                  <VSlideGroupItem
                    v-for="domain in shownRequestDomains"
                    :key="domain.id"
                    v-slot="{ isSelected, toggle }"
                  >
                    <VBtn
                      :color="isSelected ? 'secondary' : undefined"
                      variant="flat"
                      density="comfortable"
                      class="mr-2"
                      @click="toggle"
                    >
                      {{ $t(`requests.domains.${domain.id}`) }}
                    </VBtn>
                  </VSlideGroupItem>
                </VSlideGroup>
              </VCol>
            </VRow>
            <VRow>
              <VCol
                cols="12"
                class="d-flex flex-column-reverse flex-md-row align-md-start flex-no-wrap ga-4"
              >
                <div class="d-flex flex-column flex-grow-1 ga-4 align-self-stretch">
                  <RequestList
                    :loading="loading"
                    :requests="requests"
                    :privileges="privileges"
                    :additionals="additionals"
                    @closed="disableRefresh = false"
                    @opened="disableRefresh = true"
                  />
                  <VPagination
                    v-model="pagination.selectedPage"
                    :length="pagination.totalPages"
                    rounded
                    density="comfortable"
                    @update:model-value="triggerSearch"
                  />
                </div>
                <FiltersCard :title="$t('terms.filters')" :icon="mdiFilter">
                  <DateRange
                    v-model="filters.created"
                    :label="$t('terms.created')"
                    :prepend-icon="mdiCalendar"
                  />
                  <DateRange
                    v-model="filters.expires"
                    :label="$t('terms.expires')"
                    :prepend-icon="mdiCalendar"
                  />
                  <CanisterIdField
                    v-if="isExternalCanisterFilterGroup"
                    v-model="filters.canisterId"
                    prepend-inner-icon
                    name="canister_id"
                  />
                  <CheckboxSelect
                    v-model="filters.statuses"
                    :label="$t('terms.statuses')"
                    :items="statuses"
                    :prepend-icon="mdiCog"
                  />
                  <VSpacer />
                  <VDivider thickness="2" class="my-2" />
                  <VBtn
                    density="default"
                    block
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
        </DataLoader>
      </PageBody>
    </template>
  </PageLayout>
</template>

<script lang="ts" setup>
import { Principal } from '@dfinity/principal';
import { mdiCalendar, mdiCog, mdiFilter } from '@mdi/js';
import { computed, Ref, ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import { useRouter } from 'vue-router';
import {
  VBtn,
  VCol,
  VContainer,
  VDivider,
  VPagination,
  VRow,
  VSlideGroup,
  VSlideGroupItem,
  VSpacer,
} from 'vuetify/components';
import DataLoader from '~/components/DataLoader.vue';
import PageLayout from '~/components/PageLayout.vue';
import CanisterIdField from '~/components/inputs/CanisterIdField.vue';
import CheckboxSelect from '~/components/inputs/CheckboxSelect.vue';
import DateRange from '~/components/inputs/DateRange.vue';
import PageBody from '~/components/layouts/PageBody.vue';
import PageHeader from '~/components/layouts/PageHeader.vue';
import RequestList from '~/components/requests/RequestList.vue';
import ExportCsvActionBtn from '~/components/requests/export/ExportCsvActionBtn.vue';
import FiltersCard from '~/components/ui/FiltersCard.vue';
import { useFetchList, usePagination } from '~/composables/lists.composable';
import {
  useAvailableDomains,
  useFilterUtils,
  useRequestStatusItems,
  useSavedFilters,
} from '~/composables/request.composable';
import {
  Request,
  RequestAdditionalInfo,
  RequestCallerPrivileges,
  RequestStatusCode,
} from '~/generated/station/station.did';
import { useStationStore } from '~/stores/station.store';
import type { PageProps } from '~/types/app.types';
import { RequestDomains } from '~/types/station.types';
import { convertDate } from '~/utils/date.utils';
import { throttle, variantIs } from '~/utils/helper.utils';

export interface RequestsPageProps extends PageProps {
  domains?: RequestDomains[];
}

const props = withDefaults(defineProps<RequestsPageProps>(), {
  title: undefined,
  domains: () => [],
  breadcrumbs: () => [],
});

const i18n = useI18n();
const pageTitle = computed(() => props.title || i18n.t('pages.requests.title'));
const station = useStationStore();
const availableDomains = useAvailableDomains();
const statuses = useRequestStatusItems();
const filterUtils = useFilterUtils();
const disableRefresh = ref(false);
const forceReload = ref(false);
const router = useRouter();
const shownRequestDomains = computed(() => {
  if (props.domains !== undefined && props.domains.length > 0) {
    const domains = props.domains;

    return availableDomains.value.filter(domain => domains.includes(domain.id));
  }

  return availableDomains.value;
});

const requests: Ref<Request[]> = ref([]);
const privileges = ref<RequestCallerPrivileges[]>([]);
const additionals = ref<RequestAdditionalInfo[]>([]);
const filters = useSavedFilters();
const slideGroupIdIndex = ref<number>(
  availableDomains.value.findIndex(domain => domain.id === filters.value.groupBy),
);

watch(
  slideGroupIdIndex,
  index => {
    filters.value.groupBy = availableDomains.value[index].id ?? RequestDomains.All;
  },
  { immediate: true },
);

const saveFilters = (): void => {
  router.replace({ query: filterUtils.getQuery(filters.value) });
};

const pagination = usePagination({ limit: 10 });

const resetPagination = (): void => {
  pagination.value = {
    ...pagination.value,
    totalPages: 1,
    selectedPage: 1,
  };
};

const triggerSearch = throttle(() => {
  useVerifiedCall = false;
  forceReload.value = true;
}, 500);

watch(
  () => filters.value,
  () => {
    saveFilters();
    resetPagination();
    triggerSearch();
  },
  { deep: true },
);

let useVerifiedCall = false;

const fetchList = useFetchList(
  async (offset, limit) => {
    const results = station.service.listRequests(
      {
        types: shownRequestDomains.value.find(domain => domain.id === filters.value.groupBy)?.types,
        created_dt: {
          fromDt: convertDate(filters.value.created.from, {
            time: 'start-of-day',
            tz: 'local',
          }),
          toDt: convertDate(filters.value.created.to, {
            time: 'end-of-day',
            tz: 'local',
          }),
        },
        expiration_dt: {
          fromDt: convertDate(filters.value.expires.from, {
            time: 'start-of-day',
            tz: 'local',
          }),
          toDt: convertDate(filters.value.expires.to, {
            time: 'end-of-day',
            tz: 'local',
          }),
        },
        statuses: filters.value.statuses.map(status => ({
          [status]: null,
        })) as RequestStatusCode[],
        limit,
        offset,
        sortBy: {
          createdAt: 'desc',
        },
      },
      useVerifiedCall,
    );

    useVerifiedCall = true;

    return results;
  },
  {
    pagination,
    getTotal: res => Number(res.total),
  },
);

const isExternalCanisterFilterGroup = computed(
  () => filters.value.groupBy === RequestDomains.ExternalCanisters,
);

watch(
  () => filters.value.canisterId,
  () => {
    if (!isExternalCanisterFilterGroup.value) {
      return;
    }

    availableDomains.value = availableDomains.value.map(domain => {
      if (domain.id === RequestDomains.ExternalCanisters) {
        for (const typeIndex in domain.types) {
          let filterType = domain.types[typeIndex];
          const filterBy: [] | [Principal] = filters.value.canisterId
            ? [filters.value.canisterId]
            : [];

          if (variantIs(filterType, 'CallExternalCanister')) {
            filterType = { CallExternalCanister: filterBy };
          } else if (variantIs(filterType, 'ConfigureExternalCanister')) {
            filterType = { ConfigureExternalCanister: filterBy };
          } else if (variantIs(filterType, 'FundExternalCanister')) {
            filterType = { FundExternalCanister: filterBy };
          } else if (variantIs(filterType, 'ChangeExternalCanister')) {
            filterType = { ChangeExternalCanister: filterBy };
          }

          domain.types[typeIndex] = filterType;
        }
      }

      return domain;
    });
  },
);
</script>
