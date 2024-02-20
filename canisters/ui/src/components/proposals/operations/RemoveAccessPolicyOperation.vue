<template>
  <div v-if="isListMode" class="d-flex flex-column ga-0 text-caption">
    <ProposalOperationListRow v-if="formValue.id">
      <template #name>{{ $t('terms.id') }}</template>
      <template #content>
        {{ formValue.id }}
      </template>
    </ProposalOperationListRow>
  </div>
  <VProgressCircular v-else-if="loading" />
  <AccessPolicyForm v-else :model-value="formValue" mode="view" />
</template>

<script setup lang="ts">
import { Ref, computed, onBeforeMount, ref } from 'vue';
import AccessPolicyForm from '~/components/access-policies/AccessPolicyForm.vue';
import logger from '~/core/logger.core';
import { AccessPolicy, Proposal, RemoveAccessPolicyOperation } from '~/generated/wallet/wallet.did';
import { useWalletStore } from '~/stores/wallet.store';
import ProposalOperationListRow from '../ProposalOperationListRow.vue';

const props = withDefaults(
  defineProps<{
    proposal: Proposal;
    operation: RemoveAccessPolicyOperation;
    mode?: 'list' | 'detail';
  }>(),
  {
    mode: 'list',
  },
);

const isListMode = computed(() => props.mode === 'list');
const formValue: Ref<Partial<AccessPolicy>> = ref({});
const loading = ref(false);
const wallet = useWalletStore();

const fetchDetails = async () => {
  try {
    if (loading.value || isListMode.value) {
      return;
    }

    loading.value = true;
    const currentEntry = await wallet.service.getAccessPolicy({
      id: props.operation.input.policy_id,
    });

    formValue.value = currentEntry.policy;
  } catch (e) {
    logger.error('Failed to fetch access policy details', e);
  } finally {
    loading.value = false;
  }
};

onBeforeMount(() => {
  const entry: Partial<AccessPolicy> = {};
  entry.id = props.operation.input.policy_id;

  formValue.value = entry;

  fetchDetails();
});
</script>
