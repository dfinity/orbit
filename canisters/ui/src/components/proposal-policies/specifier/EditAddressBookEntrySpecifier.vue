<template>
  <div class="d-flex ga-4 flex-column">
    <div class="d-flex ga-2">
      <VBtn :active="isAny" variant="outlined" @click="setSelectionMode('Any')">
        {{ $t('terms.all') }}
      </VBtn>
      <VBtn :active="isId" variant="outlined" @click="setSelectionMode('Id')">
        {{ $t('terms.subset') }}
      </VBtn>
    </div>
    <VAutocomplete
      v-if="isId"
      v-model="idsModel"
      :label="$t('app.search_items')"
      variant="underlined"
      density="comfortable"
      multiple
      :items="autocompleteItems"
      @update:search="autocomplete.searchItems"
    />
  </div>
</template>
<script setup lang="ts">
import { computed, toRefs } from 'vue';
import { CommonSpecifier, ProposalSpecifier } from '~/generated/wallet/wallet.did';
import { variantIs } from '~/utils/helper.utils';
import { useAddressBookAutocomplete } from '~/composables/autocomplete.composable';
import { onMounted } from 'vue';

const input = withDefaults(
  defineProps<{
    modelValue?: ProposalSpecifier & { EditAddressBookEntry: CommonSpecifier };
  }>(),
  {
    modelValue: () => ({ EditAddressBookEntry: { Any: null } }),
  },
);

const props = toRefs(input);

const emit = defineEmits<{
  (event: 'update:modelValue', payload: ProposalSpecifier): void;
}>();

const model = computed({
  get: () => props.modelValue.value,
  set: value => emit('update:modelValue', value),
});

const isAny = computed(() => variantIs(model.value.EditAddressBookEntry, 'Any'));
const isId = computed(() => variantIs(model.value.EditAddressBookEntry, 'Id'));

const idsModel = computed({
  get: () =>
    variantIs(model.value.EditAddressBookEntry, 'Id') ? model.value.EditAddressBookEntry.Id : [],
  set: value => {
    if (variantIs(model.value.EditAddressBookEntry, 'Id')) {
      model.value.EditAddressBookEntry.Id = value;
    }
  },
});

const setSelectionMode = (variant: 'Any' | 'Id'): void => {
  if (variantIs(model.value.EditAddressBookEntry, variant)) {
    return;
  }

  if (variant === 'Any') {
    model.value.EditAddressBookEntry = { Any: null };
    return;
  }

  if (variant === 'Id') {
    model.value.EditAddressBookEntry = { Id: [] };
    return;
  }
};

const autocomplete = useAddressBookAutocomplete();

const autocompleteItems = computed(() => {
  const items = autocomplete.results.value.map(item => ({
    title: item.address,
    value: item.id,
  }));

  return items;
});

onMounted(() => {
  autocomplete.searchItems();
});
</script>
