<template>
  <VForm ref="form" @submit.prevent="submit">
    <VTextField
      v-if="modelValue.id"
      v-model="modelValue.id"
      name="id"
      :label="$t('terms.id')"
      variant="plain"
      density="compact"
      readonly
    />
    <VTextField
      v-model="modelValue.name"
      name="name"
      :label="$t('terms.user_group')"
      :variant="isViewMode ? 'plain' : 'underlined'"
      :rules="rules.name"
      :readonly="isViewMode"
    />
  </VForm>
</template>

<script lang="ts" setup>
import { computed, reactive, ref, watch } from 'vue';
import { UserGroup } from '~/generated/wallet/wallet.did';
import { i18n } from '~/modules/i18n.module';
import { FormValidationRules, VFormValidation } from '~/types/utils.types';
import { maxLengthRule, requiredRule } from '~/utils/form.utils';

const form = ref<VFormValidation | null>(null);
const isFormValid = computed(() => (form.value ? form.value.isValid : false));
const rules: {
  name: FormValidationRules;
} = {
  name: [requiredRule, maxLengthRule(50, i18n.global.t('terms.name'))],
};

const props = withDefaults(
  defineProps<{
    modelValue: Partial<UserGroup>;
    valid?: boolean;
    mode?: 'view' | 'edit';
  }>(),
  {
    valid: true,
    mode: 'edit',
  },
);

const emit = defineEmits<{
  (event: 'update:modelValue', payload: Partial<UserGroup>): void;
  (event: 'valid', payload: boolean): void;
  (event: 'submit', payload: Partial<UserGroup>): void;
}>();

watch(
  () => isFormValid.value,
  isValid => emit('valid', isValid ?? false),
);

const modelValue = reactive({ ...props.modelValue });

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
