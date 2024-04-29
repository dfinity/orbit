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
      v-model="idsModel"
      :label="$t('terms.addresses')"
      variant="underlined"
      density="comfortable"
      multiple
      :disabled="props.disabled.value || props.readonly.value"
    />
  </div>
</template>
<script setup lang="ts">
import { computed, toRefs } from 'vue';
import AddressBookAutocomplete from '~/components/inputs/AddressBookAutocomplete.vue';
import { ResourceIds } from '~/generated/station/station.did';
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

const emit = defineEmits<{
  (event: 'update:modelValue', payload: ResourceIds): void;
}>();

const model = computed({
  get: () => props.modelValue.value,
  set: value => emit('update:modelValue', value),
});

const isAny = computed(() => variantIs(model.value, 'Any'));
const isIds = computed(() => variantIs(model.value, 'Ids'));

const idsModel = computed({
  get: () => (variantIs(model.value, 'Ids') ? model.value.Ids : []),
  set: value => {
    if (variantIs(model.value, 'Ids')) {
      model.value.Ids = value;
    }
  },
});

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
</script>
