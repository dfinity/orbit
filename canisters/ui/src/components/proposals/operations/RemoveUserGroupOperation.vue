<template>
  <div v-if="isListMode" class="d-flex flex-column ga-0 text-caption">
    <ProposalOperationListColumn v-if="formValue.id">
      <template #name>{{ $t('terms.id') }}</template>
      <template #content>
        {{ formValue.id }}
      </template>
    </ProposalOperationListColumn>
  </div>
  <VProgressCircular v-else-if="loading" />
  <UserGroupForm v-else :model-value="formValue" mode="view" />
</template>

<script setup lang="ts">
import { Ref, computed, onBeforeMount, ref } from 'vue';
import UserGroupForm from '~/components/users/UserGroupForm.vue';
import logger from '~/core/logger.core';
import { Proposal, RemoveUserGroupOperation, UserGroup } from '~/generated/wallet/wallet.did';
import { useWalletStore } from '~/stores/wallet.store';
import ProposalOperationListColumn from '../ProposalOperationListColumn.vue';

const props = withDefaults(
  defineProps<{
    proposal: Proposal;
    operation: RemoveUserGroupOperation;
    mode?: 'list' | 'detail';
  }>(),
  {
    mode: 'list',
  },
);

const isListMode = computed(() => props.mode === 'list');
const formValue: Ref<Partial<UserGroup>> = ref({});
const loading = ref(false);
const wallet = useWalletStore();

const fetchDetails = async () => {
  try {
    if (loading.value || isListMode.value) {
      return;
    }

    loading.value = true;
    const currentEntry = await wallet.service.getUserGroup({
      user_group_id: props.operation.input.user_group_id,
    });

    formValue.value = currentEntry.user_group;
  } catch (e) {
    logger.error('Failed to fetch user group details', e);
  } finally {
    loading.value = false;
  }
};

onBeforeMount(() => {
  const entry: Partial<UserGroup> = {};
  entry.id = props.operation.input.user_group_id;

  formValue.value = entry;

  fetchDetails();
});
</script>
