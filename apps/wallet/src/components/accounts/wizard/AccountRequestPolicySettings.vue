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
          <RuleBuilder
            v-model="model.configurationRule"
            :specifier="{ EditAccount: { Any: null } }"
            :disabled="isViewMode"
            @remove="model.configurationRule = undefined"
          />
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
          <RuleBuilder
            v-model="model.transferRule"
            :specifier="{ Transfer: { Any: null } }"
            :disabled="isViewMode"
            @remove="model.transferRule = undefined"
          />
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

export interface AccountRequestPolicyModel {
  configurationRule?: RequestPolicyRule;
  transferRule?: RequestPolicyRule;
}

const props = withDefaults(
  defineProps<{
    modelValue: AccountRequestPolicyModel;
    mode?: 'view' | 'edit';
  }>(),
  {
    valid: true,
    mode: 'edit',
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
