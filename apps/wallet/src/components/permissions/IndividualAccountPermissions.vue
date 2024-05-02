<template>
  <VAutocomplete
    v-model="selectedAccountId"
    class="mt-2 px-2"
    name="account_id"
    :label="$t('terms.account')"
    :loading="autocomplete.loading.value"
    :items="groupList"
    chips
    clearable
    @update:search="autocomplete.searchItems"
  />
  <DataLoader
    v-if="selectedAccountId"
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
import { useAccountsAutocomplete } from '~/composables/autocomplete.composable';
import { getAccountPermissions } from '~/configs/permissions.config';
import type {
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

const autocomplete = useAccountsAutocomplete();
const selectedAccountId = ref<UUID | null>(null);
const resources = ref<AggregatedResoucePermissions[]>([]);
const disableRefresh = ref(false);

onMounted(() => {
  autocomplete.searchItems();
});

const groupList = computed(() => {
  const groups = autocomplete.results.value.map(group => ({
    title: group.name,
    value: group.id,
  }));

  return groups;
});

watch(
  () => selectedAccountId.value,
  () => {
    if (selectedAccountId.value) {
      resources.value = getAccountPermissions(selectedAccountId.value);
    } else {
      resources.value = [];
    }
  },
);
</script>
