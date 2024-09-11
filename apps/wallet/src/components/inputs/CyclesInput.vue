<template>
  <div class="d-flex flex-row flex-nowrap ga-1">
    <VTextField
      v-model="displayedCycles"
      class="flex-grow-1"
      :name="props.name"
      :label="props.label"
      :variant="props.variant"
      :density="props.density"
      :readonly="props.readonly"
      type="number"
      :rules="
        props.required
          ? [requiredRule, intNumberRangeRule(props.label ?? 'cycles', 1, Number.MAX_SAFE_INTEGER)]
          : [intNumberRangeRule(props.label ?? 'cycles', 1, Number.MAX_SAFE_INTEGER)]
      "
      :prepend-icon="mdiDatabaseRefresh"
    />
    <div>
      <VSelect
        v-model="unit"
        :name="props.name ? `${props.name}_unit` : undefined"
        :items="availableUnits"
        item-value="value"
        item-title="text"
        :readonly="props.readonly"
        :variant="props.variant"
        :density="props.density"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { mdiDatabaseRefresh } from '@mdi/js';
import { computed, ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import { VSelect, VTextField } from 'vuetify/components';
import { fromCyclesUnit, toCyclesUnit } from '~/mappers/cycles.mapper';
import { CyclesUnit } from '~/types/app.types';
import { intNumberRangeRule, requiredRule } from '~/utils/form.utils';
import { parseToBigIntOrUndefined } from '~/utils/helper.utils';

const props = withDefaults(
  defineProps<{
    modelValue?: bigint;
    unit?: CyclesUnit;
    label?: string;
    name?: string;
    readonly?: boolean;
    units?: CyclesUnit[];
    required?: boolean;
    variant?: 'underlined' | 'outlined' | 'filled';
    density?: 'comfortable' | 'compact' | 'default';
  }>(),
  {
    modelValue: undefined,
    unit: CyclesUnit.Smallest,
    label: undefined,
    name: undefined,
    readonly: false,
    required: false,
    units: () => Object.values(CyclesUnit),
    variant: 'filled',
    density: 'comfortable',
  },
);

const i18n = useI18n();
const unit = ref<CyclesUnit>(props.unit);
const e8sCycles = ref<bigint | undefined>(props.modelValue);
const displayedCycles = ref<bigint | undefined>(
  props.modelValue ? toCyclesUnit(props.modelValue, props.unit) : undefined,
);

const emit = defineEmits<{
  (event: 'update:modelValue', payload: bigint | undefined): void;
  (event: 'update:unit', payload: CyclesUnit): void;
}>();

const availableUnits = computed(() =>
  props.units.map(unit => ({ value: unit, text: i18n.t(`cycles.units.${unit.toLowerCase()}`) })),
);

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
