<template>
  <VRow>
    <VCol v-for="(_, idx) in model.change" :key="idx" cols="12" class="px-0">
      <VCard flat>
        <VCardTitle data-test-id="update-approval-policy">
          <TextLabel
            :label="$t('app.account_dialog_request_policy_configuration')"
            :tooltip="$t('app.account_dialog_request_policy_configuration_hint')"
          />
        </VCardTitle>
        <VCardText>
          <RuleBuilder
            v-model="model.change[idx].rule"
            :specifier="{ EditAccount: { Any: null } }"
            :disabled="isViewMode"
            @remove="delete model.change[idx]"
          />
        </VCardText>
      </VCard>
    </VCol>
  </VRow>
</template>

<script lang="ts" setup>
import { computed } from 'vue';
import { VCard, VCardText, VCardTitle, VCol, VRow } from 'vuetify/components';
import { CanisterApprovalPolicyModel } from '~/components/external-canisters/wizard/wizard.types';
import RuleBuilder from '~/components/request-policies/rule/RuleBuilder.vue';
import TextLabel from '~/components/ui/TextLabel.vue';

const props = withDefaults(
  defineProps<{
    modelValue: CanisterApprovalPolicyModel;
    mode?: 'view' | 'edit';
  }>(),
  {
    valid: true,
    mode: 'edit',
  },
);

const emit = defineEmits<{
  (event: 'update:modelValue', payload: CanisterApprovalPolicyModel): void;
}>();

const isViewMode = computed(() => props.mode === 'view');
const model = computed({
  get: () => props.modelValue,
  set: value => emit('update:modelValue', value),
});
</script>
