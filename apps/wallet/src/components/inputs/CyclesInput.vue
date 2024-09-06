<template>
  <div class="d-flex flex-row flex-nowrap ga-1">
    <VTextField
      v-model="displayedCycles"
      class="flex-grow-1"
      :name="props.name"
      :label="props.label"
      :variant="props.variant"
      :density="props.density"
      type="number"
      :rules="[intNumberRangeRule(props.label ?? 'cycles', 1, Number.MAX_SAFE_INTEGER)]"
      :prepend-icon="mdiDatabaseRefresh"
    />
    <div>
      <VSelect
        v-model="unit"
        :name="props.name ? `${props.name}_unit` : undefined"
        :items="Object.values(CyclesUnit)"
        :variant="props.variant"
        :density="props.density"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { mdiDatabaseRefresh } from '@mdi/js';
import { ref, watch } from 'vue';
import { VSelect, VTextField } from 'vuetify/components';
import { fromCyclesUnit, toCyclesUnit } from '~/mappers/cycles.mapper';
import { CyclesUnit } from '~/types/app.types';
import { intNumberRangeRule } from '~/utils/form.utils';
import { parseToBigIntOrUndefined } from '~/utils/helper.utils';

const props = withDefaults(
  defineProps<{
    modelValue?: bigint;
    unit?: CyclesUnit;
    label?: string;
    name?: string;
    variant?: 'underlined' | 'outlined' | 'filled';
    density?: 'comfortable' | 'compact' | 'default';
  }>(),
  {
    modelValue: undefined,
    unit: CyclesUnit.Smallest,
    label: undefined,
    name: undefined,
    variant: 'filled',
    density: 'comfortable',
  },
);

const unit = ref<CyclesUnit>(props.unit);
const e8sCycles = ref<bigint | undefined>(props.modelValue);
const displayedCycles = ref<bigint | undefined>(
  props.modelValue ? toCyclesUnit(props.modelValue, props.unit) : undefined,
);

const emit = defineEmits<{
  (event: 'update:modelValue', payload: bigint | undefined): void;
  (event: 'update:unit', payload: CyclesUnit): void;
}>();

watch(
  () => props.modelValue,
  value => {
    e8sCycles.value = value;
    displayedCycles.value = value ? toCyclesUnit(value, unit.value) : undefined;
  },
);

watch(
  () => displayedCycles.value,
  cycles => {
    cycles = parseToBigIntOrUndefined(cycles);
    e8sCycles.value = cycles !== undefined ? fromCyclesUnit(cycles, unit.value) : undefined;

    emit('update:modelValue', e8sCycles.value);
  },
);

watch(
  () => unit.value,
  value => {
    emit('update:unit', value);
  },
);

watch(
  () => props.unit,
  value => {
    if (value !== unit.value) {
      unit.value = value;
    }
  },
);
</script>
