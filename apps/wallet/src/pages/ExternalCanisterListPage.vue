<template>
  <PageLayout>
    <template #main-header>
      <PageHeader :title="pageTitle" :breadcrumbs="props.breadcrumbs">
        <template #actions>
          <AuthCheck :privileges="[Privilege.CreateExternalCanister]">
            <BtnCanisterSetup :text="$t('pages.external_canisters.btn_add_canister')" />
          </AuthCheck>
        </template>
      </PageHeader>
    </template>
    <template #main-body>
      <PageBody>
        <AuthCheck :privileges="[Privilege.ListRequests]">
          <RecentRequests
            class="mb-4"
            :see-all-link="{
              name: Routes.Requests,
              query: { group_by: RequestDomains.ExternalCanisters },
            }"
            :types="[
              { CreateExternalCanister: null },
              { ConfigureExternalCanister: [] },
              { FundExternalCanister: [] },
              { ChangeExternalCanister: [] },
              { CallExternalCanister: [] },
            ]"
            hide-not-found
          />
        </AuthCheck>

        <VContainer class="pa-0" fluid>
          <VRow>
            <VCol
              cols="12"
              class="d-flex flex-column-reverse flex-md-row align-md-start flex-no-wrap ga-4"
            >
              <div class="d-flex flex-column flex-grow-1 ga-4 align-self-stretch">
                <DataLoader
                  v-slot="{ loading }"
                  v-model:force-reload="forceReload"
                  :disable-refresh="disableRefresh"
                  :load="fetchList"
                  :refresh-interval-ms="5000"
                  @loaded="
                    result => {
                      externalCanisters = result.canisters;
                      privileges = result.privileges;
                    }
                  "
                >
                  <VDataTable
                    class="elevation-2 rounded"
                    :loading="loading"
                    :headers="headers"
                    :items="externalCanisters"
                    :items-per-page="-1"
                    :sort-by="[
                      {
                        key: 'name',
                        order: filters.fields.value.sort_by === 'name_asc' ? 'asc' : 'desc',
                      },
                    ]"
                    hover
                    @update:sort-by="
                      (sortedItems: Array<{ key: string; order: string }>): void => {
                        if (sortedItems.length) {
                          filters.fields.value.sort_by = `${sortedItems[0].key}_${sortedItems[0].order}`;
                        } else {
                          filters.fields.value.sort_by = 'name_asc';
                        }
                      }
                    "
                    @click:row="
                      (_: unknown, { item }: any) => {
                        $router.push({
                          name: Routes.ExternalCanister,
                          params: { id: item.canister_id },
                        });
                      }
                    "
                  >
                    <template #bottom>
                      <!--this hides the footer as pagination is not required-->
                    </template>
                    <template #item.name="{ item }">
                      <div class="d-flex flex-column ga-1 my-2">
                        <div>{{ item.name }}</div>
                        <div v-if="item.description?.[0]" class="text-medium-emphasis">
                          <TextOverflow
                            overflow-position="end"
                            :text="item.description[0]"
                            :max-length="48"
                          />
                        </div>
                        <div
                          v-if="app.isMobile && item.labels.length"
                          class="d-flex flex-wrap ga-2 mt-2"
                        >
                          <VChip v-for="label in item.labels" :key="label" size="small" label>
                            {{ label }}
                          </VChip>
                        </div>
                        <div class="mt-2 d-flex flex-nowrap align-center">
                          <VChip size="small" :prepend-icon="mdiIdentifier">
                            <TextOverflow :text="item.canister_id.toText()" :max-length="16" />
                            <template #append>
                              <VBtn
                                size="x-small"
                                variant="text"
                                :icon="mdiOpenInNew"
                                density="comfortable"
                                class="ml-1"
                                :href="`https://dashboard.internetcomputer.org/canister/${item.canister_id.toText()}`"
                                target="_blank"
                                @click.stop
                              />
                            </template>
                          </VChip>
                          <VBtn
                            size="x-small"
                            variant="text"
                            :icon="mdiContentCopy"
                            @click.stop="
                              copyToClipboard({
                                textToCopy: item.canister_id.toText(),
                                sendNotification: true,
                              })
                            "
                          />
                        </div>
                      </div>
                    </template>
                    <template #item.labels="{ item }">
                      <div class="d-flex flex-wrap ga-2">
                        <VChip v-for="label in item.labels" :key="label" size="small" label>
                          {{ label }}
                        </VChip>
                        <span v-if="!item.labels.length">-</span>
                      </div>
                    </template>
                    <template #item.state="{ item }">
                      <VChip v-if="variantIs(item.state, 'Active')" size="small" color="success">
                        {{ $t('terms.active') }}
                      </VChip>
                      <VChip
                        v-else-if="variantIs(item.state, 'Archived')"
                        size="small"
                        color="warning"
                      >
                        {{ $t('terms.archived') }}
                      </VChip>
                      <VChip v-else size="small" color="error">
                        {{ $t('terms.unknown') }}
                      </VChip>
                    </template>
                    <template #item.actions>
                      <div class="d-flex justify-end">
                        <VIcon :icon="mdiChevronRight" size="large" />
                      </div>
                    </template>
                  </VDataTable>
                  <VPagination
                    v-model="pagination.selectedPage"
                    class="mt-2"
                    :length="pagination.totalPages"
                    rounded
                    density="comfortable"
                    @update:model-value="triggerSearch"
                  />
                </DataLoader>
              </div>
              <FiltersCard :title="$t('terms.filters')" :icon="mdiFilter">
                <ExternalCanisterAutocomplete
                  v-model="filters.fields.value.canisters"
                  v-model:search="filters.fields.value.name_prefix"
                  v-model:items="filters.fields.value.canister_items"
                  :prepend-icon="mdiDatabase"
                  multiple
                />
                <ExternalCanisterLabelAutocomplete
                  v-model="filters.fields.value.labels"
                  :prepend-icon="mdiTag"
                  multiple
                />
                <CheckboxSelect
                  v-model="filters.fields.value.states"
                  :label="$t('terms.statuses')"
                  :items="availableStates"
                  :prepend-icon="mdiCog"
                />
                <VDivider thickness="2" class="my-2" />
                <VBtn
                  density="default"
                  block
                  color="primary-variant"
                  flat
                  size="small"
                  variant="tonal"
                  @click="filters.reset()"
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
</template>

<script lang="ts" setup>
import { Principal } from '@dfinity/principal';
import {
  mdiChevronRight,
  mdiCog,
  mdiContentCopy,
  mdiDatabase,
  mdiFilter,
  mdiIdentifier,
  mdiOpenInNew,
  mdiTag,
} from '@mdi/js';
import { computed, ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import {
  VBtn,
  VChip,
  VCol,
  VContainer,
  VDataTable,
  VDivider,
  VIcon,
  VPagination,
  VRow,
} from 'vuetify/components';
import AuthCheck from '~/components/AuthCheck.vue';
import DataLoader from '~/components/DataLoader.vue';
import PageLayout from '~/components/PageLayout.vue';
import TextOverflow from '~/components/TextOverflow.vue';
import BtnCanisterSetup from '~/components/external-canisters/BtnCanisterSetup.vue';
import ExternalCanisterAutocomplete from '~/components/external-canisters/ExternalCanisterAutocomplete.vue';
import ExternalCanisterLabelAutocomplete from '~/components/external-canisters/ExternalCanisterLabelAutocomplete.vue';
import CheckboxSelect from '~/components/inputs/CheckboxSelect.vue';
import PageBody from '~/components/layouts/PageBody.vue';
import PageHeader from '~/components/layouts/PageHeader.vue';
import RecentRequests from '~/components/requests/RecentRequests.vue';
import FiltersCard from '~/components/ui/FiltersCard.vue';
import {
  useExternalCanistersFilters,
  useExternalCanistersStates,
} from '~/composables/external-canisters.composable';
import { useFetchList, usePagination } from '~/composables/lists.composable';
import { Routes } from '~/configs/routes.config';
import {
  ExternalCanister,
  ExternalCanisterCallerPrivileges,
} from '~/generated/station/station.did';
import { mapExternalCanisterStateEnumToVariant } from '~/mappers/external-canister.mapper';
import { useAppStore } from '~/stores/app.store';
import { useStationStore } from '~/stores/station.store';
import type { PageProps, TableHeader } from '~/types/app.types';
import { Privilege } from '~/types/auth.types';
import { RequestDomains } from '~/types/station.types';
import { copyToClipboard } from '~/utils/app.utils';
import { throttle, variantIs } from '~/utils/helper.utils';

const props = withDefaults(defineProps<PageProps>(), { title: undefined, breadcrumbs: () => [] });
const app = useAppStore();
const station = useStationStore();
const i18n = useI18n();
const pageTitle = computed(() => props.title || i18n.t('pages.external_canisters.title'));
const externalCanisters = ref<ExternalCanister[]>([]);
const privileges = ref<ExternalCanisterCallerPrivileges[]>([]);
const disableRefresh = ref(false);
const forceReload = ref(false);
const pagination = usePagination();
const filters = useExternalCanistersFilters();
const availableStates = useExternalCanistersStates();

const headers = computed<TableHeader[]>(() => {
  if (app.isMobile) {
    return [
      { title: i18n.t('terms.canister'), key: 'name', sortable: true },
      { title: i18n.t('terms.status'), key: 'state', sortable: false },
      { title: '', key: 'actions', sortable: false, headerProps: { class: 'w-0' } },
    ];
  }

  return [
    { title: i18n.t('terms.canister'), key: 'name', sortable: true },
    { title: i18n.t('terms.labels'), key: 'labels', sortable: false },
    { title: i18n.t('terms.status'), key: 'state', sortable: false },
    { title: '', key: 'actions', sortable: false, headerProps: { class: 'w-0' } },
  ];
});

let useVerifiedCall = false;

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
  () => filters.fields.value,
  () => {
    filters.save();
    resetPagination();
    triggerSearch();
  },
  { deep: true },
);

const fetchList = useFetchList(
  (offset, limit) => {
    const results = station.service.listExternalCanisters(
      {
        offset,
        limit,
        labels: filters.fields.value.labels,
        states: filters.fields.value.states.map(mapExternalCanisterStateEnumToVariant),
        canisterIds: filters.fields.value.canisters.map(id => Principal.fromText(id)),
        sortBy: {
          Name: filters.fields.value.sort_by === 'name_asc' ? { Asc: null } : { Desc: null },
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
</script>
