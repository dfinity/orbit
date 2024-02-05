<template>
  <VContainer>
    <VRow fluid>
      <VCol cols="12" class="px-0 pt-0">
        <VTable density="compact" hover>
          <thead>
            <tr>
              <th class="w-50">{{ $t(`permissions.resource_title`) }}</th>
              <th>{{ $t(`permissions.group_members_title`) }}</th>
              <th>{{ $t(`permissions.specific_users_title`) }}</th>
              <th>{{ $t(`permissions.everyone_title`) }}</th>
            </tr>
          </thead>
          <tbody>
            <tr v-if="loading">
              <td colspan="4" class="bb-none pt-4 pb-4">
                <VProgressCircular indeterminate color="primary" />
              </td>
            </tr>
            <ResourcePermissionsRow
              v-for="(resource, idx) in resourcePermissions"
              v-else
              :key="idx + updatedResourceListKey"
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
import { computed, ref, watch } from 'vue';
import { ResourcePermissions } from '~/configs/permissions.config';
import { logger, variantIs } from '~/core';
import { AccessPolicy, BasicUser, UUID, UserGroup } from '~/generated/wallet/wallet.did';
import ResourcePermissionsRow from './ResourcePermissionsRow.vue';

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

const userGroups = computed<Record<UUID, UserGroup>>(() => {
  return props.preloadUserGroups.reduce<Record<UUID, UserGroup>>((acc, group) => {
    acc[group.id] = group;
    return acc;
  }, {});
});

const users = computed<Record<UUID, BasicUser>>(() => {
  return props.preloadUsers.reduce<Record<UUID, BasicUser>>((acc, user) => {
    acc[user.id] = user;
    return acc;
  }, {});
});

const updatedResourceListKey = ref(0);
const resourcePermissions = ref<ResourcePermissions[]>([]);

const updateResourceList = (): void => {
  const resourceAccessPolicies = props.resources;
  for (const policy of props.accessPolicies) {
    for (const resource of resourceAccessPolicies) {
      for (const resourceSpecifier of resource.specifiers) {
        if (resource.match(resourceSpecifier.specifier, policy)) {
          if (variantIs(policy.user, 'Any')) {
            resourceSpecifier.users.allUsers.policyId = policy.id;
          } else if (variantIs(policy.user, 'Id')) {
            resourceSpecifier.users.specificUsers.policyId = policy.id;
            resourceSpecifier.users.specificUsers.users = policy.user.Id.reduce<
              Record<UUID, BasicUser>
            >((acc, id) => {
              let user = users.value[id];
              if (!user) {
                user = { id, name: '-', status: { Active: null } };

                logger.warn(
                  `User with id ${id} not found in preload data. This should not happen.`,
                );
              }

              acc[id] = user;
              return acc;
            }, {});
          } else if (variantIs(policy.user, 'Group')) {
            resourceSpecifier.users.membersOfGroup.policyId = policy.id;
            resourceSpecifier.users.membersOfGroup.groups = policy.user.Group.reduce<
              Record<UUID, UserGroup>
            >((acc, id) => {
              let group = userGroups.value[id];
              if (!group) {
                group = { id, name: '-' };

                logger.warn(
                  `Group with id ${id} not found in preload data. This should not happen.`,
                );
              }

              acc[id] = group;
              return acc;
            }, {});
          }
        }
      }
    }
  }

  resourcePermissions.value = resourceAccessPolicies;
};

watch(
  () => props.accessPolicies,
  () => {
    updateResourceList();

    updatedResourceListKey.value++;
  },
  { deep: true },
);

const emit = defineEmits<{
  (event: 'editing', payload: boolean): void;
}>();
</script>
