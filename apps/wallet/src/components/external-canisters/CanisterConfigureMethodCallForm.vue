<template>
  <VForm v-bind="$attrs" ref="form" @submit.prevent="submit">
    <slot name="errors" :errors="additionalFieldErrors">
      <FormErrorsContainer v-bind="{ errors: additionalFieldErrors }" />
    </slot>
    <VContainer>
      <VRow>
        <VCol cols="12" class="pb-0 px-6">
          <VTextField
            v-model="model.methodName"
            density="comfortable"
            :prepend-inner-icon="mdiCodeBraces"
            :readonly="readonly"
            :label="$t('external_canisters.call_configuration.method_name')"
            :hint="$t('external_canisters.call_configuration.method_name_hint')"
            :rules="[requiredRule]"
            name="method_name"
          />
        </VCol>
        <VCol cols="12" class="pt-0 px-6">
          <p class="text-subtitle-1 mb-2">
            <TextLabel
              :label="$t('external_canisters.call_configuration.method_call_permission')"
              :tooltip="$t('external_canisters.call_configuration.method_call_permission_hint')"
            />
          </p>
          <VDivider class="mb-2" />
          <AllowInput v-model="model.permission" :mode="readonly ? 'view' : 'edit'" />
        </VCol>
        <VCol cols="12" class="px-6">
          <p class="text-subtitle-1 mb-2">
            <TextLabel
              :label="$t('external_canisters.call_configuration.method_call_approval_policy')"
              :tooltip="
                $t('external_canisters.call_configuration.method_call_approval_policy_hint')
              "
            />
          </p>
          <VDivider class="mb-2" />
          <div
            v-for="(_, policyIdx) in model.requestPolicies"
            :key="policyIdx"
            class="d-flex flex-column ga-4"
          >
            <div class="d-flex flex-row">
              <div
                v-if="!readonly && model.requestPolicies.length > 1 && policyIdx !== 0"
                class="d-flex align-center"
              >
                <VBtn
                  :icon="mdiMinus"
                  variant="flat"
                  density="comfortable"
                  class="mr-2"
                  size="x-small"
                  @click.stop="deleteApprovalPolicyByIndex(policyIdx)"
                />
              </div>
              <div class="flex-grow-1 justify-center d-flex flex-column">
                <RuleBuilder
                  v-model="model.requestPolicies[policyIdx].rule"
                  :specifier="{
                    // The specifier here is only added to provide some context to the RuleBuilder, but
                    // the actual value is not used in the form submission
                    CallExternalCanister: {
                      execution_method: { Any: null },
                      validation_method: { No: null },
                    },
                  }"
                  :disabled="readonly"
                  @remove="model.requestPolicies[policyIdx].rule = undefined"
                />
              </div>
            </div>
            <VDivider v-if="policyIdx < model.requestPolicies.length - 1" />
          </div>
        </VCol>

        <VExpansionPanels>
          <VExpansionPanel v-model="openCustomValidationConfigPanel" class="mb-2" elevation="0">
            <template #title>
              <div class="flex-grow-1">
                <p class="text-subtitle-1">
                  {{ $t('external_canisters.call_configuration.advanced_validation') }}
                </p>
                <VDivider />
              </div>
            </template>
            <VExpansionPanelText>
              <p class="text-body-2 text-medium-emphasis">
                {{ $t('external_canisters.call_configuration.advanced_validation_hint') }}
              </p>
              <div v-if="showAdvancedValidation" class="d-flex flex-column ga-1 mt-3">
                <VTextField
                  v-model="model.validationMethodName"
                  density="comfortable"
                  :prepend-inner-icon="mdiCodeBraces"
                  :label="$t('external_canisters.call_configuration.validation_method_name')"
                  :hint="$t('external_canisters.call_configuration.validation_method_name_hint')"
                  :rules="[requiredRule]"
                  name="validation_method_name"
                />
                <CanisterIdField
                  v-model="model.validationCanisterId"
                  prepend-inner-icon
                  :readonly="readonly"
                  :label="$t('external_canisters.call_configuration.validation_canister_id')"
                  :hint="$t('external_canisters.call_configuration.validation_canister_id_hint')"
                  required
                  name="validation_canister_id"
                />
              </div>
              <template v-if="!readonly">
                <VBtn
                  v-if="!showAdvancedValidation"
                  size="small"
                  block
                  class="mt-2"
                  variant="tonal"
                  :prepend-icon="mdiPlus"
                  @click.stop="addValidationMethod"
                >
                  {{ $t('external_canisters.call_configuration.add_advanced_validation') }}
                </VBtn>
                <VBtn
                  v-else
                  size="small"
                  block
                  class="mt-2"
                  variant="tonal"
                  :prepend-icon="mdiTrashCan"
                  @click.stop="deleteValidatioNMethod"
                >
                  {{ $t('external_canisters.call_configuration.remove_advanced_validation') }}
                </VBtn>
              </template>
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
import { Principal } from '@dfinity/principal';
import { mdiCodeBraces, mdiMinus, mdiPlus, mdiTrashCan } from '@mdi/js';
import { computed, ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import { VCol, VContainer, VExpansionPanelText, VForm, VRow, VTextField } from 'vuetify/components';
import RuleBuilder from '~/components/request-policies/rule/RuleBuilder.vue';
import FormActions from '~/components/ui/FormActions.vue';
import FormErrorsContainer from '~/components/ui/FormErrorsContainer.vue';
import TextLabel from '~/components/ui/TextLabel.vue';
import { FieldWithError, useForm } from '~/composables/forms.composable';
import {
  useOnFailedOperation,
  useOnSuccessfulOperation,
} from '~/composables/notifications.composable';
import {
  ExternalCanisterChangeCallPermissionsInput,
  ExternalCanisterChangeCallRequestPoliciesInput,
  ValidationMethodResourceTarget,
} from '~/generated/station/station.did';
import { mapMethodCallConfigurationToKey } from '~/mappers/external-canister.mapper';
import { useStationStore } from '~/stores/station.store';
import { requiredRule } from '~/utils/form.utils';
import { assertAndReturn } from '~/utils/helper.utils';
import AllowInput from '../inputs/AllowInput.vue';
import CanisterIdField from '../inputs/CanisterIdField.vue';
import { CanisterMethodCallConfigurationModel } from './external-canisters.types';

const props = withDefaults(
  defineProps<{
    modelValue: CanisterMethodCallConfigurationModel;
    readonly?: boolean;
  }>(),
  {
    readonly: false,
  },
);

const emit = defineEmits<{
  (event: 'update:modelValue', payload: CanisterMethodCallConfigurationModel): void;
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
const openCustomValidationConfigPanel = ref(false);
const showAdvancedValidation = ref(false);

const createValidationTarget = (
  validationCanisterId?: Principal,
  validationMethodName?: string,
): ValidationMethodResourceTarget =>
  validationCanisterId && validationMethodName
    ? {
        ValidationMethod: {
          canister_id: validationCanisterId,
          method_name: validationMethodName,
        },
      }
    : { No: null };

const { submit, edited, initialModel, additionalFieldErrors, submitting, valid, submitted } =
  useForm({
    model,
    isValid(model: CanisterMethodCallConfigurationModel) {
      const errors: FieldWithError[] = [];
      if (model.methodName?.length) {
        // check if there are any duplicated method names
        const alreadyConfiguredMethods = Object.values(model.alreadyConfiguredMethods)
          .map(config =>
            mapMethodCallConfigurationToKey({
              executionMethod: config.methodName,
              validationMethod: config.validationTarget,
            }),
          )
          .filter(
            configuredMethod =>
              configuredMethod !==
              mapMethodCallConfigurationToKey({
                executionMethod: initialModel.value.methodName ?? '',
                validationMethod: !initialModel.value.validationMethodName
                  ? { No: null }
                  : {
                      ValidationMethod: {
                        canister_id:
                          initialModel.value.validationCanisterId ?? initialModel.value.canisterId,
                        method_name: initialModel.value.validationMethodName ?? '',
                      },
                    },
              }),
          );

        if (
          alreadyConfiguredMethods.includes(
            mapMethodCallConfigurationToKey({
              executionMethod: model.methodName,
              validationMethod: !model.validationMethodName
                ? { No: null }
                : {
                    ValidationMethod: {
                      canister_id: model.validationCanisterId ?? model.canisterId,
                      method_name: model.validationMethodName ?? '',
                    },
                  },
            }),
          )
        ) {
          errors.push({
            field: i18n.t(
              'external_canisters.call_configuration.duplicated_configuration_error_type',
            ),
            errorMessages: [
              i18n.t('external_canisters.call_configuration.duplicated_method_call_configuration'),
            ],
          });
        }
      }

      return errors;
    },
    submit: async (updatedModel: CanisterMethodCallConfigurationModel): Promise<void> => {
      try {
        const methodName = assertAndReturn(updatedModel.methodName, 'Method name is required');
        const validationTarget = createValidationTarget(
          updatedModel.validationCanisterId,
          updatedModel.validationMethodName,
        );

        const previousMethodName = initialModel.value.methodName ?? '';
        const previousValidationTarget = createValidationTarget(
          initialModel.value.validationCanisterId,
          initialModel.value.validationMethodName,
        );

        const isPreviousEntryRemoved =
          previousMethodName?.length &&
          (previousMethodName !== methodName ||
            JSON.stringify(previousValidationTarget) !== JSON.stringify(validationTarget));

        const request = await station.service.editExternalCanisterSettings(
          updatedModel.canisterId,
          {
            // only call permissions and request policies are updated in this form, all other fields must be ignored
            permissions: [
              {
                calls: [
                  {
                    OverrideSpecifiedByExecutionValidationMethodPairs: [
                      {
                        method_configuration: {
                          execution_method: methodName,
                          validation_method: validationTarget,
                        },
                        allow: [updatedModel.permission],
                      },
                      ...(isPreviousEntryRemoved
                        ? ([
                            {
                              method_configuration: {
                                execution_method: previousMethodName,
                                validation_method: previousValidationTarget,
                              },
                              allow: [], // request for the removal of the previous entry
                            },
                          ] as Extract<
                            ExternalCanisterChangeCallPermissionsInput,
                            { OverrideSpecifiedByExecutionValidationMethodPairs: unknown }
                          >['OverrideSpecifiedByExecutionValidationMethodPairs'])
                        : []),
                    ],
                  },
                ],
                change: [],
                read: [],
              },
            ],
            request_policies: [
              {
                calls: [
                  {
                    OverrideSpecifiedByExecutionValidationMethodPairs: [
                      {
                        method_configuration: {
                          execution_method: methodName,
                          validation_method: validationTarget,
                        },
                        policies: updatedModel.requestPolicies
                          .filter(policy => !!policy.rule)
                          .map(policy => ({
                            // if we are reusing the same policy from the previous entry, we must remove the policy_id
                            policy_id: isPreviousEntryRemoved ? [] : (policy.policy_id ?? []),
                            rule: assertAndReturn(policy.rule, 'Policy rule must be defined'),
                          })),
                      },
                      ...(isPreviousEntryRemoved
                        ? ([
                            {
                              method_configuration: {
                                execution_method: previousMethodName,
                                validation_method: previousValidationTarget,
                              },
                              policies: [], // request for the removal of the previous entries
                            },
                          ] as Extract<
                            ExternalCanisterChangeCallRequestPoliciesInput,
                            { OverrideSpecifiedByExecutionValidationMethodPairs: unknown }
                          >['OverrideSpecifiedByExecutionValidationMethodPairs'])
                        : []),
                    ],
                  },
                ],
                change: [],
              },
            ],
            // ignore all other fields in the model as they are not updated in this form
            name: [],
            description: [],
            labels: [],
            state: [],
          },
        );

        useOnSuccessfulOperation(request);
      } catch (error) {
        useOnFailedOperation();

        throw error;
      }
    },
    takeModelSnapshot(model) {
      const snapshot: Map<string, string | undefined> = new Map();

      snapshot.set('canisterId', model.canisterId.toText());
      snapshot.set('methodName', model.methodName);
      snapshot.set('permission', JSON.stringify(model.permission));
      snapshot.set('policies', JSON.stringify(model.requestPolicies));
      snapshot.set('validationCanisterId', JSON.stringify(model.validationCanisterId));
      snapshot.set('validationMethodName', JSON.stringify(model.validationMethodName));

      return JSON.stringify(Object.fromEntries(snapshot));
    },
  });

watch(valid, value => emit('valid', value));
watch(edited, value => emit('edited', value));
watch(submitting, value => emit('submitting', value));

const onFormInitialized = (): void => {
  // make sure there is a at least one request policy in the model
  if (!model.value.requestPolicies.length) {
    model.value = {
      ...model.value,
      requestPolicies: [{ rule: { AutoApproved: null }, policy_id: [] }],
    };
  }

  showAdvancedValidation.value = !!model.value.validationMethodName;
};

watch(
  submitted,
  (value, prev) => {
    if (prev === undefined || value === true) {
      onFormInitialized();
    }

    if (value) {
      emit('submitted');
    }
  },
  { immediate: true },
);

const deleteApprovalPolicyByIndex = (index: number) => {
  model.value.requestPolicies = model.value.requestPolicies.filter((_, idx) => idx !== index);
};

const deleteValidatioNMethod = () => {
  showAdvancedValidation.value = false;

  model.value = {
    ...model.value,
    validationCanisterId: undefined,
    validationMethodName: undefined,
  };
};

const addValidationMethod = () => {
  showAdvancedValidation.value = true;

  model.value = {
    ...model.value,
    validationMethodName: undefined,
    validationCanisterId: model.value.canisterId,
  };
};
</script>
