<template>
  <div class="d-flex ga-4 flex-column">
    <div v-if="!props.readonly.value" class="d-flex ga-2">
      <VBtn
        :active="isAny"
        :disabled="props.disabled.value"
        variant="outlined"
        size="small"
        @click="setSelectionMode('Any')"
      >
        {{ $t('terms.all') }}
      </VBtn>
      <VBtn
        :active="isIds"
        :disabled="props.disabled.value"
        variant="outlined"
        size="small"
        @click="setSelectionMode('Ids')"
      >
        {{ $t('terms.subset') }}
      </VBtn>
    </div>
    <AddressBookAutocomplete
      v-if="isIds"
      v-model="loadedEntries"
      :label="$t('terms.addresses')"
      variant="underlined"
      density="comfortable"
      multiple
      :disabled="props.disabled.value || props.readonly.value"
    />
  </div>
</template>
<script setup lang="ts">
import { computed, ref, toRefs, watch } from 'vue';
import AddressBookAutocomplete from '~/components/inputs/AddressBookAutocomplete.vue';
import { AddressBookEntry, ResourceIds } from '~/generated/station/station.did';
import { services } from '~/plugins/services.plugin';
import { variantIs } from '~/utils/helper.utils';

const input = withDefaults(
  defineProps<{
    modelValue?: ResourceIds;
    disabled?: boolean;
    readonly?: boolean;
  }>(),
  {
    modelValue: () => ({ Any: null }),
    disabled: false,
    readonly: false,
  },
);

const props = toRefs(input);

const stationService = services().station;

const emit = defineEmits<{
  (event: 'update:modelValue', payload: ResourceIds): void;
}>();

const model = computed({
  get: () => props.modelValue.value,
  set: value => emit('update:modelValue', value),
});

const isAny = computed(() => variantIs(model.value, 'Any'));
const isIds = computed(() => variantIs(model.value, 'Ids'));

const setSelectionMode = (variant: 'Any' | 'Ids'): void => {
  if (variantIs(model.value, variant)) {
    return;
  }

  if (variant === 'Any') {
    model.value = { Any: null };
    return;
  }

  if (variant === 'Ids') {
    model.value = { Ids: [] };
    return;
  }
};

const loadedEntries = ref<AddressBookEntry[]>([]);

watch(loadedEntries, () => {
  model.value = { Ids: loadedEntries.value.map(e => e.id) };
});

watch(
  isIds,
  async () => {
    if (isIds.value) {
      const list = variantIs(model.value, 'Ids') ? model.value['Ids'] : [];
      const allEntries = [];
      while (list.length > 0) {
        const entries = await stationService.listAddressBook({
          ids: list.splice(0, 100),
        });
        allEntries.push(...entries.address_book_entries);
      }
      loadedEntries.value = allEntries;
    }
  },
  {
    immediate: true,
  },
);
</script>
