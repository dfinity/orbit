<template>
  <div v-if="isListMode" class="d-flex flex-column ga-0 text-caption">
    <ProposalOperationListRow>
      <template #name>{{ $t('terms.specifier') }}</template>
      <template #content>
        {{ proposalPolicyType }}
      </template>
    </ProposalOperationListRow>
  </div>
  <ProposalPolicyForm v-else :model-value="formValue" mode="view" />
</template>

<script setup lang="ts">
import { Ref, computed, onBeforeMount, ref } from 'vue';
import { useI18n } from 'vue-i18n';
import ProposalPolicyForm from '~/components/proposal-policies/ProposalPolicyForm.vue';
import ProposalOperationListRow from '~/components/proposals/ProposalOperationListRow.vue';
import {
  AddProposalPolicyOperation,
  Proposal,
  ProposalPolicy,
} from '~/generated/station/station.did';

const props = withDefaults(
  defineProps<{
    proposal: Proposal;
    operation: AddProposalPolicyOperation;
    mode?: 'list' | 'detail';
  }>(),
  {
    mode: 'list',
  },
);

const i18n = useI18n();
const isListMode = computed(() => props.mode === 'list');
const formValue: Ref<Partial<ProposalPolicy>> = ref({});

const proposalPolicyType = computed(() => {
  const keys = Object.keys(props.operation.input.specifier);
  for (const specifier of keys) {
    return i18n.t(`proposal_policies.specifier.${specifier.toLowerCase()}`);
  }

  return '-';
});

onBeforeMount(() => {
  const policy: Partial<ProposalPolicy> = {};
  policy.specifier = props.operation.input.specifier;
  policy.criteria = props.operation.input.criteria;

  formValue.value = policy;
});
</script>
