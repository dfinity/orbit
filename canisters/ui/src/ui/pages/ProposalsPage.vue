<template>
  <PageLayout>
    <template #main-header>
      <PageHeader :title="$t('pages.proposals.title')">
        <template #actions>
          <!--todo: add export to csv functionality-->
          <VBtn color="primary-variant" variant="flat" disabled>
            {{ $t('app.export_csv') }}
          </VBtn>
        </template>
      </PageHeader>
    </template>

    <template #main-body>
      <PageBody>
        <DataLoader
          v-model:force-reload="forceReload"
          :load="fetchProposals"
          :disable-refresh="disableRefresh"
          :refresh-interval-ms="5000"
        >
          <template #default="{ data, loading }">
            <VContainer class="pa-0" fluid>
              <VRow>
                <VCol cols="12">
                  <VSlideGroup v-model="filters.groupBy" show-arrows>
                    <VSlideGroupItem
                      v-for="(domain, idx) in availableDomains"
                      :key="idx"
                      v-slot="{ isSelected, toggle }"
                    >
                      <VBtn
                        :color="isSelected ? 'primary-variant' : undefined"
                        variant="flat"
                        density="comfortable"
                        class="mr-2"
                        @click="toggle"
                      >
                        {{ $t(`proposals.domains.${domain.id}`) }}
                      </VBtn>
                    </VSlideGroupItem>
                  </VSlideGroup>
                </VCol>
              </VRow>
              <VRow>
                <VCol cols="12 d-flex flex-column-reverse flex-md-row flex-wrap ga-4">
                  <div class="d-flex flex-column flex-grow-1 ga-4">
                    <ProposalList
                      :loading="loading"
                      :proposals="data?.proposals ?? []"
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
                  <VCard
                    color="background"
                    variant="flat"
                    min-height="300px"
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
                      <DateRange
                        v-model="filters.expires"
                        :label="$t('terms.expires')"
                        :prepend-icon="mdiCalendar"
                      />
                      <BtnSelect
                        v-model="filters.statuses"
                        :label="$t('terms.statuses')"
                        :items="statuses"
                        :prepend-icon="mdiCog"
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
          </template>
        </DataLoader>
      </PageBody>
    </template>
  </PageLayout>
</template>

<script lang="ts" setup>
import { mdiCalendar, mdiCog, mdiFilter } from '@mdi/js';
import { ref, watch } from 'vue';
import { useRouter } from 'vue-router';
import { throttle } from '~/core/utils';
import { ProposalStatusCode } from '~/generated/wallet/wallet.did';
import DataLoader from '~/ui/components/DataLoader.vue';
import PageLayout from '~/ui/components/PageLayout.vue';
import BtnSelect from '~/ui/components/inputs/BtnSelect.vue';
import DateRange from '~/ui/components/inputs/DateRange.vue';
import PageBody from '~/ui/components/layouts/PageBody.vue';
import PageHeader from '~/ui/components/layouts/PageHeader.vue';
import ProposalList from '~/ui/components/proposals/ProposalList.vue';
import {
  useAvailableDomains,
  useFilterUtils,
  useProposalStatusItems,
  useSavedFilters,
} from '~/ui/composables/proposal.composable';
import { useAppStore } from '~/ui/stores/app';
import { useWalletStore } from '~/ui/stores/wallet';
import { convertDate } from '~/utils/date.utils';

const app = useAppStore();
const wallet = useWalletStore();
const availableDomains = useAvailableDomains();
const statuses = useProposalStatusItems();
const filters = useSavedFilters();
const filterUtils = useFilterUtils();
const disableRefresh = ref(false);
const forceReload = ref(false);
const router = useRouter();

const saveFilters = (): void => {
  router.replace({ query: filterUtils.getQuery(filters.value) });
};

const pagination = ref<{
  limit: number;
  totalPages: number;
  selectedPage: number;
}>({
  limit: 25,
  totalPages: 1,
  selectedPage: 1,
});

const resetPagination = (): void => {
  pagination.value = {
    ...pagination.value,
    totalPages: 1,
    selectedPage: 1,
  };
};

const triggerSearch = throttle(() => {
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

const fetchProposals = async (): ReturnType<typeof wallet.service.listProposals> => {
  const nextOffset =
    pagination.value.selectedPage * pagination.value.limit - pagination.value.limit;

  const result = await wallet.service.listProposals({
    types: availableDomains.value.find((_, idx) => idx === filters.value.groupBy)?.types,
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
    statuses: filters.value.statuses.map(status => ({ [status]: null })) as ProposalStatusCode[],
    limit: pagination.value.limit,
    offset: nextOffset,
    sortBy: {
      createdAt: 'asc',
    },
  });

  pagination.value.totalPages = Math.max(
    Math.ceil(Number(result.total) / pagination.value.limit),
    1,
  );

  return result;
};
</script>
