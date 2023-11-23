<template>
  <VCard density="compact" variant="elevated">
    <VCardText class="pb-0">
      <VSelect
        v-model="selectedPolicy"
        :items="approvalItems"
        :readonly="isViewMode"
        density="compact"
        :label="$t('wallets.policy')"
      />
      <VSlider
        v-if="selectedPolicy === PolicyType.FixedApprovalThreshold"
        v-model="policyInput.number"
        color="primary-variant"
        class="mt-4"
        thumb-label="always"
        :step="1"
        :readonly="isViewMode"
        :min="1"
        :max="props.owners ?? 1"
        :hint="$t('wallets.policy_fixed_approval_threshold_desc')"
        :persistent-hint="true"
      />
      <VSlider
        v-else-if="selectedPolicy === PolicyType.VariableApprovalThreshold"
        v-model="policyInput.number"
        color="primary-variant"
        class="mt-4"
        :readonly="isViewMode"
        thumb-label="always"
        :step="1"
        :min="1"
        :max="100"
        :hint="$t('wallets.policy_variable_approval_threshold_desc')"
        :persistent-hint="true"
      />
      <div v-else>{{ $t('wallets.policy_config_unavailable') }}</div>
    </VCardText>
    <VCardActions v-if="!isViewMode">
      <VSpacer />
      <VBtn color="error" variant="text" @click="removePolicy">
        {{ $t('terms.remove') }}
      </VBtn>
    </VCardActions>
  </VCard>
</template>

<script lang="ts" setup>
import { computed, ref, watch } from 'vue';
import { Policy } from '~/generated/wallet/wallet.did';
import { i18n } from '~/ui/modules';
import { useWalletStore } from '~/ui/stores';
import { PolicyType } from '~/types';

const props = defineProps<{
  modelValue?: Policy | null;
  owners?: number;
  mode?: 'edit' | 'view';
}>();

const emit = defineEmits<{
  (event: 'update:modelValue', payload?: Policy | null): void;
  (event: 'removed'): void;
}>();

const isViewMode = computed(() => props.mode === 'view');

const modelValue = computed({
  get: () => props.modelValue,
  set: value => emit('update:modelValue', value),
});

const walletStore = useWalletStore();
const approvalItems = walletStore.policyTypes.map(type => ({
  title: i18n.global.t(`wallets.policies.${type}`),
  value: type,
}));

const removePolicy = (): void => emit('removed');
const selectedPolicy = ref<PolicyType | null>(null);
const policyInput = ref<{
  number?: number;
}>({
  number: undefined,
});

if (modelValue.value?.approval_threshold) {
  const threshold = modelValue.value.approval_threshold;
  if ('VariableThreshold' in threshold) {
    selectedPolicy.value = PolicyType.VariableApprovalThreshold;
    policyInput.value.number = threshold.VariableThreshold;
  } else if ('FixedThreshold' in threshold) {
    selectedPolicy.value = PolicyType.FixedApprovalThreshold;
    policyInput.value.number = threshold.FixedThreshold;
  }
}

const clearPolicy = (): void => {
  policyInput.value.number = undefined;
  modelValue.value = null;
};

watch(selectedPolicy, () => {
  clearPolicy();

  switch (selectedPolicy.value) {
    case PolicyType.FixedApprovalThreshold:
      policyInput.value.number = 1;
      break;
    case PolicyType.VariableApprovalThreshold:
      policyInput.value.number = 100;
      break;
  }
});
watch(policyInput, () => reevaluatePolicy(), {
  deep: true,
});

const reevaluatePolicy = (): void => {
  if (!selectedPolicy.value) {
    clearPolicy();
    return;
  }

  switch (selectedPolicy.value) {
    case PolicyType.FixedApprovalThreshold:
      reevaluateFixedThresholdPolicy();
      break;
    case PolicyType.VariableApprovalThreshold:
      reevaluateVariableThresholdPolicy();
      break;
  }
};

const reevaluateFixedThresholdPolicy = (): void => {
  const nrOfOwners = props.owners ?? 1;
  if (!policyInput.value.number || policyInput.value.number > nrOfOwners) {
    return;
  }

  modelValue.value = {
    approval_threshold: {
      FixedThreshold: policyInput.value.number,
    },
  };
};

const reevaluateVariableThresholdPolicy = (): void => {
  if (!policyInput.value.number || policyInput.value.number > 100 || policyInput.value.number < 1) {
    return;
  }

  modelValue.value = {
    approval_threshold: {
      VariableThreshold: Math.round(policyInput.value.number),
    },
  };
};
</script>
