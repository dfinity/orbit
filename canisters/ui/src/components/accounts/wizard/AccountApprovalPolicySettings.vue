<template>
  <VRow>
    <VCol cols="12" class="px-0">
      <VCard flat>
        <VCardTitle>
          <TextLabel
            :label="$t('app.account_dialog_approval_policy_configuration')"
            :tooltip="$t('app.account_dialog_approval_policy_configuration_hint')"
          />
        </VCardTitle>
        <VCardText>
          <CriteriaBuilder
            v-model="model.configurationCriteria"
            :specifier="{ EditAccount: { Any: null } }"
            :disabled="isViewMode"
            @remove="model.configurationCriteria = undefined"
          />
        </VCardText>
      </VCard>
    </VCol>
    <VDivider />
    <VCol cols="12" class="px-0">
      <VCard flat>
        <VCardTitle>
          <TextLabel
            :label="$t('app.account_dialog_approval_policy_transfer')"
            :tooltip="$t('app.account_dialog_approval_policy_transfer_hint')"
          />
        </VCardTitle>
        <VCardText>
          <CriteriaBuilder
            v-model="model.transferCriteria"
            :specifier="{ Transfer: { account: { Any: null } } }"
            :disabled="isViewMode"
            @remove="model.transferCriteria = undefined"
          />
        </VCardText>
      </VCard>
    </VCol>
  </VRow>
</template>

<script lang="ts" setup>
import { computed } from 'vue';
import { VCard, VCardText, VCardTitle, VCol, VDivider, VRow } from 'vuetify/components';
import TextLabel from '~/components/TextLabel.vue';
import CriteriaBuilder from '~/components/proposal-policies/criteria/CriteriaBuilder.vue';
import { ProposalPolicyCriteria } from '~/generated/wallet/wallet.did';

export interface AccountApprovalPolicyModel {
  configurationCriteria?: ProposalPolicyCriteria;
  transferCriteria?: ProposalPolicyCriteria;
}

const props = withDefaults(
  defineProps<{
    modelValue: AccountApprovalPolicyModel;
    mode: 'view' | 'edit';
  }>(),
  {
    valid: true,
    mode: 'edit',
  },
);

const emit = defineEmits<{
  (event: 'update:modelValue', payload: AccountApprovalPolicyModel): void;
}>();

const isViewMode = computed(() => props.mode === 'view');
const model = computed({
  get: () => props.modelValue,
  set: value => emit('update:modelValue', value),
});
</script>
