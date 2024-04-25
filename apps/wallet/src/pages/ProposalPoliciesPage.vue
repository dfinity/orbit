<template>
  <PageLayout>
    <template #main-header>
      <PageHeader :title="pageTitle" :breadcrumbs="props.breadcrumbs">
        <template #actions>
          <AuthCheck :privileges="[Privilege.AddProposalPolicy]">
            <OpenProposalPolicyBtn :text="$t('pages.proposal_policies.create_label')" />
          </AuthCheck>
        </template>
      </PageHeader>
    </template>

    <template #main-body>
      <PageBody>
        <AuthCheck :privileges="[Privilege.ListProposals]">
          <RecentProposals
            class="mb-4"
            :see-all-link="{
              name: Routes.Proposals,
              query: { group_by: ProposalDomains.System },
            }"
            :types="[
              { AddProposalPolicy: null },
              { EditProposalPolicy: null },
              { RemoveProposalPolicy: null },
            ]"
            hide-not-found
          />
        </AuthCheck>
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
            <template #item.actions="{ item: policy }">
              <div class="d-flex ga-0">
                <ActionBtn
                  v-if="hasDeletePrivilege(policy.id)"
                  v-model="policy.id"
                  :icon="mdiTrashCanOutline"
                  :submit="id => wallet.service.removeProposalPolicy(id)"
                  data-test-id="remove-proposal-policy-btn"
                  @failed="useOnFailedOperation"
                  @submitted="useOnSuccessfulOperation"
                />
                <OpenProposalPolicyBtn
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
      </PageBody>
    </template>
  </PageLayout>
</template>

<script lang="ts" setup>
import { mdiEye, mdiPencil, mdiTrashCanOutline } from '@mdi/js';
import { computed, ref } from 'vue';
import { useI18n } from 'vue-i18n';
import { VDataTable, VPagination } from 'vuetify/components';
import AuthCheck from '~/components/AuthCheck.vue';
import DataLoader from '~/components/DataLoader.vue';
import PageLayout from '~/components/PageLayout.vue';
import ActionBtn from '~/components/buttons/ActionBtn.vue';
import PageBody from '~/components/layouts/PageBody.vue';
import PageHeader from '~/components/layouts/PageHeader.vue';
import OpenProposalPolicyBtn from '~/components/proposal-policies/OpenProposalPolicyBtn.vue';
import SpecifierSelector from '~/components/proposal-policies/specifier/SpecifierSelector.vue';
import RecentProposals from '~/components/proposals/RecentProposals.vue';
import { useFetchList, usePagination } from '~/composables/lists.composable';
import {
  useOnFailedOperation,
  useOnSuccessfulOperation,
} from '~/composables/notifications.composable';
import { Routes } from '~/configs/routes.config';
import {
  ProposalPolicy,
  ProposalPolicyCallerPrivileges,
  UUID,
} from '~/generated/wallet/wallet.did';
import { useWalletStore } from '~/stores/wallet.store';
import type { PageProps, TableHeader } from '~/types/app.types';
import { Privilege } from '~/types/auth.types';
import { ProposalDomains } from '~/types/wallet.types';
import { throttle } from '~/utils/helper.utils';

const props = withDefaults(defineProps<PageProps>(), { title: undefined, breadcrumbs: () => [] });
const i18n = useI18n();
const wallet = useWalletStore();
const pageTitle = computed(() => props.title ?? i18n.t('pages.proposal_policies.title'));
const forceReload = ref(false);
const disableRefresh = ref(false);
const policies = ref<ProposalPolicy[]>([]);
const privileges = ref<ProposalPolicyCallerPrivileges[]>([]);
const pagination = usePagination();
const triggerSearch = throttle(() => (forceReload.value = true), 500);
const headers = ref<TableHeader[]>([
  {
    title: i18n.t('terms.approval_policy'),
    key: 'name',
    headerProps: { class: 'font-weight-bold w-100' },
    sortable: false,
  },
  { title: '', key: 'actions', headerProps: { class: 'font-weight-bold' }, sortable: false },
]);

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
    const results = wallet.service.listProposalPolicies(
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
</script>
