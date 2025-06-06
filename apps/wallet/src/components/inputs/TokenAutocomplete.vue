<template>
  <VAutocomplete
    v-model="model"
    :multiple="props.multiple.value"
    :label="props.label.value"
    item-value="id"
    :item-title="item => `${item.name} (${item.symbol})`"
    :items="items"
    :variant="props.variant.value"
    :density="props.density.value"
    :readonly="props.readonly.value"
    :disabled="props.disabled.value"
    :rules="props.rules.value"
    :no-data-text="props.noDataText.value"
    data-test-id="token-autocomplete"
  />
</template>

<script setup lang="ts">
import { computed, toRefs } from 'vue';
import { VAutocomplete } from 'vuetify/components';
import { useStationStore } from '~/stores/station.store';
import { FormValidationRuleFn } from '~/types/helper.types';

const input = withDefaults(
  defineProps<{
    modelValue?: string | string[];
    excludedIds?: string[];
    label?: string;
    variant?: 'underlined' | 'outlined' | 'filled' | 'plain';
    density?: 'comfortable' | 'compact';
    multiple?: boolean;
    readonly?: boolean;
    disabled?: boolean;
    rules?: FormValidationRuleFn[];
    noDataText?: string;
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
    excludedIds: undefined,
    noDataText: undefined,
  },
);

const props = toRefs(input);

const station = useStationStore();

const model = computed({
  get: () => props.modelValue.value,
  set: value => emit('update:modelValue', value),
});

const emit = defineEmits<{
  (event: 'update:modelValue', payload?: string | string[]): void;
}>();

const items = computed(() =>
  props.excludedIds
    ? station.configuration.details.supported_assets.filter(
        asset => !props.excludedIds.value?.includes(asset.id),
      )
    : station.configuration.details.supported_assets,
);
</script>
