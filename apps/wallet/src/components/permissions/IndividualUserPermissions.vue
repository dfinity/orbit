<template>
  <VAutocomplete
    v-model="selectedUserId"
    class="mt-2 px-2"
    name="user_id"
    :label="$t('terms.user')"
    :loading="autocomplete.loading.value"
    :items="userList"
    chips
    clearable
    @update:search="autocomplete.searchItems"
  />
  <DataLoader
    v-if="selectedUserId"
    v-slot="{ data, loading }"
    :load="() => fetchPolicies(useResourcesFromAggregatedView(resources))"
    :refresh-interval-ms="5000"
    :disable-refresh="disableRefresh"
  >
    <PermissionList
      :loading="loading"
      :resources="resources"
      :permissions="data ? data.policies : []"
      :privileges="data ? data.privileges : []"
      :preload-user-groups="data ? data.userGroups : []"
      :preload-users="data ? data.users : []"
      @editing="disableRefresh = $event"
    />
  </DataLoader>
</template>

<script lang="ts" setup>
import { computed, onMounted, ref, toRefs, watch } from 'vue';
import DataLoader from '~/components/DataLoader.vue';
import { useUsersAutocomplete } from '~/composables/autocomplete.composable';
import { getUserPermissions } from '~/configs/permissions.config';
import {
  Permission,
  PermissionCallerPrivileges,
  BasicUser,
  Resource,
  UUID,
  UserGroup,
} from '~/generated/station/station.did';
import { AggregatedResoucePermissions } from '~/types/permissions.types';
import PermissionList from './PermissionList.vue';
import { useResourcesFromAggregatedView } from '~/composables/permissions.composable';
import { VAutocomplete } from 'vuetify/components';

const autocomplete = useUsersAutocomplete();
const selectedUserId = ref<UUID | null>(null);
const resources = ref<AggregatedResoucePermissions[]>([]);
const disableRefresh = ref(false);

onMounted(() => {
  autocomplete.searchItems();
});

const props = withDefaults(
  defineProps<{
    fetchPolicies?: (resources: Resource[]) => Promise<{
      policies: Permission[];
      userGroups: UserGroup[];
      users: BasicUser[];
      privileges: PermissionCallerPrivileges[];
    }>;
  }>(),
  {
    fetchPolicies: () =>
      Promise.resolve({ policies: [], userGroups: [], users: [], privileges: [] }),
  },
);

const { fetchPolicies } = toRefs(props);

const userList = computed(() => {
  const users = autocomplete.results.value.map(user => ({
    title: user.name,
    value: user.id,
  }));

  return users;
});

watch(
  () => selectedUserId.value,
  () => {
    if (selectedUserId.value) {
      resources.value = getUserPermissions(selectedUserId.value);
    } else {
      resources.value = [];
    }
  },
);
</script>
