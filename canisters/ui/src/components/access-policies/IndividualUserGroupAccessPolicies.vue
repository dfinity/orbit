<template>
  <VAutocomplete
    v-model="selectedUserGroupId"
    class="mt-2 px-2"
    name="user_group_id"
    :label="$t('terms.user_group')"
    :loading="autocomplete.loading.value"
    variant="underlined"
    :items="groupList"
    chips
    clearable
    @update:search="autocomplete.searchItems"
  />
  <DataLoader
    v-if="selectedUserGroupId"
    v-slot="{ data, loading }"
    :load="fetchPolicies"
    :refresh-interval-ms="5000"
    :disable-refresh="disableRefresh"
  >
    <AccessPolicyList
      :loading="loading"
      :resources="resources"
      :access-policies="data ? data.policies : []"
      :preload-user-groups="data ? data.userGroups : []"
      :preload-users="data ? data.users : []"
      @editing="disableRefresh = $event"
    />
  </DataLoader>
</template>

<script lang="ts" setup>
import { computed, onMounted, ref, toRefs, watch } from 'vue';
import { getUserGroupAccessPolicies } from '~/configs/access-policies.config';
import { AccessPolicy, BasicUser, UUID, UserGroup } from '~/generated/wallet/wallet.did';
import { AggregatedResouceAccessPolicies } from '~/types/access-policies.types';
import DataLoader from '~/components/DataLoader.vue';
import { useUserGroupsAutocomplete } from '~/composables/autocomplete.composable';
import AccessPolicyList from './AccessPolicyList.vue';

const autocomplete = useUserGroupsAutocomplete();
const selectedUserGroupId = ref<UUID | null>(null);
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
    }>;
  }>(),
  {
    fetchPolicies: () => Promise.resolve({ policies: [], userGroups: [], users: [] }),
  },
);

const { fetchPolicies } = toRefs(props);

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
      resources.value = getUserGroupAccessPolicies(selectedUserGroupId.value);
    } else {
      resources.value = [];
    }
  },
);
</script>
