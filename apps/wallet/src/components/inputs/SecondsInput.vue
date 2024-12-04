<template>
  <div class="d-flex flex-row flex-nowrap ga-1">
    <VTextField
      ref="secondsFieldInput"
      v-model="displayedSeconds"
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
          min: unit === TimeUnit.Seconds ? 1 : 0.001,
          decimals: unit === TimeUnit.Seconds ? 0 : 3,
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
import { TimeUnit } from '~/types/app.types';
import { numberRangeRule, requiredRule } from '~/utils/form.utils';
import { parseToNumberOrUndefined } from '~/utils/helper.utils';
import { fromTimeUnit, toTimeUnit } from '~/mappers/time.mapper.ts';

const props = withDefaults(
  defineProps<{
    modelValue?: bigint;
    unit?: TimeUnit;
    label?: string;
    name?: string;
    readonly?: boolean;
    units?: TimeUnit[];
    required?: boolean;
    hint?: string;
    variant?: 'underlined' | 'outlined' | 'filled';
    density?: 'comfortable' | 'compact' | 'default';
  }>(),
  {
    modelValue: undefined,
    unit: TimeUnit.Seconds,
    label: undefined,
    name: undefined,
    readonly: false,
    required: false,
    hint: undefined,
    units: () => Object.values(TimeUnit),
    variant: 'filled',
    density: 'comfortable',
  },
);

const secondsFieldInput = ref<VTextField>();
const i18n = useI18n();
const unit = ref<TimeUnit>(props.unit);
const timeInSeconds = ref<bigint | undefined>(props.modelValue);
const displayedSeconds = ref<number | undefined>(
  props.modelValue ? toTimeUnit(props.modelValue, props.unit) : undefined,
);

const emit = defineEmits<{
  (event: 'update:modelValue', payload: bigint | undefined): void;
  (event: 'update:unit', payload: TimeUnit): void;
}>();

const availableUnits = computed(() =>
  props.units.map(unit => ({ value: unit, text: i18n.t(`time.units.${unit.toLowerCase()}`) })),
);

const syncTimeInSeconds = (seconds?: number): void => {
  // Reset model value if the input is invalid
  if (secondsFieldInput.value?.errorMessages?.length) {
    emit('update:modelValue', undefined);

    return;
  }

  seconds = parseToNumberOrUndefined(seconds);
  timeInSeconds.value = seconds !== undefined ? fromTimeUnit(seconds, unit.value) : undefined;

  emit('update:modelValue', timeInSeconds.value);
};

watch(
  () => secondsFieldInput.value?.errorMessages,
  _ => syncTimeInSeconds(displayedSeconds.value),
  { deep: true },
);

watch(
  () => displayedSeconds.value,
  cycles => syncTimeInSeconds(cycles),
);

watch(
  () => unit.value,
  value => {
    emit('update:unit', value);

    // Triggers revalidation with the updated rules when unit changes.
    if (displayedSeconds.value) {
      secondsFieldInput.value?.focus({
        preventScroll: true,
      });

      // Next tick is required to ensure the input is focused before blurring it.
      nextTick(() => {
        secondsFieldInput.value?.blur();

        // Update displayed cycles when unit changes
        syncTimeInSeconds(displayedSeconds.value);
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
