<template>
  <VForm ref="form" @submit.prevent="submit">
    <VTextField
      v-if="modelValue.id"
      v-model="modelValue.id"
      name="id"
      :label="$t('terms.id')"
      variant="plain"
      density="compact"
      disabled
    />
    <VTextField
      v-model="modelValue.name"
      name="name"
      :label="$t('terms.user_group')"
      :variant="isViewMode ? 'plain' : 'underlined'"
      :rules="rules.name"
      :disabled="isViewMode"
    />
  </VForm>
</template>

<script lang="ts" setup>
import { computed, reactive, ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import { UserGroup } from '~/generated/wallet/wallet.did';
import { FormValidationRules, VFormValidation } from '~/types/helper.types';
import { maxLengthRule, requiredRule } from '~/utils/form.utils';

const props = withDefaults(
  defineProps<{
    modelValue: Partial<UserGroup>;
    valid?: boolean;
    mode?: 'view' | 'edit';
    triggerSubmit?: boolean;
  }>(),
  {
    valid: true,
    mode: 'edit',
    triggerSubmit: false,
  },
);

const i18n = useI18n();
const form = ref<VFormValidation | null>(null);
const isFormValid = computed(() => (form.value ? form.value.isValid : false));
const rules: {
  name: FormValidationRules;
} = {
  name: [requiredRule, maxLengthRule(50, i18n.t('terms.name'))],
};

const emit = defineEmits<{
  (event: 'update:modelValue', payload: Partial<UserGroup>): void;
  (event: 'update:triggerSubmit', payload: boolean): void;
  (event: 'valid', payload: boolean): void;
  (event: 'submit', payload: Partial<UserGroup>): void;
}>();

watch(
  () => isFormValid.value,
  isValid => emit('valid', isValid ?? false),
);

const modelValue = reactive({ ...props.modelValue });

watch(
  () => props.triggerSubmit,
  () => {
    if (props.triggerSubmit) {
      emit('update:triggerSubmit', false);
      submit();
    }
  },
);

watch(
  () => modelValue,
  value => emit('update:modelValue', value),
  { deep: true },
);

const isViewMode = computed(() => props.mode === 'view');

const submit = async () => {
  const { valid } = form.value ? await form.value.validate() : { valid: false };

  if (valid) {
    emit('submit', modelValue);
  }
};
</script>
