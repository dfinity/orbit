<template>
  <VForm ref="form" @submit.prevent="submit">
    <VContainer class="px-0 py-2">
      <VRow>
        <VCol cols="12" class="pb-0">
          <CanisterIdField
            v-show="props.display.canisterId || !model.canisterId"
            v-model="model.canisterId"
            name="canister_id"
            :readonly="props.readonly"
          />
        </VCol>
        <VCol>
          <CyclesInput
            v-model="model.cycles"
            :label="$t('external_canisters.cycles')"
            name="cycles"
            :unit="CyclesUnit.Billion"
            :readonly="props.readonly"
            required
          />
        </VCol>
      </VRow>
    </VContainer>

    <slot name="actions"> </slot>
  </VForm>
</template>
<script lang="ts" setup>
import { computed, ref, watch } from 'vue';
import { VCol, VContainer, VForm, VRow } from 'vuetify/components';
import { CyclesUnit } from '~/types/app.types';
import { VFormValidation } from '~/types/helper.types';
import CanisterIdField from '../inputs/CanisterIdField.vue';
import CyclesInput from '../inputs/CyclesInput.vue';
import { CanisterTopUpModel } from './external-canisters.types';

const props = withDefaults(
  defineProps<{
    modelValue: CanisterTopUpModel;
    triggerSubmit?: boolean;
    saving?: boolean;
    readonly?: boolean;
    display?: {
      canisterId: boolean;
    };
  }>(),
  {
    readonly: false,
    saving: false,
    triggerSubmit: false,
    display: () => ({
      canisterId: true,
    }),
  },
);
const emit = defineEmits<{
  (event: 'update:modelValue', payload: CanisterTopUpModel): void;
  (event: 'update:triggerSubmit', payload: boolean): void;
  (event: 'valid', payload: boolean): void;
  (event: 'submit', payload: CanisterTopUpModel): void;
}>();

const form = ref<VFormValidation | null>(null);
const valid = ref(true);
const fieldsWithErrors = ref<string[]>([]);

const model = computed({
  get: () => props.modelValue,
  set: value => emit('update:modelValue', value),
});

const triggerSubmit = computed({
  get: () => props.triggerSubmit,
  set: value => emit('update:triggerSubmit', value),
});

watch(
  () => valid.value,
  () => emit('valid', valid.value),
);

watch(
  () => model.value,
  _ => {
    valid.value = form.value?.isValid ?? false;
    fieldsWithErrors.value = form.value?.errors.map(error => error.id) ?? [];
  },
  { deep: true },
);

watch(triggerSubmit, value => {
  if (value) {
    emit('update:triggerSubmit', false);

    submit();
  }
});

const revalidate = async (): Promise<boolean> => {
  const { valid: isValid, errors } = form.value
    ? await form.value.validate()
    : { valid: false, errors: [] };

  valid.value = isValid;

  fieldsWithErrors.value = errors.map(error => error.id);

  return isValid;
};

const submit = async () => {
  const isValid = await revalidate();

  if (isValid) {
    emit('submit', model.value);
  }
};
</script>
