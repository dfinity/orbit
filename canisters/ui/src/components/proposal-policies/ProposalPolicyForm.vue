<template>
  <VForm ref="form" @submit.prevent="submit">
    <VTextField
      v-if="model.id && props.display.value.id"
      v-model="model.id"
      name="id"
      :label="$t('terms.id')"
      variant="plain"
      density="compact"
      readonly
    />
  </VForm>
</template>

<script lang="ts" setup>
import { computed, ref, toRefs } from 'vue';
import { ProposalPolicy } from '~/generated/wallet/wallet.did';
import { VFormValidation } from '~/types/helper.types';

export type ProposalPolicyFormProps = {
  modelValue: Partial<ProposalPolicy>;
  valid?: boolean;
  display?: {
    id?: boolean;
    specifier?: boolean;
  };
};

const form = ref<VFormValidation | null>(null);

const p = withDefaults(defineProps<ProposalPolicyFormProps>(), {
  valid: true,
  display: () => ({
    id: true,
    specifier: true,
  }),
});
const props = toRefs(p);

const emit = defineEmits<{
  (event: 'update:modelValue', payload: ProposalPolicyFormProps['modelValue']): void;
  (event: 'valid', payload: boolean): void;
  (event: 'submit', payload: ProposalPolicyFormProps['modelValue']): void;
}>();

const model = computed({
  get: () => props.modelValue.value,
  set: value => emit('update:modelValue', value),
});

const submit = async () => {
  const { valid } = form.value ? await form.value.validate() : { valid: false };

  if (valid) {
    emit('submit', model.value);
  }
};
</script>
