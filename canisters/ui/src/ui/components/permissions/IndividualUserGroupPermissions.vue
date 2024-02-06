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
import { ResourcePermissions, getUserGroupResourcePermissions } from '~/configs/permissions.config';
import { AccessPolicy, BasicUser, UUID, UserGroup } from '~/generated/wallet/wallet.did';
import DataLoader from '~/ui/components/DataLoader.vue';
import AccessPolicyList from '~/ui/components/permissions/AccessPolicyList.vue';
import { useUserGroupsAutocomplete } from '~/ui/composables/autocomplete.composable';

const autocomplete = useUserGroupsAutocomplete();
const selectedUserGroupId = ref<UUID | null>(null);
const resources = ref<ResourcePermissions[]>([]);
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
      resources.value = getUserGroupResourcePermissions(selectedUserGroupId.value);
    } else {
      resources.value = [];
    }
  },
);
</script>
