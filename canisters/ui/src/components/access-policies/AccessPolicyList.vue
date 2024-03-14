<template>
  <VContainer fluid data-test-id="access-policy-list">
    <VRow>
      <VCol cols="12" class="px-0 pt-0">
        <VTable density="compact" hover>
          <thead>
            <tr v-if="!app.isMobile">
              <th class="w-50">{{ $t(`access_policies.resource_title`) }}</th>
              <th>{{ $t(`access_policies.group_members_title`) }}</th>
              <th>{{ $t(`access_policies.specific_users_title`) }}</th>
              <th>{{ $t(`access_policies.everyone_title`) }}</th>
            </tr>
            <tr v-else data-test-id="mobile-table-headers">
              <th>{{ $t(`access_policies.resource_title`) }}</th>
            </tr>
          </thead>
          <tbody>
            <tr v-if="loading">
              <td colspan="4" class="bb-none pt-4 pb-4">
                <VProgressCircular indeterminate color="primary" />
              </td>
            </tr>
            <AccessPolicyListItem
              v-for="(resourceAccessPolicy, idx) in aggregatedAccessPolicies"
              v-else
              :key="idx"
              :resource="resourceAccessPolicy"
              @editing="emit('editing', $event)"
            />
          </tbody>
        </VTable>
      </VCol>
    </VRow>
  </VContainer>
</template>

<script lang="ts" setup>
import { computed, toRefs } from 'vue';
import { defaultAllowLevels } from '~/configs/access-policies.config';
import { logger } from '~/core/logger.core';
import {
  AccessPolicy,
  AccessPolicyCallerPrivileges,
  BasicUser,
  Resource,
  UUID,
  UserGroup,
} from '~/generated/wallet/wallet.did';
import {
  AccessPolicyForAllUsers,
  AggregatedResouceAccessPolicies,
} from '~/types/access-policies.types';
import { useAppStore } from '~/stores/app.store';
import AccessPolicyListItem from './AccessPolicyListItem.vue';
import { variantIs } from '~/utils/helper.utils';

const app = useAppStore();
const props = withDefaults(
  defineProps<{
    resources: AggregatedResouceAccessPolicies[];
    accessPolicies: AccessPolicy[];
    privileges: AccessPolicyCallerPrivileges[];
    preloadUserGroups?: UserGroup[];
    preloadUsers?: BasicUser[];
    loading?: boolean;
  }>(),
  {
    preloadUserGroups: () => [],
    preloadUsers: () => [],
    loading: false,
  },
);

const { preloadUserGroups, preloadUsers, accessPolicies, resources, privileges, loading } =
  toRefs(props);

const userGroups = computed<Record<UUID, UserGroup>>(() => {
  return preloadUserGroups.value.reduce<Record<UUID, UserGroup>>((acc, group) => {
    acc[group.id] = group;
    return acc;
  }, {});
});

const users = computed<Record<UUID, BasicUser>>(() => {
  return preloadUsers.value.reduce<Record<UUID, BasicUser>>((acc, user) => {
    acc[user.id] = user;
    return acc;
  }, {});
});

const hasEditPrivilege = (resource: Resource): boolean => {
  const privilege = privileges.value.find(
    privilege => JSON.stringify(privilege.resource) === JSON.stringify(resource),
  );

  return privilege?.can_edit ?? false;
};

const aggregatedAccessPolicies = computed<AggregatedResouceAccessPolicies[]>(() => {
  const resourceAccessPolicies = resources.value.map(resource => ({
    match: resource.match,
    resourceType: resource.resourceType,
    resources: resource.resources.map(resource => ({
      ...resource,
      users: defaultAllowLevels(),
    })),
  }));

  const hasAccessPolicies = accessPolicies.value.length > 0;
  if (!hasAccessPolicies) {
    return resourceAccessPolicies;
  }

  for (const aggregatedResource of resourceAccessPolicies) {
    for (const resource of aggregatedResource.resources) {
      let policy = accessPolicies.value.find(policy =>
        aggregatedResource.match(resource.resource, policy.resource),
      );
      if (!policy) {
        logger.warn(
          `No match found for policy. This should not happen. Policy: ${JSON.stringify(resource.resource)}`,
        );

        continue;
      }

      resource.canEdit = hasEditPrivilege(resource.resource);

      if (policy.allow.authentication?.length) {
        let authentication = policy.allow.authentication[0];
        if (variantIs(authentication, 'Required')) {
          resource.allow.allUsers = AccessPolicyForAllUsers.AuthenticationRequired;
        } else if (variantIs(authentication, 'None')) {
          resource.allow.allUsers = AccessPolicyForAllUsers.Public;
        }
      }

      resource.allow.specificUsers =
        policy.allow.users?.[0]?.map(id => {
          const user = users.value[id];
          if (!user) {
            logger.warn(`User with id ${id} not found in preload data. This should not happen.`);
          }
          return user;
        }) ?? [];

      resource.allow.membersOfGroup =
        policy.allow.user_groups?.[0]?.map(id => {
          const group = userGroups.value[id];
          if (!group) {
            logger.warn(`Group with id ${id} not found in preload data. This should not happen.`);
          }
          return group;
        }) ?? [];
    }
  }

  return resourceAccessPolicies;
});

const emit = defineEmits<{
  (event: 'editing', payload: boolean): void;
}>();
</script>
