<template>
  <VTextField
    v-model="canisterIdInput"
    :name="props.name"
    :label="$t('terms.canister_id')"
    :variant="props.variant"
    :density="props.density"
    :readonly="props.readonly"
    :rules="props.required ? [requiredRule, validCanisterId] : [validCanisterId]"
    :prepend-icon="!props.prependInnerIcon ? mdiIdentifier : undefined"
    :prepend-inner-icon="props.prependInnerIcon ? mdiIdentifier : undefined"
  />
</template>
<script lang="ts" setup>
import { Principal } from '@icp-sdk/core/principal';
import { mdiIdentifier } from '@mdi/js';
import { computed, ref, watch } from 'vue';
import { VTextField } from 'vuetify/components';
import { requiredRule, validCanisterId } from '~/utils/form.utils';

const props = withDefaults(
  defineProps<{
    modelValue?: Principal;
    readonly?: boolean;
    required?: boolean;
    name?: string;
    prependInnerIcon?: boolean;
    density?: 'comfortable' | 'compact' | 'default';
    variant?: 'filled' | 'outlined' | 'plain' | 'solo' | 'underlined';
  }>(),
  {
    modelValue: undefined,
    readonly: false,
    required: false,
    name: undefined,
    prependInnerIcon: false,
    density: 'comfortable',
    variant: 'filled',
  },
);

const emit = defineEmits<{
  (event: 'update:modelValue', payload?: Principal): void;
}>();

const model = computed({
  get: () => props.modelValue,
  set: value => emit('update:modelValue', value),
});

const canisterIdInput = ref<string | undefined>(props.modelValue?.toText());

watch(canisterIdInput, newValue => {
  try {
    newValue = newValue?.trim();

    if (!newValue) {
      throw new Error('Empty canisterId');
    }

    model.value = Principal.fromText(newValue);
  } catch (_) {
    // Unset the canisterId if the input is invalid or empty
    model.value = undefined;
  }
});

watch(
  () => model.value,
  updatedCanisterId => {
    if (updatedCanisterId) {
      canisterIdInput.value = updatedCanisterId.toText();
    }
  },
  { immediate: true },
);
</script>
