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
            class="canister-setup-wizard"
          >
            <VStepperHeader class="box-shadow-none">
              <VStepperItem
                :value="CanisterWizardSetupStep.Configuration"
                :error="hasConfigurationErrors"
                :title="$t('terms.configuration')"
              >
                <template #icon>{{ CanisterWizardSetupStep.Configuration }}</template>
              </VStepperItem>
              <VDivider />
              <VStepperItem :value="CanisterWizardSetupStep.Permission" :title="$t('terms.access')">
                <template #icon>{{ CanisterWizardSetupStep.Permission }}</template>
              </VStepperItem>
              <VDivider />
              <VStepperItem
                :value="CanisterWizardSetupStep.ApprovalPolicy"
                :title="$t('terms.approval_policies')"
              >
                <template #icon>{{ CanisterWizardSetupStep.ApprovalPolicy }}</template>
              </VStepperItem>
            </VStepperHeader>
            <VStepperWindow>
              <VStepperWindowItem :value="CanisterWizardSetupStep.Configuration">
                <CanisterConfigurationStep v-model="model.configuration" :mode="props.mode" />
              </VStepperWindowItem>
              <VStepperWindowItem :value="CanisterWizardSetupStep.Permission">
                <CanisterPermissionStep v-model="model.permission" :mode="props.mode" />
              </VStepperWindowItem>
              <VStepperWindowItem :value="CanisterWizardSetupStep.ApprovalPolicy">
                <ApprovalPolicyStep v-model="model.approvalPolicy" :mode="props.mode" />
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
import { useAppStore } from '~/stores/app.store';
import { VFormValidation } from '~/types/helper.types';
import ApprovalPolicyStep from './ApprovalPolicyStep.vue';
import CanisterConfigurationStep from './CanisterConfigurationStep.vue';
import CanisterPermissionStep from './CanisterPermissionStep.vue';
import { CanisterWizardModel, CanisterWizardSetupStep } from './wizard.types';

const props = withDefaults(
  defineProps<{
    step?: 1 | 2 | 3;
    saving?: boolean;
    mode?: 'view' | 'edit';
    modelValue: CanisterWizardModel;
  }>(),
  {
    step: 1,
    mode: 'view',
    saving: false,
  },
);

const emit = defineEmits<{
  (event: 'update:modelValue', payload: CanisterWizardModel): void;
  (event: 'valid', payload: boolean): void;
  (event: 'submit', payload: void): void;
}>();

const app = useAppStore();
const step = ref(props.step);
const form = ref<VFormValidation | null>(null);
const valid = ref(true);

const isViewMode = computed(() => props.mode === 'view');
const canAdvance = computed(() => step.value < CanisterWizardSetupStep.ApprovalPolicy);
const canReturn = computed(
  () => step.value > CanisterWizardSetupStep.Configuration && !props.saving,
);
const canSubmit = computed(() => valid.value);
const showSubmit = computed(
  () => step.value === CanisterWizardSetupStep.ApprovalPolicy && !isViewMode.value,
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
.canister-setup-wizard {
  & .v-stepper-window {
    margin-top: 0;
    margin-bottom: 0;
  }
}
</style>
