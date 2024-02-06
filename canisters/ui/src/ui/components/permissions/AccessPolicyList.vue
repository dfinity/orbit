<template>
  <VContainer fluid>
    <VRow>
      <VCol cols="12" class="px-0 pt-0">
        <VTable density="compact" hover>
          <thead>
            <tr>
              <th :class="{ 'w-50': !app.isMobile }">{{ $t(`permissions.resource_title`) }}</th>
              <th v-if="!app.isMobile">{{ $t(`permissions.group_members_title`) }}</th>
              <th v-if="!app.isMobile">{{ $t(`permissions.specific_users_title`) }}</th>
              <th v-if="!app.isMobile">{{ $t(`permissions.everyone_title`) }}</th>
            </tr>
          </thead>
          <tbody>
            <tr v-if="loading">
              <td colspan="4" class="bb-none pt-4 pb-4">
                <VProgressCircular indeterminate color="primary" />
              </td>
            </tr>
            <AccessPolicyListItem
              v-for="(resource, idx) in resourcePermissions"
              v-else
              :key="idx"
              :resource="resource"
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
import { ResourcePermissions, defaultUserSpecifiers } from '~/configs/permissions.config';
import { logger, variantIs } from '~/core';
import { AccessPolicy, BasicUser, UUID, UserGroup } from '~/generated/wallet/wallet.did';
import AccessPolicyListItem from '~/ui/components/permissions/AccessPolicyListItem.vue';
import { useAppStore } from '~/ui/stores/app';

const app = useAppStore();

const props = withDefaults(
  defineProps<{
    resources: ResourcePermissions[];
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

const resourcePermissions = computed<ResourcePermissions[]>(() => {
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
            resourceSpecifier.users.allUsers.policyId = policy.id;
          } else if (variantIs(policy.user, 'Id')) {
            resourceSpecifier.users.specificUsers.policyId = policy.id;
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
            resourceSpecifier.users.membersOfGroup.policyId = policy.id;
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
