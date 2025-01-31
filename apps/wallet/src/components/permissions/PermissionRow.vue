<template>
  <tr>
    <td class="w-100 py-1 text-body-2">
      <slot>
        <p>
          {{ title }}
          <VIcon
            v-if="props.type === 'action'"
            :icon="mdiProgressPencil"
            size="x-small"
            class="text-medium-emphasis"
          />
        </p>
        <p v-if="explainer" class="text-caption text-medium-emphasis">
          {{ explainer }}
        </p>
      </slot>
    </td>
    <td class="text-center">
      <slot name="actions">
        <VIcon v-if="isPubliclyAvailable" :icon="mdiEarth" size="small" />
        <VIcon v-if="isAvailableToAllAuthenticatedUsers" :icon="mdiAccountLock" size="small" />
        <template v-else-if="!resourceIsAvailableToNoUser">
          <VChip
            v-if="props.allowed.user_groups.length > 0"
            size="small"
            variant="text"
            density="comfortable"
            :prepend-icon="mdiAccountGroup"
          >
            <TextOverflow
              :text="props.db.userGroupsById[props.allowed.user_groups[0]]?.name"
              :max-length="12"
            />
            <span v-if="remainingUserGroups">+ {{ remainingUserGroups }}</span>
          </VChip>
          <VChip
            v-if="props.allowed.users.length > 0"
            size="small"
            variant="text"
            density="comfortable"
            :prepend-icon="mdiAccount"
          >
            <TextOverflow
              :text="props.db.usersById[props.allowed.users[0]]?.name"
              :max-length="12"
            />
            <span v-if="remainingUsers">+ {{ remainingUsers }}</span>
          </VChip>
        </template>
      </slot>
    </td>
  </tr>
</template>
<script lang="ts" setup>
import { mdiAccount, mdiAccountGroup, mdiAccountLock, mdiEarth, mdiProgressPencil } from '@mdi/js';
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import { VChip, VIcon } from 'vuetify/components';
import { Allow, BasicUser, Resource, UserGroup } from '~/generated/station/station.did';
import { fromResourceToDisplayText } from '~/mappers/permissions.mapper';
import { variantIs } from '~/utils/helper.utils';
import TextOverflow from '../TextOverflow.vue';

const props = withDefaults(
  defineProps<{
    resource: Resource;
    type?: 'view' | 'action';
    allowed?: Allow;
    hover?: boolean;
    db?: {
      usersById: Record<string, BasicUser>;
      userGroupsById: Record<string, UserGroup>;
    };
  }>(),
  {
    db: () => ({ usersById: {}, userGroupsById: {} }),
    allowed: () => ({ auth_scope: { Restricted: null }, users: [], user_groups: [] }),
    type: 'view',
    description: undefined,
    hover: false,
  },
);

const i18n = useI18n();
const title = computed(() =>
  i18n.te(`permissions.actions.${fromResourceToDisplayText(props.resource)}`)
    ? i18n.t(`permissions.actions.${fromResourceToDisplayText(props.resource)}`)
    : fromResourceToDisplayText(props.resource),
);
const explainer = computed(() =>
  i18n.te(`permissions.actions.${fromResourceToDisplayText(props.resource)}_description`)
    ? i18n.t(`permissions.actions.${fromResourceToDisplayText(props.resource)}_description`)
    : undefined,
);

const isPubliclyAvailable = computed(() => variantIs(props.allowed.auth_scope, 'Public'));
const isAvailableToAllAuthenticatedUsers = computed(() =>
  variantIs(props.allowed.auth_scope, 'Authenticated'),
);
const isAvailableToSubsetOfUsers = computed(() =>
  variantIs(props.allowed.auth_scope, 'Restricted'),
);

const resourceIsAvailableToNoUser = computed(
  () =>
    isAvailableToSubsetOfUsers.value &&
    props.allowed.users.length === 0 &&
    props.allowed.user_groups.length === 0,
);

const remainingUserGroups = computed(() => Math.max(0, props.allowed.user_groups.length - 1));
const remainingUsers = computed(() => Math.max(0, props.allowed.users.length - 1));

export interface SelectedPermission {
  allowed: Allow;
  resource: Resource;
}
</script>
