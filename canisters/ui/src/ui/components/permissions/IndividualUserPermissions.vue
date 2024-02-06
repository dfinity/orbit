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
      :preload-user-groups="data ? data.userGroups : []"
      :preload-users="data ? data.users : []"
      @editing="disableRefresh = $event"
    />
  </DataLoader>
</template>

<script lang="ts" setup>
import { computed, onMounted, ref, toRefs, watch } from 'vue';
import { ResourcePermissions, getUserResourcePermissions } from '~/configs/permissions.config';
import { AccessPolicy, BasicUser, UUID, UserGroup } from '~/generated/wallet/wallet.did';
import DataLoader from '~/ui/components/DataLoader.vue';
import AccessPolicyList from '~/ui/components/permissions/AccessPolicyList.vue';
import { useUsersAutocomplete } from '~/ui/composables/autocomplete.composable';

const autocomplete = useUsersAutocomplete();
const selectedUserId = ref<UUID | null>(null);
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
      resources.value = getUserResourcePermissions(selectedUserId.value);
    } else {
      resources.value = [];
    }
  },
);
</script>
