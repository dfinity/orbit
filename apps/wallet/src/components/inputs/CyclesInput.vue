<template>
  <div class="d-flex flex-row flex-nowrap ga-1">
    <VTextField
      ref="cyclesFieldInput"
      v-model="displayedCycles"
      class="flex-grow-1"
      :name="props.name"
      :label="props.label"
      :variant="props.variant"
      :density="props.density"
      :readonly="props.readonly"
      type="number"
      :hint="props.hint"
      :persistent-hint="!!props.hint"
      :rules="[
        ...(props.required ? [requiredRule] : []),
        numberRangeRule({
          min: unit === CyclesUnit.Smallest ? 1 : 0.001,
          decimals: unit === CyclesUnit.Smallest ? 0 : 3,
        }),
      ]"
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
import { computed, nextTick, ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import { VSelect, VTextField } from 'vuetify/components';
import { fromCyclesUnit, toCyclesUnit } from '~/mappers/cycles.mapper';
import { CyclesUnit } from '~/types/app.types';
import { numberRangeRule, requiredRule } from '~/utils/form.utils';
import { parseToNumberOrUndefined } from '~/utils/helper.utils';

const props = withDefaults(
  defineProps<{
    modelValue?: bigint;
    unit?: CyclesUnit;
    label?: string;
    name?: string;
    readonly?: boolean;
    units?: CyclesUnit[];
    required?: boolean;
    hint?: string;
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
    hint: undefined,
    units: () => Object.values(CyclesUnit),
    variant: 'filled',
    density: 'comfortable',
  },
);

const cyclesFieldInput = ref<VTextField>();
const i18n = useI18n();
const unit = ref<CyclesUnit>(props.unit);
const e8sCycles = ref<bigint | undefined>(props.modelValue);
const displayedCycles = ref<number | undefined>(
  props.modelValue ? toCyclesUnit(props.modelValue, props.unit) : undefined,
);

const emit = defineEmits<{
  (event: 'update:modelValue', payload: bigint | undefined): void;
  (event: 'update:unit', payload: CyclesUnit): void;
}>();

const availableUnits = computed(() =>
  props.units.map(unit => ({ value: unit, text: i18n.t(`cycles.units.${unit.toLowerCase()}`) })),
);

const e8sSyncCycles = (cycles?: number): void => {
  // Reset model value if the input is invalid
  if (cyclesFieldInput.value?.errorMessages?.length) {
    emit('update:modelValue', undefined);

    return;
  }

  cycles = parseToNumberOrUndefined(cycles);
  e8sCycles.value = cycles !== undefined ? fromCyclesUnit(cycles, unit.value) : undefined;

  emit('update:modelValue', e8sCycles.value);
};

watch(
  () => cyclesFieldInput.value?.errorMessages,
  _ => e8sSyncCycles(displayedCycles.value),
  { deep: true },
);

watch(
  () => displayedCycles.value,
  cycles => e8sSyncCycles(cycles),
);

watch(
  () => unit.value,
  value => {
    emit('update:unit', value);

    // Triggers revalidation with the updated rules when unit changes.
    if (displayedCycles.value) {
      cyclesFieldInput.value?.focus({
        preventScroll: true,
      });

      // Next stick is required to ensure the input is focused before blurring it.
      nextTick(() => {
        cyclesFieldInput.value?.blur();

        // Update displayed cycles when unit changes
        e8sSyncCycles(displayedCycles.value);
      });
    }
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
