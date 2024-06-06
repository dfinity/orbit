<template>
  <VContainer fluid data-test-id="permission-list" class="px-3 pt-2">
    <VRow>
      <VCol cols="12" class="px-0 pt-0">
        <VTable density="compact" hover class="elevation-2 rounded">
          <thead>
            <tr v-if="!app.isMobile">
              <th class="w-50">{{ $t(`permissions.resource_title`) }}</th>
              <th>{{ $t(`permissions.group_members_title`) }}</th>
              <th>{{ $t(`permissions.specific_users_title`) }}</th>
              <th>{{ $t(`permissions.everyone_title`) }}</th>
            </tr>
            <tr v-else data-test-id="mobile-table-headers">
              <th>{{ $t(`permissions.resource_title`) }}</th>
            </tr>
          </thead>
          <tbody>
            <tr v-if="loading">
              <td colspan="4" class="bb-none pt-4 pb-4">
                <VProgressCircular indeterminate color="primary" />
              </td>
            </tr>
            <PermissionListItem
              v-for="(resourcePermission, idx) in aggregatedPermissions"
              v-else
              :key="idx"
              :resource="resourcePermission"
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
import { defaultAllowLevels } from '~/configs/permissions.config';
import { logger } from '~/core/logger.core';
import {
  Permission,
  PermissionCallerPrivileges,
  BasicUser,
  Resource,
  UUID,
  UserGroup,
} from '~/generated/station/station.did';
import { AggregatedResoucePermissions } from '~/types/permissions.types';
import { useAppStore } from '~/stores/app.store';
import PermissionListItem from './PermissionListItem.vue';
import { toAuthScopeEnum } from '~/mappers/permissions.mapper';
import { VCol, VContainer, VProgressCircular, VRow, VTable } from 'vuetify/components';

const app = useAppStore();
const props = withDefaults(
  defineProps<{
    resources: AggregatedResoucePermissions[];
    permissions: Permission[];
    privileges: PermissionCallerPrivileges[];
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

const { preloadUserGroups, preloadUsers, permissions, resources, privileges, loading } =
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

const aggregatedPermissions = computed<AggregatedResoucePermissions[]>(() => {
  const resourcePermissions = resources.value.map(resource => ({
    match: resource.match,
    resourceType: resource.resourceType,
    resources: resource.resources.map(resource => ({
      ...resource,
      users: defaultAllowLevels(),
    })),
  }));

  const hasPermissions = permissions.value.length > 0;
  if (!hasPermissions) {
    return resourcePermissions;
  }

  for (const aggregatedResource of resourcePermissions) {
    for (const resource of aggregatedResource.resources) {
      let policy = permissions.value.find(policy =>
        aggregatedResource.match(resource.resource, policy.resource),
      );
      if (!policy) {
        logger.warn(
          `No match found for policy. This should not happen. Policy: ${JSON.stringify(resource.resource)}`,
        );

        continue;
      }

      resource.canEdit = hasEditPrivilege(resource.resource);
      resource.allow.authScope = toAuthScopeEnum(policy.allow.auth_scope);
      resource.allow.specificUsers =
        policy.allow.users.map(id => {
          const user = users.value[id];
          if (!user) {
            logger.warn(`User with id ${id} not found in preload data. This should not happen.`);
          }
          return user;
        }) ?? [];

      resource.allow.membersOfGroup =
        policy.allow.user_groups.map(id => {
          const group = userGroups.value[id];
          if (!group) {
            logger.warn(`Group with id ${id} not found in preload data. This should not happen.`);
          }
          return group;
        }) ?? [];
    }
  }

  return resourcePermissions;
});

const emit = defineEmits<{
  (event: 'editing', payload: boolean): void;
}>();
</script>
