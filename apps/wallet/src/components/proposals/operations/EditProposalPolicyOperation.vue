<template>
  <div v-if="isListMode" class="d-flex flex-column ga-0 text-caption">
    <ProposalOperationListRow v-if="formValue.id">
      <template #name>{{ $t('terms.id') }}</template>
      <template #content>
        {{ formValue.id }}
      </template>
    </ProposalOperationListRow>
    <ProposalOperationListRow v-if="proposalPolicyType">
      <template #name>{{ $t('terms.specifier') }}</template>
      <template #content>
        {{ proposalPolicyType }}
      </template>
    </ProposalOperationListRow>
  </div>
  <VProgressCircular v-else-if="loading" />
  <ProposalPolicyForm v-else :model-value="formValue" mode="view" />
</template>

<script setup lang="ts">
import { Ref, computed, onBeforeMount, ref } from 'vue';
import logger from '~/core/logger.core';
import {
  EditProposalPolicyOperation,
  Proposal,
  ProposalPolicy,
} from '~/generated/station/station.did';
import { useStationStore } from '~/stores/station.store';
import ProposalOperationListRow from '../ProposalOperationListRow.vue';
import ProposalPolicyForm from '~/components/proposal-policies/ProposalPolicyForm.vue';
import { useI18n } from 'vue-i18n';

const props = withDefaults(
  defineProps<{
    proposal: Proposal;
    operation: EditProposalPolicyOperation;
    mode?: 'list' | 'detail';
  }>(),
  {
    mode: 'list',
  },
);

const isListMode = computed(() => props.mode === 'list');
const formValue: Ref<Partial<ProposalPolicy>> = ref({});
const loading = ref(false);
const station = useStationStore();

const fetchDetails = async () => {
  try {
    if (loading.value || isListMode.value) {
      return;
    }

    loading.value = true;
    const currentEntry = await station.service.getProposalPolicy(props.operation.input.policy_id);
    if (formValue.value.criteria) {
      currentEntry.policy.criteria = formValue.value.criteria;
    }
    if (formValue.value.specifier) {
      currentEntry.policy.specifier = formValue.value.specifier;
    }

    formValue.value = currentEntry.policy;
  } catch (e) {
    logger.error('Failed to fetch proposal policy details', e);
  } finally {
    loading.value = false;
  }
};

const i18n = useI18n();
const proposalPolicyType = computed(() => {
  const keys = Object.keys(formValue.value.specifier ?? {});
  for (const specifier of keys) {
    return i18n.t(`proposal_policies.specifier.${specifier.toLowerCase()}`);
  }

  return undefined;
});

onBeforeMount(() => {
  const policy: Partial<ProposalPolicy> = {};
  policy.id = props.operation.input.policy_id;
  if (props.operation.input.criteria?.[0]) {
    policy.criteria = props.operation.input.criteria[0];
  }
  if (props.operation.input.specifier?.[0]) {
    policy.specifier = props.operation.input.specifier[0];
  }

  formValue.value = policy;

  fetchDetails();
});
</script>
