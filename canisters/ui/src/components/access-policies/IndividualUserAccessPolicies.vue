<template>
  <VAutocomplete
    v-model="selectedUserId"
    class="mt-2 px-2"
    name="user_id"
    :label="$t('terms.user')"
    :loading="autocomplete.loading.value"
    variant="underlined"
    :items="userList"
    chips
    clearable
    @update:search="autocomplete.searchItems"
  />
  <DataLoader
    v-if="selectedUserId"
    v-slot="{ data, loading }"
    :load="fetchPolicies"
    :refresh-interval-ms="5000"
    :disable-refresh="disableRefresh"
  >
    <AccessPolicyList
      :loading="loading"
      :resources="resources"
      :access-policies="data ? data.policies : []"
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
import { getUserAccessPolicies } from '~/configs/access-policies.config';
import {
  AccessPolicy,
  AccessPolicyCallerPrivileges,
  BasicUser,
  UUID,
  UserGroup,
} from '~/generated/wallet/wallet.did';
import { AggregatedResouceAccessPolicies } from '~/types/access-policies.types';
import AccessPolicyList from './AccessPolicyList.vue';

const autocomplete = useUsersAutocomplete();
const selectedUserId = ref<UUID | null>(null);
const resources = ref<AggregatedResouceAccessPolicies[]>([]);
const disableRefresh = ref(false);

onMounted(() => {
  autocomplete.searchItems();
});

const props = withDefaults(
  defineProps<{
    fetchPolicies?: () => Promise<{
      policies: AccessPolicy[];
      userGroups: UserGroup[];
      users: BasicUser[];
      privileges: AccessPolicyCallerPrivileges[];
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
    title: user.name?.[0] ? user.name[0] : user.id,
    value: user.id,
  }));

  return users;
});

watch(
  () => selectedUserId.value,
  () => {
    if (selectedUserId.value) {
      resources.value = getUserAccessPolicies(selectedUserId.value);
    } else {
      resources.value = [];
    }
  },
);
</script>
