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
        >
          <template #default="{ data, loading }">
            <VContainer class="pa-0" fluid>
              <VRow>
                <VCol cols="12">
                  <VSlideGroup v-model="filters.groupBy" show-arrows>
                    <VSlideGroupItem
                      v-for="(filterGroup, idx) in availableFilterGroups"
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
                        {{ $t(`proposals.domains.${filterGroup.id}`) }}
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
import { computed, ref, watch } from 'vue';
import { logger } from '~/core';
import { parseDate, throttle } from '~/core/utils';
import { ListProposalsOperationType, ProposalStatusCode } from '~/generated/wallet/wallet.did';
import { ProposalStatusEnum } from '~/types';
import DataLoader from '~/ui/components/DataLoader.vue';
import PageLayout from '~/ui/components/PageLayout.vue';
import BtnSelect from '~/ui/components/inputs/BtnSelect.vue';
import DateRange, { DateRangeModel } from '~/ui/components/inputs/DateRange.vue';
import PageBody from '~/ui/components/layouts/PageBody.vue';
import PageHeader from '~/ui/components/layouts/PageHeader.vue';
import ProposalList from '~/ui/components/proposals/ProposalList.vue';
import { i18n, router } from '~/ui/modules';
import { useAppStore } from '~/ui/stores/app';
import { useWalletStore } from '~/ui/stores/wallet';

const app = useAppStore();
const wallet = useWalletStore();

const availableFilterGroups = ref<
  {
    id: string;
    types: ListProposalsOperationType[];
  }[]
>([
  { id: 'accounts', types: [{ AddAccount: null }, { EditAccount: null }] },
  { id: 'address_book', types: [] },
  { id: 'transfers', types: [{ Transfer: [] }] },
  { id: 'users', types: [{ AddUser: null }, { EditUser: null }] },
  {
    id: 'system',
    types: [
      { AddAccessPolicy: null },
      { EditAccessPolicy: null },
      { RemoveAccessPolicy: null },
      { AddProposalPolicy: null },
      { EditProposalPolicy: null },
      { RemoveProposalPolicy: null },
      { ChangeCanister: null },
      { AddUserGroup: null },
      { EditUserGroup: null },
      { RemoveUserGroup: null },
    ],
  },
]);

const statuses = computed<{ key: ProposalStatusEnum; text: string }[]>(() =>
  Object.values(ProposalStatusEnum).map(status => ({
    key: status,
    text: i18n.global.t(`proposals.status.${status.toLowerCase()}`),
  })),
);

type Filters = {
  groupBy: number;
  created: DateRangeModel;
  expires: DateRangeModel;
  statuses: ProposalStatusEnum[];
};

type StorableFilters = {
  group_by?: string;
  created_from?: string;
  created_to?: string;
  expires_from?: string;
  expires_to?: string;
  statuses?: ProposalStatusEnum[];
};

const getDefaultFilters = (): Filters => ({
  groupBy: 0,
  created: {
    from: new Date(new Date().setDate(new Date().getDate() - 30)),
    to: new Date(),
  },
  expires: {
    from: undefined,
    to: undefined,
  },
  statuses: [ProposalStatusEnum.Created],
});

const getSavedFilters = (): Filters => {
  const defaultFilters = getDefaultFilters();

  try {
    const query = router.currentRoute.value.query as StorableFilters;
    const createdDt: DateRangeModel = {
      from: query?.created_from ? parseDate(query.created_from) : defaultFilters.created.from,
      to: query?.created_to ? parseDate(query.created_to) : defaultFilters.created.to,
    };

    createdDt.from = createdDt.from! > createdDt.to! ? createdDt.to : createdDt.from;
    createdDt.to = createdDt.to! < createdDt.from! ? createdDt.from : createdDt.to;

    const expiresDt: DateRangeModel = {
      from: query?.expires_from ? parseDate(query.expires_from) : defaultFilters.expires.from,
      to: query?.expires_to ? parseDate(query.expires_to) : defaultFilters.expires.to,
    };

    expiresDt.from = expiresDt.from! > expiresDt.to! ? expiresDt.to : expiresDt.from;
    expiresDt.to = expiresDt.to! < expiresDt.from! ? expiresDt.from : expiresDt.to;

    let statuses = query?.statuses ?? defaultFilters.statuses;
    if (!Array.isArray(statuses)) {
      statuses = [statuses];
    }

    return {
      groupBy: query?.group_by
        ? availableFilterGroups.value.findIndex(group => group.id === query.group_by)
        : defaultFilters.groupBy,
      created: createdDt,
      expires: expiresDt,
      statuses: Object.values(ProposalStatusEnum).filter(status => statuses.includes(status)),
    };
  } catch (e) {
    logger.error('Failed to parse filters from query', e);

    app.sendNotification({
      type: 'error',
      message: i18n.global.t('app.params_parse_error'),
    });

    return defaultFilters;
  }
};

const disableRefresh = ref(false);
const forceReload = ref(false);
const filters = ref<Filters>(getSavedFilters());

const saveFilters = (): void => {
  const { groupBy, created, expires, statuses } = filters.value;
  const storableFilters: StorableFilters = {
    created_from: created.from?.toISOString(),
    created_to: created.to?.toISOString(),
    expires_from: expires.from?.toISOString(),
    expires_to: expires.to?.toISOString(),
    group_by: availableFilterGroups.value.find((_, idx) => idx === groupBy)?.id,
    statuses: statuses,
  };

  router.replace({ query: storableFilters });
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

const fetchProposals = async () => {
  const nextOffset =
    pagination.value.selectedPage * pagination.value.limit - pagination.value.limit;
  const result = await wallet.service.listProposals({
    types: availableFilterGroups.value.find((_, idx) => idx === filters.value.groupBy)?.types,
    created_dt: { fromDt: filters.value.created.from, toDt: filters.value.created.to },
    expiration_dt: { fromDt: filters.value.expires.from, toDt: filters.value.expires.to },
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
