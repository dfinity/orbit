<template>
  <VCombobox
    v-model="model"
    :custom-filter="() => true"
    :label="props.label.value"
    item-value="address"
    item-title="address"
    clear-on-select
    hide-selected
    :rules="props.required ? [requiredRule, addressValidator] : [addressValidator]"
    :return-object="false"
    :loading="autocomplete.loading.value"
    :items="items"
    :variant="props.variant.value"
    :density="props.density.value"
    :readonly="props.readonly.value"
    :disabled="props.disabled.value"
    @update:search="autocomplete.searchItems($event)"
  >
    <template #item="{ props: itemProps, item }">
      <!-- prettier-ignore -->
      <VListItem @click="(itemProps.onClick as any)">
        <VListItemTitle>{{ item.raw.address_owner }}</VListItemTitle>
        <VListItemSubtitle>{{ item.raw.address }}</VListItemSubtitle>
      </VListItem>
    </template>
  </VCombobox>
</template>
<script setup lang="ts">
import { computed, onMounted, ref, toRefs, watch } from 'vue';
import { VCombobox } from 'vuetify/components';
import { useAddressBookAutocomplete } from '~/composables/autocomplete.composable';
import { AddressBookEntry } from '~/generated/station/station.did';
import { requiredRule, validAddress } from '~/utils/form.utils';

const input = withDefaults(
  defineProps<{
    modelValue?: string;
    label?: string;
    variant?: 'underlined' | 'outlined' | 'solo';
    density?: 'comfortable' | 'compact';
    readonly?: boolean;
    disabled?: boolean;
    required?: boolean;
    blockchain: string;
  }>(),
  {
    modelValue: undefined,
    label: undefined,
    variant: undefined,
    density: 'comfortable',
    multiple: false,
    readonly: false,
    disabled: false,
    required: false,
  },
);

const props = toRefs(input);

const addressValidator = computed(() => validAddress(props.blockchain.value));

const emit = defineEmits<{
  (event: 'update:modelValue', payload: string | undefined): void;
}>();

const model = computed({
  get: () => props.modelValue.value,
  set: value => emit('update:modelValue', value),
});

const items = ref<AddressBookEntry[]>([]);

const autocomplete = useAddressBookAutocomplete();

onMounted(() => {
  autocomplete.searchItems();
});

watch(
  () => autocomplete.results.value,
  results => {
    items.value = results;
  },
);
</script>
