<template>
  <VForm v-bind="$attrs" ref="form" @submit.prevent="submit">
    <slot name="errors" :errors="additionalFieldErrors">
      <FormErrorsContainer v-bind="{ errors: additionalFieldErrors }" />
    </slot>
    <VContainer>
      <VRow>
        <VCol v-if="!props.hide.canisterId" cols="12" class="pb-0 px-6">
          <CanisterIdField
            v-model="model.canisterId"
            :readonly="readonly"
            required
            name="canister_id"
          />
        </VCol>
        <VCol cols="12" class="pb-0 px-6">
          <VCombobox
            v-if="props.allowAnyMethod"
            v-model="model.methodName"
            density="comfortable"
            :prepend-icon="mdiCodeBraces"
            :readonly="readonly"
            :items="availableExecutionMethods"
            :label="$t('external_canisters.perform_call.method_name')"
            :hint="$t('external_canisters.perform_call.method_name_hint')"
            :rules="[requiredRule]"
            name="method_name"
            @keydown.enter.stop.prevent=""
          />
          <VSelect
            v-else
            v-model="model.methodName"
            density="comfortable"
            :prepend-icon="mdiCodeBraces"
            :readonly="readonly"
            :items="availableExecutionMethods"
            :label="$t('external_canisters.perform_call.method_name')"
            :hint="$t('external_canisters.perform_call.method_name_hint')"
            :rules="[requiredRule]"
            name="method_name"
            @keydown.enter.stop.prevent=""
          />
        </VCol>
        <VCol cols="12" class="pb-0 px-6">
          <CanisterArgumentField
            v-model="model.arg"
            name="argument"
            :readonly="readonly"
            :candid="
              props.candidIdl ? { idl: props.candidIdl, method: model.methodName } : undefined
            "
          />
        </VCol>
        <VCol v-if="hasConfiguredValidationMethods" cols="12" class="pb-0 px-6">
          <VSelect
            v-model="model.validationTarget"
            :prepend-icon="mdiCodeBraces"
            :label="$t('external_canisters.perform_call.validation_method')"
            :hint="$t('external_canisters.perform_call.validation_method_hint')"
            :items="validationMethods"
            item-value="value"
            item-title="text"
            :readonly="readonly"
            density="comfortable"
            :rules="[requiredRule]"
          />
        </VCol>

        <VExpansionPanels>
          <VExpansionPanel v-model="openAdvancedPanel" class="mb-2" elevation="0">
            <template #title>
              <div class="flex-grow-1">
                <p class="text-subtitle-1">
                  {{ $t('terms.settings') }}
                </p>
                <VDivider />
              </div>
            </template>
            <VExpansionPanelText>
              <CyclesInput
                v-model="model.cycles"
                :label="$t('external_canisters.perform_call.attach_cycles')"
                name="cycles"
                :readonly="readonly"
                :unit="CyclesUnit.Trillion"
                :hint="$t('external_canisters.perform_call.attach_cycles_hint')"
                @keydown.enter.stop.prevent=""
              />

              <VTextarea
                v-if="!readonly"
                v-model="model.requestComment"
                name="comment"
                class="mt-2"
                :prepend-icon="mdiComment"
                :label="$t(`requests.comment_optional`)"
                variant="filled"
                density="comfortable"
                auto-grow
                rows="2"
                hide-details
              />
            </VExpansionPanelText>
          </VExpansionPanel>
        </VExpansionPanels>
      </VRow>
    </VContainer>
  </VForm>
  <slot
    v-if="!readonly"
    name="actions"
    :valid="valid"
    :submitting="submitting"
    :edited="edited"
    :submit="submit"
  >
    <FormActions v-bind="{ valid, submitting, edited, submit }" />
  </slot>
</template>
<script lang="ts" setup>
import { mdiCodeBraces, mdiComment } from '@mdi/js';
import { computed, watch } from 'vue';
import { VCol, VContainer, VForm, VRow } from 'vuetify/components';
import FormActions from '~/components/ui/FormActions.vue';
import FormErrorsContainer from '~/components/ui/FormErrorsContainer.vue';
import { useForm } from '~/composables/forms.composable';
import {
  useOnFailedOperation,
  useOnSuccessfulOperation,
} from '~/composables/notifications.composable';
import { useStationStore } from '~/stores/station.store';
import { requiredRule } from '~/utils/form.utils';
import { assertAndReturn, variantIs } from '~/utils/helper.utils';
import CanisterIdField from '../inputs/CanisterIdField.vue';
import { CanisterCallModel, CanisterAllowedMethod } from './external-canisters.types';
import CanisterArgumentField from '../inputs/CanisterArgumentField.vue';
import { CyclesUnit } from '~/types/app.types';
import CyclesInput from '../inputs/CyclesInput.vue';
import { ref } from 'vue';
import { SelectItem } from '~/types/helper.types';
import { ValidationMethodResourceTarget } from '~/generated/station/station.did';
import { useI18n } from 'vue-i18n';
import { isApiError } from '~/utils/app.utils';
import { getServiceMethods } from '~/utils/didc.utils';
import logger from '~/core/logger.core';

const props = withDefaults(
  defineProps<{
    modelValue: CanisterCallModel;
    allowedMethods?: CanisterAllowedMethod[];
    allowAnyMethod?: boolean;
    readonly?: boolean;
    candidIdl?: string;
    hide?: {
      canisterId?: boolean;
    };
  }>(),
  {
    readonly: false,
    allowedMethods: () => [],
    allowAnyMethod: false,
    candidIdl: undefined,
    hide: () => ({
      canisterId: false,
    }),
  },
);

const emit = defineEmits<{
  (event: 'update:modelValue', payload: CanisterCallModel): void;
  (event: 'valid', payload: boolean): void;
  (event: 'edited', payload: boolean): void;
  (event: 'submitting', payload: boolean): void;
  (event: 'submitted'): void;
}>();

const model = computed({
  get: () => props.modelValue,
  set: value => emit('update:modelValue', value),
});

const i18n = useI18n();
const station = useStationStore();
const openAdvancedPanel = ref(false);

const { submit, edited, additionalFieldErrors, fieldsWithErrors, submitting, valid, submitted } =
  useForm({
    model,
    submit: async (updatedModel: CanisterCallModel): Promise<void> => {
      try {
        const methodName = assertAndReturn(updatedModel.methodName, 'Method name');
        const canisterId = assertAndReturn(updatedModel.canisterId, 'Canister ID');
        const validationMethod =
          updatedModel.validationTarget &&
          variantIs(updatedModel.validationTarget, 'ValidationMethod')
            ? updatedModel.validationTarget.ValidationMethod
            : undefined;

        const request = await station.service.callExternalCanister(
          canisterId,
          {
            method: methodName,
            arg: model.value?.arg,
            attachCycles: model.value?.cycles,
            validationMethod: validationMethod,
          },
          {
            comment: model.value.requestComment?.trim().length
              ? model.value.requestComment.trim()
              : undefined,
          },
        );

        useOnSuccessfulOperation(request);
      } catch (error) {
        useOnFailedOperation();

        const errorMessages = [];
        if (isApiError(error) && error.code === 'VALIDATION_ERROR' && error.details?.[0]) {
          const details = error.details[0];
          for (const [_, errorMessage] of details) {
            if (errorMessage && errorMessage.length) {
              errorMessages.push(errorMessage);
            }
          }
        }

        if (!errorMessages.length) {
          errorMessages.push(i18n.t('external_canisters.perform_call.call_submit_failed'));
        }

        fieldsWithErrors.value = [
          { field: i18n.t('terms.error'), errorMessages, isCustomValidation: true },
        ];

        throw error;
      }
    },
  });

watch(valid, value => emit('valid', value));
watch(edited, value => emit('edited', value));
watch(submitting, value => emit('submitting', value));

watch(
  submitted,
  (value, _) => {
    if (value) {
      emit('submitted');
    }
  },
  { immediate: true },
);

const availableExecutionMethods = computed<string[]>(() => {
  const allowAnyMethod =
    props.allowAnyMethod || props.allowedMethods.some(method => method.methodName === '*');
  const configuredMethods = props.allowedMethods
    .map(method => method.methodName)
    .filter(methodName => !!methodName && methodName.trim().length && methodName !== '*');

  if (allowAnyMethod && props.candidIdl) {
    try {
      configuredMethods.push(...getServiceMethods(props.candidIdl));
    } catch (e) {
      logger.warn('Failed to get service methods from candid idl', e);
    }
  }

  return Array.from(new Set(configuredMethods)).sort();
});

const validationMethods = computed<SelectItem<ValidationMethodResourceTarget>[]>(() => {
  const methodName = model.value.methodName?.trim();

  if (!methodName) {
    return [];
  }

  const matchedValidationMethods = props.allowedMethods.filter(
    method => method.methodName === methodName,
  );

  return matchedValidationMethods.map(method => ({
    value: method.validationTarget,
    text: variantIs(method.validationTarget, 'ValidationMethod')
      ? method.validationTarget.ValidationMethod.canister_id.toText() ===
        model.value.canisterId?.toText()
        ? method.validationTarget.ValidationMethod.method_name
        : i18n.t('external_canisters.perform_call.validation_method_item_remote', {
            method: method.validationTarget.ValidationMethod.method_name,
            canister: method.validationTarget.ValidationMethod.canister_id.toText(),
          })
      : i18n.t('external_canisters.perform_call.validation_method_item_none'),
  }));
});

const hasConfiguredValidationMethods = computed(() => {
  if (validationMethods.value.length > 1) {
    return true;
  }

  if (validationMethods.value.length === 1) {
    return JSON.stringify(validationMethods.value[0].value) !== JSON.stringify({ No: null });
  }

  return false;
});

watch(
  () => model.value.methodName,
  (current, previous) => {
    // Resets the validation target when the method name changes
    if (current !== previous) {
      model.value.validationTarget = undefined;
    }
  },
);

watch(
  validationMethods,
  updatedMethods => {
    if (updatedMethods.length && model.value.validationTarget === undefined) {
      model.value.validationTarget = updatedMethods[0].value;
    }
  },
  { immediate: true },
);
</script>
