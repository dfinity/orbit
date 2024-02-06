<template>
  <PageLayout>
    <template #main-header>
      <PageHeader :title="title" :breadcrumbs="props.breadcrumbs" />
    </template>

    <template #main-body>
      <PageBody>
        <VContainer fluid class="px-0">
          <VRow>
            <VCol cols="12" class="w-100">
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
            </VCol>
            <VCol cols="12" class="w-100">
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
            </VCol>
            <VCol cols="12">
              <div class="text-h6 px-2">{{ $t('permissions.individual_resources_title') }}</div>

              <VAutocomplete
                v-model="individualResourceSelected"
                class="px-2 mt-2"
                :items="individualResources"
                density="comfortable"
                variant="underlined"
                :label="$t('permissions.select_resource')"
                hide-details
              />

              <IndividualAccountPermissions
                v-if="individualResourceSelected === ResourceTypeEnum.Account"
                :fetch-policies="fetchAccessPolicies"
              />
              <IndividualUserPermissions
                v-else-if="individualResourceSelected === ResourceTypeEnum.User"
                :fetch-policies="fetchAccessPolicies"
              />
              <IndividualUserGroupPermissions
                v-else-if="individualResourceSelected === ResourceTypeEnum.UserGroup"
                :fetch-policies="fetchAccessPolicies"
              />
            </VCol>
          </VRow>
        </VContainer>
      </PageBody>
    </template>
  </PageLayout>
</template>

<script lang="ts" setup>
import { computed } from 'vue';
import { ref } from 'vue';
import { globalResourcePermissions } from '~/configs/permissions.config';
import { AccessPolicy, BasicUser, UserGroup } from '~/generated/wallet/wallet.did';
import { ProposalDomains } from '~/types';
import { ResourceTypeEnum } from '~/types/permissions.types';
import DataLoader from '~/ui/components/DataLoader.vue';
import PageLayout from '~/ui/components/PageLayout.vue';
import PageBody from '~/ui/components/layouts/PageBody.vue';
import PageHeader from '~/ui/components/layouts/PageHeader.vue';
import AccessPolicyList from '~/ui/components/permissions/AccessPolicyList.vue';
import IndividualAccountPermissions from '~/ui/components/permissions/IndividualAccountPermissions.vue';
import IndividualUserGroupPermissions from '~/ui/components/permissions/IndividualUserGroupPermissions.vue';
import IndividualUserPermissions from '~/ui/components/permissions/IndividualUserPermissions.vue';
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
    title: undefined,
    breadcrumbs: () => [],
  },
);

const title = computed(() => {
  return props.title || i18n.global.t('pages.permissions.title');
});

const wallet = useWalletStore();
const disableRefresh = ref(false);
const resources = globalResourcePermissions();
const individualResourceSelected = ref<ResourceTypeEnum | null>(null);
const individualResourceKeys = ref<ResourceTypeEnum[]>([
  ResourceTypeEnum.Account,
  ResourceTypeEnum.User,
  ResourceTypeEnum.UserGroup,
]);

const individualResources = computed(() => {
  return individualResourceKeys.value.map(key => ({
    value: key,
    title: i18n.global.t(`permissions.resources.${key.toLowerCase()}`),
  }));
});

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
