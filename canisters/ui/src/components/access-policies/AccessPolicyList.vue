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
import { defaultUserSpecifiers } from '~/configs/access-policies.config';
import { logger } from '~/core/logger.core';
import { variantIs } from '~/utils/helper.utils';
import { AccessPolicy, BasicUser, UUID, UserGroup } from '~/generated/wallet/wallet.did';
import { AggregatedResouceAccessPolicies } from '~/types/access-policies.types';
import { useAppStore } from '~/stores/app.store';
import AccessPolicyListItem from './AccessPolicyListItem.vue';

const app = useAppStore();

const props = withDefaults(
  defineProps<{
    resources: AggregatedResouceAccessPolicies[];
    accessPolicies: AccessPolicy[];
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

const { preloadUserGroups, preloadUsers, accessPolicies, resources } = toRefs(props);

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

const resourceAccessPolicies = computed<AggregatedResouceAccessPolicies[]>(() => {
  const resourceAccessPolicies = resources.value.map(resource => ({
    match: resource.match,
    resourceType: resource.resourceType,
    specifiers: resource.specifiers.map(specifier => ({
      ...specifier,
      users: defaultUserSpecifiers(),
    })),
  }));

  for (const policy of accessPolicies.value) {
    for (const resource of resourceAccessPolicies) {
      for (const resourceSpecifier of resource.specifiers) {
        if (resource.match(resourceSpecifier.specifier, policy)) {
          if (variantIs(policy.user, 'Any')) {
            resourceSpecifier.users.allUsers.policy.id = policy.id;
            resourceSpecifier.users.allUsers.policy.canEdit = policy.info.can_edit;
            resourceSpecifier.users.allUsers.policy.canRemove = policy.info.can_delete;
          } else if (variantIs(policy.user, 'Id')) {
            resourceSpecifier.users.specificUsers.policy.id = policy.id;
            resourceSpecifier.users.specificUsers.policy.canEdit = policy.info.can_edit;
            resourceSpecifier.users.specificUsers.policy.canRemove = policy.info.can_delete;
            resourceSpecifier.users.specificUsers.users = policy.user.Id.map(id => {
              const user = users.value[id];
              if (!user) {
                logger.warn(
                  `User with id ${id} not found in preload data. This should not happen.`,
                );
              }
              return user;
            });
          } else if (variantIs(policy.user, 'Group')) {
            resourceSpecifier.users.membersOfGroup.policy.id = policy.id;
            resourceSpecifier.users.membersOfGroup.policy.canEdit = policy.info.can_edit;
            resourceSpecifier.users.membersOfGroup.policy.canRemove = policy.info.can_delete;
            resourceSpecifier.users.membersOfGroup.groups = policy.user.Group.map(id => {
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
