<template>
  <Teleport v-if="mounted" to="#policies-actions">
    <AuthCheck :privileges="[Privilege.AddRequestPolicy]">
      <OpenRequestPolicyBtn :text="$t('pages.request_policies.create_label')" />
    </AuthCheck>
  </Teleport>

  <DataLoader
    v-slot="{ loading }"
    v-model:force-reload="forceReload"
    :disable-refresh="disableRefresh"
    :load="fetchList"
    :refresh-interval-ms="5000"
    @loaded="
      result => {
        privileges = result.privileges;
        policies = result.policies;
      }
    "
  >
    <VDataTable
      class="elevation-2 rounded"
      :headers="headers"
      :loading="loading"
      :items="policies"
      :items-per-page="-1"
      :hover="true"
    >
      <template #item.name="{ item: policy }">
        <SpecifierSelector v-model="policy.specifier" readonly />
      </template>
      <template #item.rule="{ item: policy }">
        <RuleSummary :rule="policy.rule" />
      </template>
      <template #item.actions="{ item: policy }">
        <div class="d-flex ga-0">
          <ActionBtn
            v-if="hasDeletePrivilege(policy.id)"
            v-model="policy.id"
            :icon="mdiTrashCanOutline"
            :submit="id => station.service.removeRequestPolicy(id)"
            data-test-id="remove-request-policy-btn"
            @failed="useOnFailedOperation"
            @submitted="useOnSuccessfulOperation"
          />
          <OpenRequestPolicyBtn
            :policy-id="policy.id"
            :icon="!hasEditPrivilege(policy.id) ? mdiEye : mdiPencil"
            :readonly="!hasEditPrivilege(policy.id)"
            variant="flat"
            color="default"
            size="small"
            @opened="disableRefresh = $event"
          />
        </div>
      </template>
      <template #bottom>
        <!-- This removes the bottom pagination since we want to display all the results -->
      </template>
    </VDataTable>
  </DataLoader>
  <VPagination
    v-model="pagination.selectedPage"
    class="mt-2"
    :length="pagination.totalPages"
    rounded
    density="comfortable"
    @update:model-value="triggerSearch"
  />
</template>

<script lang="ts" setup>
import { mdiEye, mdiPencil, mdiTrashCanOutline } from '@mdi/js';
import { computed, onMounted, Ref, ref } from 'vue';
import { useI18n } from 'vue-i18n';
import { useDisplay } from 'vuetify';
import { VDataTable, VPagination } from 'vuetify/components';
import AuthCheck from '~/components/AuthCheck.vue';
import DataLoader from '~/components/DataLoader.vue';
import ActionBtn from '~/components/buttons/ActionBtn.vue';
import OpenRequestPolicyBtn from '~/components/request-policies/OpenRequestPolicyBtn.vue';
import RuleSummary from '~/components/request-policies/rule/RuleSummary.vue';
import SpecifierSelector from '~/components/request-policies/specifier/SpecifierSelector.vue';
import { useFetchList, usePagination } from '~/composables/lists.composable';
import {
  useOnFailedOperation,
  useOnSuccessfulOperation,
} from '~/composables/notifications.composable';
import {
  RequestPolicy,
  RequestPolicyCallerPrivileges,
  UUID,
} from '~/generated/station/station.did';
import { useStationStore } from '~/stores/station.store';
import type { TableHeader } from '~/types/app.types';
import { Privilege } from '~/types/auth.types';
import { throttle } from '~/utils/helper.utils';

const i18n = useI18n();
const { xs } = useDisplay();
const station = useStationStore();
const forceReload = ref(false);
const disableRefresh = ref(false);
const policies: Ref<RequestPolicy[]> = ref([]);
const privileges = ref<RequestPolicyCallerPrivileges[]>([]);
const pagination = usePagination();
const mounted = ref(false);
const triggerSearch = throttle(() => (forceReload.value = true), 500);

const headers = computed(() => {
  const headers: TableHeader[] = [
    {
      title: i18n.t('terms.request_policy'),
      key: 'name',
      headerProps: { class: `font-weight-bold ${xs.value ? 'w-100' : 'w-50'}` },
      sortable: false,
    },
  ];
  if (!xs.value) {
    headers.push({
      title: i18n.t('terms.rule'),
      key: 'rule',
      headerProps: { class: 'font-weight-bold w-50' },
      sortable: false,
    });
  }

  headers.push({
    title: '',
    key: 'actions',
    headerProps: { class: 'font-weight-bold' },
    sortable: false,
  });
  return headers;
});

const hasEditPrivilege = (id: UUID): boolean => {
  const privilege = privileges.value.find(p => p.id === id);
  return !!privilege?.can_edit;
};

const hasDeletePrivilege = (id: UUID): boolean => {
  const privilege = privileges.value.find(p => p.id === id);
  return !!privilege?.can_delete;
};

let useVerifiedCall = false;

const fetchList = useFetchList(
  (offset, limit) => {
    const results = station.service.listRequestPolicies(
      {
        offset,
        limit,
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

onMounted(() => {
  mounted.value = true;
});
</script>
