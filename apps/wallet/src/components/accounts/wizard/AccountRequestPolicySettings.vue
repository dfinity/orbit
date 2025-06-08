<template>
  <VRow>
    <VCol cols="12" class="px-0">
      <VCard flat>
        <VCardTitle data-test-id="update-approval-policy">
          <TextLabel
            :label="$t('app.account_dialog_request_policy_configuration')"
            :tooltip="$t('app.account_dialog_request_policy_configuration_hint')"
          />
        </VCardTitle>
        <VCardText>
          <DiffView
            :before-value="props.currentPolicies?.configurationRule"
            :after-value="model.configurationRule"
          >
            <template #default="{ value, diffMode }">
              <RuleBuilder
                :model-value="value"
                :specifier="{ EditAccount: { Any: null } }"
                :disabled="diffMode === 'before' ? true : isViewMode"
                @update:model-value="val => diffMode === 'after' && (model.configurationRule = val)"
                @remove="diffMode === 'after' && (model.configurationRule = undefined)"
              />
            </template>
          </DiffView>
        </VCardText>
      </VCard>
    </VCol>
    <VDivider />
    <VCol cols="12" class="px-0">
      <VCard flat>
        <VCardTitle data-test-id="transfer-approval-policy">
          <TextLabel
            :label="$t('app.account_dialog_request_policy_transfer')"
            :tooltip="$t('app.account_dialog_request_policy_transfer_hint')"
          />
        </VCardTitle>
        <VCardText>
          <DiffView
            :before-value="props.currentPolicies?.transferRule"
            :after-value="model.transferRule"
          >
            <template #default="{ value, diffMode }">
              <RuleBuilder
                :model-value="value"
                :specifier="{ Transfer: { Any: null } }"
                :disabled="diffMode === 'before' ? true : isViewMode"
                @update:model-value="val => diffMode === 'after' && (model.transferRule = val)"
                @remove="diffMode === 'after' && (model.transferRule = undefined)"
              />
            </template>
          </DiffView>
        </VCardText>
      </VCard>
    </VCol>
  </VRow>
</template>

<script lang="ts" setup>
import { computed } from 'vue';
import { VCard, VCardText, VCardTitle, VCol, VDivider, VRow } from 'vuetify/components';
import TextLabel from '~/components/ui/TextLabel.vue';
import RuleBuilder from '~/components/request-policies/rule/RuleBuilder.vue';
import { RequestPolicyRule } from '~/generated/station/station.did';
import DiffView from '~/components/requests/DiffView.vue';

export interface AccountRequestPolicyModel {
  configurationRule?: RequestPolicyRule;
  transferRule?: RequestPolicyRule;
}

const props = withDefaults(
  defineProps<{
    modelValue: AccountRequestPolicyModel;
    mode?: 'view' | 'edit';
    currentPolicies?: AccountRequestPolicyModel;
  }>(),
  {
    valid: true,
    mode: 'edit',
    currentPolicies: undefined,
  },
);

const emit = defineEmits<{
  (event: 'update:modelValue', payload: AccountRequestPolicyModel): void;
}>();

const isViewMode = computed(() => props.mode === 'view');
const model = computed({
  get: () => props.modelValue,
  set: value => emit('update:modelValue', value),
});
</script>
