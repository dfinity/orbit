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
import { useAccountsAutocomplete } from '~/composables/autocomplete.composable';
import { getAccountAccessPolicies } from '~/configs/access-policies.config';
import type {
  AccessPolicy,
  AccessPolicyCallerPrivileges,
  BasicUser,
  Resource,
  UUID,
  UserGroup,
} from '~/generated/wallet/wallet.did';
import { AggregatedResouceAccessPolicies } from '~/types/access-policies.types';
import AccessPolicyList from './AccessPolicyList.vue';
import { useResourcesFromAggregatedView } from '~/composables/access-policies.composable';
import { VAutocomplete } from 'vuetify/components';

const props = withDefaults(
  defineProps<{
    fetchPolicies?: (resources: Resource[]) => Promise<{
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

const autocomplete = useAccountsAutocomplete();
const selectedAccountId = ref<UUID | null>(null);
const resources = ref<AggregatedResouceAccessPolicies[]>([]);
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
      resources.value = getAccountAccessPolicies(selectedAccountId.value);
    } else {
      resources.value = [];
    }
  },
);
</script>
