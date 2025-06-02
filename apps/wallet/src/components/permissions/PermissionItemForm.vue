<template>
  <VForm v-bind="$attrs" ref="form" @submit.prevent="submit">
    <slot name="errors" :errors="additionalFieldErrors">
      <FormErrorsContainer v-bind="{ errors: additionalFieldErrors }" />
    </slot>
    <div class="d-flex flex-column ga-2">
      <div>
        <p class="text-body-1 font-weight-bold">{{ title }}</p>
        <p v-if="explainer" class="text-body-2 text-medium-emphasis">{{ explainer }}</p>
      </div>
      <div>
        <p class="text-body-1 font-weight-medium mb-2">
          {{ $t('permissions.permitted_users') }}
        </p>
        <DiffView :before-value="props.currentPermission?.allow" :after-value="model">
          <template #default="{ value, diffMode }">
            <AllowInput
              v-if="value"
              :model-value="value"
              :mode="props.readonly ? 'view' : diffMode === 'before' ? 'view' : 'edit'"
              @update:model-value="val => diffMode === 'after' && (model = val)"
            />
          </template>
        </DiffView>
      </div>
    </div>
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
import { computed, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import { VForm } from 'vuetify/components';
import FormActions from '~/components/ui/FormActions.vue';
import FormErrorsContainer from '~/components/ui/FormErrorsContainer.vue';
import { useForm } from '~/composables/forms.composable';
import {
  useOnFailedOperation,
  useOnSuccessfulOperation,
} from '~/composables/notifications.composable';
import { Allow, Resource, Permission } from '~/generated/station/station.did';
import { fromResourceToDisplayText } from '~/mappers/permissions.mapper';
import { useStationStore } from '~/stores/station.store';
import { isApiError } from '~/utils/app.utils';
import AllowInput from '../inputs/AllowInput.vue';
import DiffView from '~/components/requests/DiffView.vue';

const props = withDefaults(
  defineProps<{
    resource: Resource;
    modelValue: Allow;
    readonly?: boolean;
    currentPermission?: Permission;
  }>(),
  {
    readonly: false,
    currentPermission: undefined,
  },
);

const emit = defineEmits<{
  (event: 'update:modelValue', payload: Allow): void;
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
const title = computed(() =>
  i18n.te(`permissions.actions.${fromResourceToDisplayText(props.resource)}`)
    ? i18n.t(`permissions.actions.${fromResourceToDisplayText(props.resource)}`)
    : fromResourceToDisplayText(props.resource),
);
const explainer = computed(() =>
  i18n.te(`permissions.actions.${fromResourceToDisplayText(props.resource)}_description`)
    ? i18n.t(`permissions.actions.${fromResourceToDisplayText(props.resource)}_description`)
    : undefined,
);

const { submit, edited, additionalFieldErrors, fieldsWithErrors, submitting, valid, submitted } =
  useForm({
    model,
    submit: async (updatedModel: Allow): Promise<void> => {
      try {
        const request = await station.service.editPermission({
          resource: props.resource,
          auth_scope: [updatedModel.auth_scope],
          users: [updatedModel.users],
          user_groups: [updatedModel.user_groups],
        });

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
          errorMessages.push(i18n.t('app.request_submit_failed'));
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
</script>
