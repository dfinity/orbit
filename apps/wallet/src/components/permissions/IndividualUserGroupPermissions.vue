<template>
  <VAutocomplete
    v-model="selectedUserGroupId"
    v-bind="$attrs"
    class="mt-2 px-2"
    name="user_group_id"
    :label="$t('terms.user_group')"
    :loading="autocomplete.loading.value"
    :items="groupList"
    chips
    clearable
    @update:search="autocomplete.searchItems"
  />
  <DataLoader
    v-if="selectedUserGroupId"
    v-slot="{ data, loading }"
    :load="() => fetchPermissions(useResourcesFromAggregatedView(resources))"
    :refresh-interval-ms="5000"
    :disable-refresh="disableRefresh"
  >
    <PermissionList
      :loading="loading"
      :resources="resources"
      :permissions="data ? data.permissions : []"
      :privileges="data ? data.privileges : []"
      :preload-user-groups="data ? data.userGroups : []"
      :preload-users="data ? data.users : []"
      @editing="disableRefresh = $event"
    />
  </DataLoader>
</template>

<script lang="ts" setup>
import { computed, onMounted, Ref, ref, toRefs, watch } from 'vue';
import { VAutocomplete } from 'vuetify/components';
import DataLoader from '~/components/DataLoader.vue';
import { useUserGroupsAutocomplete } from '~/composables/autocomplete.composable';
import { useResourcesFromAggregatedView } from '~/composables/permissions.composable';
import { getUserGroupPermissions } from '~/configs/permissions.config';
import {
  BasicUser,
  Permission,
  PermissionCallerPrivileges,
  Resource,
  UserGroup,
  UUID,
} from '~/generated/station/station.did';
import { AggregatedResoucePermissions } from '~/types/permissions.types';
import PermissionList from './PermissionList.vue';

const autocomplete = useUserGroupsAutocomplete();
const selectedUserGroupId = ref<UUID | null>(null);
const resources: Ref<AggregatedResoucePermissions[]> = ref([]);
const disableRefresh = ref(false);

onMounted(() => {
  autocomplete.searchItems();
});

const props = withDefaults(
  defineProps<{
    fetchPermissions?: (resources: Resource[]) => Promise<{
      permissions: Permission[];
      userGroups: UserGroup[];
      users: BasicUser[];
      privileges: PermissionCallerPrivileges[];
    }>;
  }>(),
  {
    fetchPermissions: () =>
      Promise.resolve({ permissions: [], userGroups: [], users: [], privileges: [] }),
  },
);

const { fetchPermissions } = toRefs(props);

const groupList = computed(() => {
  const groups = autocomplete.results.value.map(group => ({
    title: group.name,
    value: group.id,
  }));

  return groups;
});

watch(
  () => selectedUserGroupId.value,
  () => {
    if (selectedUserGroupId.value) {
      resources.value = getUserGroupPermissions(selectedUserGroupId.value);
    } else {
      resources.value = [];
    }
  },
);
</script>
