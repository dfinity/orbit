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
              v-for="(resourcePolicies, idx) in resourceAccessPolicies"
              v-else
              :key="idx"
              :resource="resourcePolicies"
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
import { variantIs } from '~/utils/helper.utils';
import {
  AccessPolicy,
  AccessPolicyCallerPrivileges,
  BasicUser,
  Resource,
  UUID,
  UserGroup,
} from '~/generated/wallet/wallet.did';
import { AggregatedResouceAccessPolicies } from '~/types/access-policies.types';
import { useAppStore } from '~/stores/app.store';
import AccessPolicyListItem from './AccessPolicyListItem.vue';

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

const { preloadUserGroups, preloadUsers, accessPolicies, resources, privileges } = toRefs(props);

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
  // todo: add logic to check if user has edit privilege
  return true;
  // return (
  //   privileges.value.find(privilege => privilege.resource_type === resource)?.can_edit ?? false
  // );
};

const resourceAccessPolicies = computed<AggregatedResouceAccessPolicies[]>(() => {
  const resourceAccessPolicies = resources.value.map(resource => ({
    match: resource.match,
    resourceType: resource.resourceType,
    resources: resource.resources.map(resource => ({
      ...resource,
      users: defaultAllowLevels(),
    })),
  }));

  for (const policy of accessPolicies.value) {
    for (const resource of resourceAccessPolicies) {
      for (const resourceSpecifier of resource.resources) {
        if (resource.match(resourceSpecifier.resource, policy)) {
          if (variantIs(policy.allow, 'Any')) {
            resourceSpecifier.users.allUsers.policy.canEdit = hasEditPrivilege(policy.resource);
          } else if (variantIs(policy.allow, 'Users')) {
            resourceSpecifier.users.specificUsers.policy.canEdit = hasEditPrivilege(
              policy.resource,
            );
            resourceSpecifier.users.specificUsers.users = policy.allow.Users.map(id => {
              const user = users.value[id];
              if (!user) {
                logger.warn(
                  `User with id ${id} not found in preload data. This should not happen.`,
                );
              }
              return user;
            });
          } else if (variantIs(policy.allow, 'UserGroups')) {
            resourceSpecifier.users.membersOfGroup.policy.canEdit = hasEditPrivilege(
              policy.resource,
            );
            resourceSpecifier.users.membersOfGroup.groups = policy.allow.UserGroups.map(id => {
              const group = userGroups.value[id];
              if (!group) {
                logger.warn(
                  `Group with id ${id} not found in preload data. This should not happen.`,
                );
              }
              return group;
            });
          }
        }
      }
    }
  }

  return resourceAccessPolicies;
});

const emit = defineEmits<{
  (event: 'editing', payload: boolean): void;
}>();
</script>
