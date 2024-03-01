<template>
  <div v-if="isListMode" class="d-flex flex-column ga-0 text-caption">
    <ProposalOperationListRow>
      <template #name>{{ $t('access_policies.resource_title') }}</template>
      <template #content>
        {{ accessPolicyResource }}
      </template>
    </ProposalOperationListRow>
  </div>
  <AccessPolicyForm v-else :model-value="formValue" mode="view" />
</template>

<script setup lang="ts">
import { Ref, computed, onBeforeMount, ref } from 'vue';
import { useI18n } from 'vue-i18n';
import AccessPolicyForm from '~/components/access-policies/AccessPolicyForm.vue';
import ProposalOperationListRow from '~/components/proposals/ProposalOperationListRow.vue';
import { AccessPolicy, AddAccessPolicyOperation, Proposal } from '~/generated/wallet/wallet.did';

const props = withDefaults(
  defineProps<{
    proposal: Proposal;
    operation: AddAccessPolicyOperation;
    mode?: 'list' | 'detail';
  }>(),
  {
    mode: 'list',
  },
);

const i18n = useI18n();
const isListMode = computed(() => props.mode === 'list');
const formValue: Ref<Partial<AccessPolicy>> = ref({});

const accessPolicyResource = computed(() => {
  const keys = Object.keys(props.operation.input.resource);
  for (const specifier of keys) {
    return i18n.t(`access_policies.resources.${specifier.toLowerCase()}`);
  }

  return '-';
});

onBeforeMount(() => {
  const policy: Partial<AccessPolicy> = {};
  policy.resource = props.operation.input.resource;
  policy.user = props.operation.input.user;

  formValue.value = policy;
});
</script>
