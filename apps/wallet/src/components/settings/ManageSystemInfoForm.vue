<template>
  <VForm ref="form" @submit.prevent="submit">
    <VTextField
      v-model="name"
      name="name"
      :label="$t('terms.name')"
      density="comfortable"
      :rules="[maxLengthRule(48, $t('terms.name'))]"
      :variant="isViewMode ? 'plain' : 'filled'"
      :disabled="isViewMode"
    />
  </VForm>
</template>

<script lang="ts" setup>
import { computed, ref, watch } from 'vue';
import { VForm, VTextField } from 'vuetify/components';
import { ManageSystemInfoOperationInput } from '~/generated/station/station.did';
import { VFormValidation } from '~/types/helper.types';
import { maxLengthRule } from '~/utils/form.utils';

const props = withDefaults(
  defineProps<{
    modelValue: Partial<ManageSystemInfoOperationInput>;
    valid?: boolean;
    triggerSubmit?: boolean;
    mode?: 'view' | 'edit';
  }>(),
  {
    valid: true,
    triggerSubmit: false,
    mode: 'edit',
  },
);

const emit = defineEmits<{
  (event: 'update:modelValue', payload: Partial<ManageSystemInfoOperationInput>): void;
  (event: 'update:triggerSubmit', payload: boolean): void;
  (event: 'valid', payload: boolean): void;
  (event: 'submit', payload: Partial<ManageSystemInfoOperationInput>): void;
}>();

const form = ref<VFormValidation | null>(null);
const isFormValid = computed(() => (form.value ? form.value.isValid : false));

const model = computed(() => props.modelValue);
watch(model.value, newValue => emit('update:modelValue', newValue), { deep: true });

watch(
  () => isFormValid.value,
  isValid => emit('valid', isValid ?? false),
);

const name = computed({
  get: () => model.value.name?.[0],
  set: value => {
    model.value.name = !value ? [] : [value];
  },
});

watch(
  () => props.triggerSubmit,
  () => {
    if (props.triggerSubmit) {
      emit('update:triggerSubmit', false);
      submit();
    }
  },
);

const isViewMode = computed(() => props.mode === 'view');

const submit = async () => {
  const { valid } = form.value ? await form.value.validate() : { valid: false };

  if (valid) {
    emit('submit', model.value);
  }
};
</script>
