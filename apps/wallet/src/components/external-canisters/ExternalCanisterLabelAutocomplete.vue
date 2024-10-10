<template>
  <div class="d-flex flex-column ga-1">
    <label class="d-flex ga-1 align-center">
      <VIcon v-if="props.prependIcon" :icon="props.prependIcon" size="small" />
      {{ dropdownLabel }}
    </label>
    <VAutocomplete
      v-model="model"
      :multiple="props.multiple"
      :placeholder="props.placeholder"
      item-value="value"
      item-title="text"
      :items="items"
      :variant="props.variant"
      :density="props.density"
      :readonly="props.readonly"
      :disabled="props.disabled"
      :rules="props.rules"
      chips
      closable-chips
    />
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, ref } from 'vue';
import { useI18n } from 'vue-i18n';
import { VAutocomplete } from 'vuetify/components';
import logger from '~/core/logger.core';
import { useStationStore } from '~/stores/station.store';
import { FormValidationRuleFn, SelectItem } from '~/types/helper.types';

const props = withDefaults(
  defineProps<{
    modelValue?: string[];
    label?: string;
    variant?: 'underlined' | 'outlined' | 'filled';
    density?: 'comfortable' | 'compact' | 'default';
    multiple?: boolean;
    readonly?: boolean;
    disabled?: boolean;
    prependIcon?: string;
    placeholder?: string;
    rules?: FormValidationRuleFn[];
  }>(),
  {
    modelValue: () => [],
    label: undefined,
    variant: 'filled',
    density: 'comfortable',
    multiple: false,
    readonly: false,
    disabled: false,
    placeholder: '*',
    prependIcon: undefined,
    rules: undefined,
  },
);

const i18n = useI18n();
const station = useStationStore();

const emit = defineEmits<{
  (event: 'update:modelValue', payload: string[]): void;
}>();

const model = computed({
  get: () => props.modelValue,
  set: value => emit('update:modelValue', value),
});

const items = ref<SelectItem[]>([]);
const dropdownLabel = computed(() => props.label ?? i18n.t('terms.labels'));

onMounted(async () => {
  try {
    const result = await station.service.fetchExternalCanisterFilters({
      with_labels: true,
    });
    const labels = result.labels?.[0] ?? [];
    items.value = labels.map((label: string) => ({ value: label, text: label }));
  } catch (error) {
    logger.error('Failed to fetch external canister filters', error);
  }
});
</script>
