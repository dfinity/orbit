<template>
  <VAutocomplete
    v-model="model"
    :multiple="props.multiple.value"
    :label="dropdownLabel"
    item-value="value"
    item-title="text"
    :items="items"
    :variant="props.variant.value"
    :density="props.density.value"
    :readonly="props.readonly.value"
    :disabled="props.disabled.value"
    @update:search="autocomplete.search"
  />
</template>
<script setup lang="ts">
import { computed, onMounted, ref, toRefs, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import { useUserGroupsAutocomplete } from '~/composables/autocomplete.composable';
import { UUID } from '~/generated/wallet/wallet.did';
import { SelectItem } from '~/types/helper.types';

const input = withDefaults(
  defineProps<{
    modelValue?: UUID[] | UUID;
    label?: string;
    variant?: 'underlined' | 'outlined' | 'plain' | 'filled';
    density?: 'comfortable' | 'compact';
    multiple?: boolean;
    readonly?: boolean;
    disabled?: boolean;
  }>(),
  {
    modelValue: () => [],
    label: undefined,
    variant: 'underlined',
    density: 'comfortable',
    multiple: false,
    readonly: false,
    disabled: false,
  },
);

const props = toRefs(input);
const i18n = useI18n();

const emit = defineEmits<{
  (event: 'update:modelValue', payload: UUID[] | UUID): void;
  (event: 'remove', payload: void): void;
}>();

const model = computed({
  get: () => props.modelValue.value,
  set: value => emit('update:modelValue', value),
});

const items = ref<SelectItem[]>([]);

const autocomplete = useUserGroupsAutocomplete();

const updateAvailableItemsList = (results: SelectItem[] = []) => {
  const selectedItems = Array.isArray(model.value) ? model.value : [model.value];
  for (const item of selectedItems) {
    const found = results.find(i => i.value === item);
    if (!found) {
      results.push({
        value: item,
        text: item,
      });
    }
  }

  items.value = results;
};

const dropdownLabel = computed(() => props.label.value ?? i18n.t('terms.user_groups'));

onMounted(() => {
  updateAvailableItemsList();
  autocomplete.searchItems();
});

watch(
  () => autocomplete.results.value,
  results => {
    const updatedItems = results.map(result => ({
      value: result.id,
      text: result.name,
    }));

    updateAvailableItemsList(updatedItems);
  },
);
</script>
