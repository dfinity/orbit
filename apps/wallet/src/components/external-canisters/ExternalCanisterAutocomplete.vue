<template>
  <div class="d-flex flex-column">
    <label class="d-flex ga-1 align-center">
      <VIcon v-if="props.prependIcon" :icon="props.prependIcon" size="small" />
      {{ dropdownLabel }}
    </label>
    <div class="d-flex flex-column mt-1 flex-wrap">
      <VAutocomplete
        v-model="model"
        v-model:search="searchTerm"
        v-model:items="availableItems"
        :placeholder="props.placeholder"
        :multiple="props.multiple"
        item-value="value"
        item-title="text"
        :variant="props.variant"
        :density="props.density"
        :readonly="props.readonly"
        :disabled="props.disabled"
        :rules="props.rules"
        chips
        closable-chips
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import { VAutocomplete, VIcon } from 'vuetify/components';
import { useExternalCanistersAutocomplete } from '~/composables/external-canisters.composable';
import { FormValidationRuleFn, SelectItem } from '~/types/helper.types';

const props = withDefaults(
  defineProps<{
    search?: string;
    modelValue?: string[] | string;
    items?: SelectItem<string>[];
    label?: string;
    placeholder?: string;
    variant?: 'underlined' | 'outlined' | 'filled';
    density?: 'comfortable' | 'compact' | 'default';
    multiple?: boolean;
    readonly?: boolean;
    disabled?: boolean;
    prependIcon?: string;
    rules?: FormValidationRuleFn[];
  }>(),
  {
    search: '',
    modelValue: () => [],
    items: () => [],
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

const emit = defineEmits<{
  (event: 'update:modelValue', payload: string[] | string): void;
  (event: 'update:search', payload: string): void;
  (event: 'update:items', payload: SelectItem<string>[]): void;
}>();

const model = computed({
  get: () => props.modelValue,
  set: value => emit('update:modelValue', value),
});

const searchTerm = computed({
  get: () => props.search,
  set: value => emit('update:search', value),
});

const availableItems = computed({
  get: () => props.items,
  set: value => emit('update:items', value),
});

const autocomplete = useExternalCanistersAutocomplete();

const updateAvailableItemsList = (results: SelectItem<string>[] = []) => {
  const itemsSet = new Set(availableItems.value.map(i => i.value));
  const updatedItems = availableItems.value.filter(i => itemsSet.has(i.value));

  for (const item of results) {
    if (!itemsSet.has(item.value)) {
      updatedItems.push(item);

      itemsSet.add(item.value);
    }
  }

  availableItems.value = updatedItems;
};

const dropdownLabel = computed(() => props.label ?? i18n.t('terms.canisters'));

onMounted(() => {
  updateAvailableItemsList();
  autocomplete.searchItems();
});

watch(
  () => autocomplete.results.value,
  results => {
    const updatedItems = results.map(result => ({
      value: result.canister_id.toText(),
      text: result.name,
    }));

    updateAvailableItemsList(updatedItems);
  },
);
</script>
