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
      variant="underlined"
      :rules="rules.name"
    />
  </VForm>
</template>

<script lang="ts" setup>
import { computed, reactive, ref, watch } from 'vue';
import { UserGroup } from '~/generated/wallet/wallet.did';
import { i18n } from '~/ui/modules/i18n';
import { FormValidationRules, VFormValidation } from '~/ui/types';
import { maxLengthRule, requiredRule } from '~/ui/utils';

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
  }>(),
  {
    valid: true,
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


const submit = async () => {
  const { valid } = form.value ? await form.value.validate() : { valid: false };

  if (valid) {
    emit('submit', modelValue);
  }
};
</script>
