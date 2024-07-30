<template>
  <VForm ref="form" @submit.prevent="submit">
    <RegistryUpdateMode
      v-if="props.mode === ChangeCanisterFormMode.Registry"
      v-model="modelValue"
      @valid="emit('valid', $event)"
      @loading="emit('loading', $event)"
    />
    <AdvancedUpdateMode
      v-else-if="props.mode === ChangeCanisterFormMode.Advanced"
      v-model="modelValue"
    />
  </VForm>
</template>

<script lang="ts" setup>
import { computed, ref, watch } from 'vue';
import { VForm } from 'vuetify/components';
import {
  ChangeCanisterFormMode,
  ChangeCanisterFormValue,
} from '~/components/change-canister/change-canister.types';
import { VFormValidation } from '~/types/helper.types';
import AdvancedUpdateMode from './AdvancedUpdateMode.vue';
import RegistryUpdateMode from './RegistryUpdateMode.vue';

export type ChangeCanisterFormProps = {
  mode?: ChangeCanisterFormMode;
  modelValue: ChangeCanisterFormValue;
  valid?: boolean;
};

const form = ref<VFormValidation | null>(null);
const isFormValid = computed(() => (form.value ? form.value.isValid : false));

const props = withDefaults(defineProps<ChangeCanisterFormProps>(), {
  valid: false,
  mode: ChangeCanisterFormMode.Registry,
});

const emit = defineEmits<{
  (event: 'update:modelValue', payload: ChangeCanisterFormProps['modelValue']): void;
  (event: 'valid', payload: boolean): void;
  (event: 'loading', payload: boolean): void;
  (event: 'submit', payload: ChangeCanisterFormProps['modelValue']): void;
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
