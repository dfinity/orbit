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
  <AccessPolicyForm v-else :model-value="formValue" mode="view" />
</template>

<script setup lang="ts">
import { Ref, computed, onBeforeMount, ref } from 'vue';
import AccessPolicyForm from '~/components/access-policies/AccessPolicyForm.vue';
import logger from '~/core/logger.core';
import { AccessPolicy, EditAccessPolicyOperation, Proposal } from '~/generated/wallet/wallet.did';
import { useWalletStore } from '~/stores/wallet.store';
import ProposalOperationListColumn from '../ProposalOperationListColumn.vue';

const props = withDefaults(
  defineProps<{
    proposal: Proposal;
    operation: EditAccessPolicyOperation;
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

    if (formValue.value.user) {
      currentEntry.policy.user = formValue.value.user;
    }

    if (formValue.value.resource) {
      currentEntry.policy.resource = formValue.value.resource;
    }

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
  if (props.operation.input.resource?.[0]) {
    entry.resource = props.operation.input.resource[0];
  }
  if (props.operation.input.user?.[0]) {
    entry.user = props.operation.input.user[0];
  }

  formValue.value = entry;

  fetchDetails();
});
</script>
