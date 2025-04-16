<template>
  <VForm ref="form" @submit.prevent="submit">
    <slot name="prepend"></slot>

    <UserGroupAutocomplete
      v-model="modelValue.user_group_id"
      name="group"
      density="comfortable"
      :label="$t('terms.user_group')"
      variant="filled"
      :rules="[requiredRule]"
      chips
      :disabled="mode === 'view'"
    />

    <VTextField
      v-model="quorum"
      name="quorum"
      :label="$t('terms.quorum')"
      variant="filled"
      density="comfortable"
      :rules="[requiredRule, intNumberRangeRule($t('terms.min'), 1)]"
      :disabled="mode === 'view'"
    />
  </VForm>
</template>

<script lang="ts" setup>
import { computed, ref, watch } from 'vue';
import { VForm, VTextField } from 'vuetify/components';
import { UUID } from '~/generated/station/station.did';
import { VFormValidation } from '~/types/helper.types';
import { intNumberRangeRule, requiredRule } from '~/utils/form.utils';
import UserGroupAutocomplete from '../inputs/UserGroupAutocomplete.vue';

const form = ref<VFormValidation | null>(null);
const isFormValid = computed(() => (form.value ? form.value.isValid : false));

export type DisasterRecoveryModel = {
  user_group_id?: UUID;
  quorum: number;
};

const props = withDefaults(
  defineProps<{
    modelValue: DisasterRecoveryModel;
    valid?: boolean;
    mode?: 'view' | 'edit';
  }>(),
  {
    valid: false,
  },
);

const emit = defineEmits<{
  (event: 'update:modelValue', payload: DisasterRecoveryModel): void;
  (event: 'valid', payload: boolean): void;
  (event: 'submit', payload: DisasterRecoveryModel): void;
}>();

watch(
  () => isFormValid.value,
  isValid => emit('valid', isValid ?? false),
);

const modelValue = computed({
  get: () => props.modelValue,
  set: value => emit('update:modelValue', value),
});

const quorum = computed({
  get: () => modelValue.value.quorum,
  set: value => {
    if (value && typeof value !== 'number') {
      value = parseInt(value, 10);
    }

    modelValue.value.quorum = value;
    modelValue.value = {
      ...modelValue.value,
    };
  },
});

const submit = async () => {
  const { valid } = form.value ? await form.value.validate() : { valid: false };

  if (valid) {
    emit('submit', modelValue.value);
  }
};
</script>
