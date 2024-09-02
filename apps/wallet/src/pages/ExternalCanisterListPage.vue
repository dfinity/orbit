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
                  todo_ec_list_table
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
import { mdiCog, mdiDatabase, mdiFilter, mdiTag } from '@mdi/js';
import { computed, ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import { VBtn, VCol, VContainer, VDivider, VRow } from 'vuetify/components';
import AuthCheck from '~/components/AuthCheck.vue';
import DataLoader from '~/components/DataLoader.vue';
import PageLayout from '~/components/PageLayout.vue';
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
import { useStationStore } from '~/stores/station.store';
import type { PageProps } from '~/types/app.types';
import { Privilege } from '~/types/auth.types';
import { RequestDomains } from '~/types/station.types';
import { throttle } from '~/utils/helper.utils';

const props = withDefaults(defineProps<PageProps>(), { title: undefined, breadcrumbs: () => [] });
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
          Name: { Asc: null },
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
