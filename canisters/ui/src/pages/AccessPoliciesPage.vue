<template>
  <PageLayout>
    <template #main-header>
      <PageHeader :title="title" :breadcrumbs="props.breadcrumbs" />
    </template>

    <template #main-body>
      <PageBody>
        <VContainer fluid class="px-0">
          <VRow>
            <VCol cols="12">
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
            <VCol cols="12">
              <DataLoader
                v-slot="{ data, loading }"
                :load="fetchAccessPolicies"
                :refresh-interval-ms="5000"
                :disable-refresh="disableRefresh"
              >
                <AccessPolicyList
                  :loading="loading"
                  :resources="resourceAccessPolicies"
                  :access-policies="data ? data.policies : []"
                  :privileges="data ? data.privileges : []"
                  :preload-user-groups="data ? data.userGroups : []"
                  :preload-users="data ? data.users : []"
                  @editing="disableRefresh = $event"
                />
              </DataLoader>
            </VCol>
            <VCol cols="12">
              <div class="text-h6 px-2">{{ $t('access_policies.individual_resources_title') }}</div>

              <VAutocomplete
                v-model="individualResourceSelected"
                class="px-2 mt-2"
                :items="individualResources"
                density="comfortable"
                variant="underlined"
                :label="$t('access_policies.select_resource')"
                hide-details
              />

              <IndividualAccountAccessPolicies
                v-if="individualResourceSelected === ResourceTypeEnum.Account"
                :fetch-policies="fetchAccessPolicies"
              />
              <IndividualUserAccessPolicies
                v-else-if="individualResourceSelected === ResourceTypeEnum.User"
                :fetch-policies="fetchAccessPolicies"
              />
              <IndividualUserGroupAccessPolicies
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
import { computed, ref } from 'vue';
import { useI18n } from 'vue-i18n';
import DataLoader from '~/components/DataLoader.vue';
import PageLayout from '~/components/PageLayout.vue';
import AccessPolicyList from '~/components/access-policies/AccessPolicyList.vue';
import IndividualAccountAccessPolicies from '~/components/access-policies/IndividualAccountAccessPolicies.vue';
import IndividualUserAccessPolicies from '~/components/access-policies/IndividualUserAccessPolicies.vue';
import IndividualUserGroupAccessPolicies from '~/components/access-policies/IndividualUserGroupAccessPolicies.vue';
import PageBody from '~/components/layouts/PageBody.vue';
import PageHeader from '~/components/layouts/PageHeader.vue';
import RecentProposals from '~/components/proposals/RecentProposals.vue';
import { globalAccessPolicies } from '~/configs/access-policies.config';
import { Routes } from '~/configs/routes.config';
import {
  AccessPolicy,
  AccessPolicyCallerPrivileges,
  BasicUser,
  UserGroup,
} from '~/generated/wallet/wallet.did';
import { useWalletStore } from '~/stores/wallet.store';
import { ResourceTypeEnum } from '~/types/access-policies.types';
import type { PageProps } from '~/types/app.types';
import { ProposalDomains } from '~/types/wallet.types';

const props = withDefaults(defineProps<PageProps>(), {
  title: undefined,
  breadcrumbs: () => [],
});

const i18n = useI18n();
const title = computed(() => props.title || i18n.t('pages.access_policies.title'));
const wallet = useWalletStore();
const disableRefresh = ref(false);
const resourceAccessPolicies = globalAccessPolicies();
const individualResourceSelected = ref<ResourceTypeEnum | null>(null);
const individualResourceKeys = ref<ResourceTypeEnum[]>([
  ResourceTypeEnum.Account,
  ResourceTypeEnum.User,
  ResourceTypeEnum.UserGroup,
]);

const individualResources = computed(() => {
  return individualResourceKeys.value.map(key => ({
    value: key,
    title: i18n.t(`access_policies.resources.${key.toLowerCase()}`),
  }));
});

const fetchAccessPolicies = async (): Promise<{
  policies: AccessPolicy[];
  privileges: AccessPolicyCallerPrivileges[];
  userGroups: UserGroup[];
  users: BasicUser[];
}> => {
  const userGroups: UserGroup[] = [];
  const users: BasicUser[] = [];
  let policies: AccessPolicy[] = [];
  let privileges: AccessPolicyCallerPrivileges[] = [];
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
    privileges.push(...result.privileges);

    policies = policies.concat(result.policies);
    nextOffset =
      result.next_offset?.[0] !== undefined && result.next_offset[0] > 0
        ? result.next_offset[0]
        : BigInt(-1);
  } while (nextOffset > 0 && nextOffset > maxOffsetFound);

  return { policies, userGroups, users, privileges };
};
</script>
