<template>
  <VForm ref="form" @submit.prevent="revalidate">
    <VContainer>
      <VRow>
        <VCol cols="12" class="px-0">
          <VStepper
            v-model="step"
            hide-actions
            editable
            :mobile="app.isMobile"
            flat
            class="account-setup-wizard"
          >
            <VStepperHeader class="box-shadow-none">
              <VStepperItem
                :value="AccountSetupStep.Configuration"
                :error="hasConfigurationErrors"
                :title="$t('terms.configuration')"
              >
                <template #icon>{{ AccountSetupStep.Configuration }}</template>
              </VStepperItem>
              <VDivider />
              <VStepperItem :value="AccountSetupStep.AccessPolicy" :title="$t('terms.access')">
                <template #icon>{{ AccountSetupStep.AccessPolicy }}</template>
              </VStepperItem>
              <VDivider />
              <VStepperItem
                :value="AccountSetupStep.ApprovalPolicy"
                :title="$t('terms.approval_policies')"
              >
                <template #icon>{{ AccountSetupStep.ApprovalPolicy }}</template>
              </VStepperItem>
            </VStepperHeader>
            <VStepperWindow>
              <VStepperWindowItem :value="AccountSetupStep.Configuration">
                <AccountConfigurationSettings v-model="model.configuration" :mode="props.mode" />
              </VStepperWindowItem>
              <VStepperWindowItem :value="AccountSetupStep.AccessPolicy">
                <AccountAccessSettings v-model="model.access_policy" :mode="props.mode" />
              </VStepperWindowItem>
              <VStepperWindowItem :value="AccountSetupStep.ApprovalPolicy">
                <AccountApprovalPolicySettings v-model="model.approval_policy" :mode="props.mode" />
              </VStepperWindowItem>
            </VStepperWindow>
          </VStepper>
        </VCol>
        <VDivider />
        <VCol cols="12" class="d-flex">
          <VBtn :disabled="!canReturn" variant="text" @click="step--">
            {{ $t('terms.previous') }}
          </VBtn>
          <VSpacer />
          <VBtn v-if="canAdvance" variant="tonal" color="primary-variant" @click="step++">
            {{ $t('terms.next') }}
          </VBtn>
          <VBtn
            v-if="showSubmit"
            :disabled="!canSubmit"
            :loading="props.saving"
            color="primary"
            @click="submit"
          >
            {{ isCreationMode ? $t('terms.create') : $t('terms.save') }}
          </VBtn>
        </VCol>
      </VRow>
    </VContainer>
  </VForm>
</template>

<script lang="ts" setup>
import { computed, ref, watch } from 'vue';
import {
  VBtn,
  VCol,
  VContainer,
  VDivider,
  VForm,
  VRow,
  VSpacer,
  VStepper,
  VStepperHeader,
  VStepperItem,
  VStepperWindow,
  VStepperWindowItem,
} from 'vuetify/components';
import AccountAccessSettings, {
  AccountAccessPolicyModel,
} from '~/components/accounts/wizard/AccountAccessSettings.vue';
import AccountApprovalPolicySettings, {
  AccountApprovalPolicyModel,
} from '~/components/accounts/wizard/AccountApprovalPolicySettings.vue';
import AccountConfigurationSettings, {
  AccountConfigurationModel,
} from '~/components/accounts/wizard/AccountConfigurationSettings.vue';
import { useAppStore } from '~/stores/app.store';
import { VFormValidation } from '~/types/helper.types';

export interface AccountSetupWizardModel {
  configuration: Partial<AccountConfigurationModel>;
  access_policy: AccountAccessPolicyModel;
  approval_policy: AccountApprovalPolicyModel;
}

const props = withDefaults(
  defineProps<{
    step?: 1 | 2 | 3;
    saving?: boolean;
    mode?: 'view' | 'edit';
    modelValue: AccountSetupWizardModel;
  }>(),
  {
    step: 1,
    mode: 'view',
    saving: false,
  },
);

const emit = defineEmits<{
  (event: 'update:modelValue', payload: AccountSetupWizardModel): void;
  (event: 'valid', payload: boolean): void;
  (event: 'submit', payload: void): void;
}>();

enum AccountSetupStep {
  Configuration = 1,
  AccessPolicy = 2,
  ApprovalPolicy = 3,
}

const app = useAppStore();
const step = ref(props.step);
const form = ref<VFormValidation | null>(null);
const valid = ref(true);

const isViewMode = computed(() => props.mode === 'view');
const canAdvance = computed(() => step.value < AccountSetupStep.ApprovalPolicy);
const canReturn = computed(() => step.value > AccountSetupStep.Configuration && !props.saving);
const canSubmit = computed(() => valid.value);
const showSubmit = computed(
  () => step.value === AccountSetupStep.ApprovalPolicy && !isViewMode.value,
);
const isCreationMode = computed(() => !props.modelValue.configuration.id);
const fieldsWithErrors = ref<string[]>([]);
const hasConfigurationErrors = computed(() => fieldsWithErrors.value.includes('name'));

const model = computed({
  get: () => props.modelValue,
  set: value => emit('update:modelValue', value),
});

watch(
  () => valid.value,
  () => emit('valid', valid.value),
);

watch(
  () => model.value,
  _ => {
    valid.value = form.value?.isValid ?? false;
    fieldsWithErrors.value = form.value?.errors.map(error => error.id) ?? [];
  },
  { deep: true },
);

const revalidate = async (): Promise<boolean> => {
  const { valid: isValid, errors } = form.value
    ? await form.value.validate()
    : { valid: false, errors: [] };

  valid.value = isValid;

  fieldsWithErrors.value = errors.map(error => error.id);

  return isValid;
};

const submit = async () => {
  const isValid = await revalidate();

  if (isValid) {
    emit('submit');
  }
};
</script>

<style lang="scss">
.account-setup-wizard {
  & .v-stepper-window {
    margin-top: 0;
    margin-bottom: 0;
  }
}
</style>
