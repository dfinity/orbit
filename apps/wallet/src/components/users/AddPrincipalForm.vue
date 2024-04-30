<template>
  <VForm ref="form" @submit.prevent="submit">
    <slot name="prepend"></slot>

    <VTextField
      v-model="modelValue"
      :label="$t('terms.identity')"
      variant="filled"
      :rules="[requiredRule, validPrincipalRule]"
    />
  </VForm>
</template>

<script lang="ts" setup>
import { computed, ref, watch } from 'vue';
import { VForm, VTextField } from 'vuetify/components';
import { VFormValidation } from '~/types/helper.types';
import { requiredRule, validPrincipalRule } from '~/utils/form.utils';

const form = ref<VFormValidation | null>(null);
const isFormValid = computed(() => (form.value ? form.value.isValid : false));

const props = withDefaults(
  defineProps<{
    modelValue: string | null;
    valid?: boolean;
  }>(),
  {
    valid: true,
  },
);

const emit = defineEmits<{
  (event: 'update:modelValue', payload: string | null): void;
  (event: 'valid', payload: boolean): void;
  (event: 'submit', payload: string | null): void;
}>();

watch(
  () => isFormValid.value,
  isValid => emit('valid', isValid ?? false),
);

const modelValue = computed({
  get: () => props.modelValue,
  set: value => emit('update:modelValue', value),
});

const submit = async () => {
  const { valid } = form.value ? await form.value.validate() : { valid: false };

  if (valid) {
    emit('submit', modelValue.value);
  }
};
</script>
