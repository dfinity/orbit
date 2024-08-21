<template>
  <VForm ref="form" @submit.prevent="submit">
    <RegistryUpdateMode
      v-if="props.mode === SystemUpgradeFormMode.Registry"
      v-model="modelValue"
      @valid="emit('valid', $event)"
      @loading="emit('loading', $event)"
    />
    <AdvancedUpdateMode
      v-else-if="props.mode === SystemUpgradeFormMode.Advanced"
      v-model="modelValue"
    />
  </VForm>
</template>

<script lang="ts" setup>
import { computed, ref, watch } from 'vue';
import { VForm } from 'vuetify/components';
import { SystemUpgradeFormMode, SystemUpgradeFormValue } from './system-upgrade.types';
import { VFormValidation } from '~/types/helper.types';
import AdvancedUpdateMode from './AdvancedUpdateMode.vue';
import RegistryUpdateMode from './RegistryUpdateMode.vue';

export type SystemUpgradeFormProps = {
  mode?: SystemUpgradeFormMode;
  modelValue: SystemUpgradeFormValue;
  valid?: boolean;
};

const form = ref<VFormValidation | null>(null);
const isFormValid = computed(() => (form.value ? form.value.isValid : false));

const props = withDefaults(defineProps<SystemUpgradeFormProps>(), {
  valid: false,
  mode: SystemUpgradeFormMode.Registry,
});

const emit = defineEmits<{
  (event: 'update:modelValue', payload: SystemUpgradeFormProps['modelValue']): void;
  (event: 'valid', payload: boolean): void;
  (event: 'loading', payload: boolean): void;
  (event: 'submit', payload: SystemUpgradeFormProps['modelValue']): void;
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
