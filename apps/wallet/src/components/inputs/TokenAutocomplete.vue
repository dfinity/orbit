<template>
  <VAutocomplete
    v-model="model"
    :multiple="props.multiple.value"
    :label="props.label.value"
    item-value="value"
    item-title="text"
    :items="items"
    :variant="props.variant.value"
    :density="props.density.value"
    :readonly="props.readonly.value"
    :disabled="props.disabled.value"
    :rules="props.rules.value"
  />
</template>

<script setup lang="ts">
import { computed, toRefs } from 'vue';
import { Asset } from '~/generated/station/station.did';
import { useStationStore } from '~/stores/station.store';
import { FormValidationRuleFn } from '~/types/helper.types';

const input = withDefaults(
  defineProps<{
    modelValue?: string | string[];
    label?: string;
    variant?: 'underlined' | 'outlined' | 'filled';
    density?: 'comfortable' | 'compact';
    multiple?: boolean;
    readonly?: boolean;
    disabled?: boolean;
    rules?: FormValidationRuleFn[];
  }>(),
  {
    modelValue: undefined,
    label: undefined,
    variant: 'underlined',
    density: 'comfortable',
    multiple: false,
    readonly: false,
    disabled: false,
    rules: undefined,
  },
);

const props = toRefs(input);

const station = useStationStore();

const model = computed({
  get: () => props.modelValue.value,
  set: value => {
    emit(
      'selectedAsset',
      station.configuration.details.supported_assets.find(token => token.symbol === value),
    );
    emit('update:modelValue', value);
  },
});

const emit = defineEmits<{
  (event: 'update:modelValue', payload?: string): void;
  (event: 'selectedAsset', payload?: Asset): void;
}>();

const items = computed(() =>
  station.configuration.details.supported_assets.map(token => ({
    value: token.id,
    text: `${token.name} (${token.symbol})`,
  })),
);
</script>
