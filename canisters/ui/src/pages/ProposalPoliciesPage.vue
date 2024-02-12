<template>
  <PageLayout>
    <template #main-header>
      <PageHeader :title="pageTitle" :breadcrumbs="props.breadcrumbs.value">
        <template #actions>
          <AuthCheck :privileges="[Privilege.AddProposalPolicy]">
            <ProposalPolicyOpenBtn
              :text="$t('pages.proposal_policies.create_label')"
              variant="outlined"
            />
          </AuthCheck>
        </template>
      </PageHeader>
    </template>

    <template #main-body>
      <PageBody>
        <RecentProposals
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
        <DataLoader
          v-model:force-reload="forceReload"
          :load="fetchPolicies"
          :refresh-interval-ms="5000"
          :disable-refresh="disableRefresh"
        >
          <template #default="{ data, loading }">
            <VDataTable
              hover
              :headers="headers"
              :loading="loading"
              :items="data ? data : undefined"
              :items-per-page="-1"
            >
              <template #item.name="{ item }">
                {{
                  $t(
                    `proposal_policies.specifier.${mapProposalSpecifierToEnum(
                      item.specifier,
                    ).toLowerCase()}`,
                  )
                }}
              </template>
              <template #item.actions="{ item }">
                <div class="d-flex ga-0">
                  <ActionBtn
                    v-model="item.id"
                    :icon="mdiTrashCanOutline"
                    :submit="id => wallet.service.removeProposalPolicy(id)"
                    data-test-id="remove-proposal-policy-btn"
                    @failed="useOnFailedOperation"
                    @submitted="useOnSuccessfulOperation"
                  />
                  <ProposalPolicyOpenBtn
                    :policy-id="item.id"
                    :icon="mdiPencil"
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
          </template>
        </DataLoader>
        <VPagination
          v-model="pagination.selectedPage"
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
import { mdiPencil, mdiTrashCanOutline } from '@mdi/js';
import { computed, ref, toRefs } from 'vue';
import { useI18n } from 'vue-i18n';
import AuthCheck from '~/components/AuthCheck.vue';
import DataLoader from '~/components/DataLoader.vue';
import PageLayout from '~/components/PageLayout.vue';
import ActionBtn from '~/components/buttons/ActionBtn.vue';
import PageBody from '~/components/layouts/PageBody.vue';
import PageHeader from '~/components/layouts/PageHeader.vue';
import ProposalPolicyOpenBtn from '~/components/proposal-policies/ProposalPolicyOpenBtn.vue';
import RecentProposals from '~/components/proposals/RecentProposals.vue';
import {
  useOnFailedOperation,
  useOnSuccessfulOperation,
} from '~/composables/notifications.composable';
import { Routes } from '~/configs/routes.config';
import { ProposalPolicy } from '~/generated/wallet/wallet.did';
import { mapProposalSpecifierToEnum } from '~/mappers/specifiers.mapper';
import { useWalletStore } from '~/stores/wallet.store';
import { Privilege } from '~/types/auth.types';
import { BreadCrumbItem } from '~/types/navigation.types';
import { ProposalDomains } from '~/types/wallet.types';
import { throttle } from '~/utils/helper.utils';

const input = withDefaults(
  defineProps<{
    title?: string;
    breadcrumbs?: BreadCrumbItem[];
  }>(),
  {
    title: undefined,
    breadcrumbs: () => [],
  },
);
const props = toRefs(input);
const i18n = useI18n();
const pageTitle = computed(() => props.title.value ?? i18n.t('pages.proposal_policies.title'));
const wallet = useWalletStore();
const forceReload = ref(false);
const disableRefresh = ref(false);
const pagination = ref<{
  limit: number;
  totalPages: number;
  selectedPage: number;
}>({
  limit: 25,
  totalPages: 1,
  selectedPage: 1,
});

const headers = ref<{ title: string; key: string; headerProps: { class: string } }[]>([
  {
    title: i18n.t('terms.specifier'),
    key: 'name',
    headerProps: { class: 'font-weight-bold w-100' },
  },
  { title: '', key: 'actions', headerProps: { class: 'font-weight-bold' } },
]);

const triggerSearch = throttle(() => {
  forceReload.value = true;
}, 500);

const fetchPolicies = async (): Promise<ProposalPolicy[]> => {
  const { limit, selectedPage } = pagination.value;
  const offset = (selectedPage - 1) * limit;
  const { policies, total } = await wallet.service.listProposalPolicies({ limit, offset });

  pagination.value = {
    ...pagination.value,
    totalPages: Math.ceil(Number(total) / limit),
  };

  return policies;
};
</script>
