<template>
  <PageLayout>
    <template #main-header>
      <PageHeader :title="props.title" :breadcrumbs="props.breadcrumbs" />
    </template>

    <template #main-body>
      <PageBody>
        <RecentProposals
          class="mb-4"
          :see-all-link="{
            name: Routes.Proposals,
            query: { group_by: ProposalDomains.System },
          }"
          :types="[
            { AddAccessPolicy: null },
            { EditAccessPolicy: null },
            { RemoveAccessPolicy: null },
          ]"
          hide-not-found
        />

        <DataLoader
          v-slot="{ data, loading }"
          :load="fetchAccessPolicies"
          :refresh-interval-ms="5000"
          :disable-refresh="disableRefresh"
        >
          <AccessPolicyList
            :loading="loading"
            :resources="resources"
            :access-policies="data ? data.policies : []"
            :preload-user-groups="data ? data.userGroups : []"
            :preload-users="data ? data.users : []"
            @editing="disableRefresh = $event"
          />
        </DataLoader>
      </PageBody>
    </template>
  </PageLayout>
</template>

<script lang="ts" setup>
import { ref } from 'vue';
import { globalResourcePermissions } from '~/configs/permissions.config';
import { AccessPolicy, BasicUser, UserGroup } from '~/generated/wallet/wallet.did';
import { ProposalDomains } from '~/types';
import DataLoader from '~/ui/components/DataLoader.vue';
import PageLayout from '~/ui/components/PageLayout.vue';
import PageBody from '~/ui/components/layouts/PageBody.vue';
import PageHeader from '~/ui/components/layouts/PageHeader.vue';
import AccessPolicyList from '~/ui/components/permissions/AccessPolicyList.vue';
import RecentProposals from '~/ui/components/proposals/RecentProposals.vue';
import { Routes } from '~/ui/config/routes';
import { i18n } from '~/ui/modules';
import { useWalletStore } from '~/ui/stores/wallet';
import { BreadCrumbItem } from '~/ui/types/navigation';

const props = withDefaults(
  defineProps<{
    title?: string;
    breadcrumbs?: BreadCrumbItem[];
  }>(),
  {
    title: i18n.global.t('pages.permissions.title'),
    breadcrumbs: () => [],
  },
);

const wallet = useWalletStore();
const disableRefresh = ref(false);
const resources = globalResourcePermissions();

const fetchAccessPolicies = async (): Promise<{
  policies: AccessPolicy[];
  userGroups: UserGroup[];
  users: BasicUser[];
}> => {
  const userGroups: UserGroup[] = [];
  const users: BasicUser[] = [];
  let policies: AccessPolicy[] = [];
  let limit = 500;
  let nextOffset = BigInt(0);
  let maxOffsetFound = nextOffset;

  do {
    // This is to avoid infinite loops in case the offset is not updated properly
    maxOffsetFound = nextOffset;

    const result = await wallet.service.listAccessPolicies({
      limit: [limit],
      offset: [nextOffset],
    });

    userGroups.push(...result.user_groups);
    users.push(...result.users);

    policies = policies.concat(result.policies);
    nextOffset =
      result.next_offset?.[0] !== undefined && result.next_offset[0] > 0
        ? result.next_offset[0]
        : BigInt(-1);
  } while (nextOffset > 0 && nextOffset > maxOffsetFound);

  return { policies, userGroups, users };
};
</script>
