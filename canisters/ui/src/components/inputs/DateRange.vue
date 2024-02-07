<template>
  <div class="d-flex flex-column">
    <label v-if="props.label" class="d-flex ga-1 align-center">
      <VIcon v-if="props.prependIcon" :icon="props.prependIcon" size="small" />
      {{ props.label }}
    </label>
    <div class="d-flex flex-row ga-1 mt-2 flex-wrap">
      <VMenu v-model="openFromDateSelector" :close-on-content-click="false" location="end">
        <template #activator="{ props: menuProps }">
          <VTextField
            readonly
            density="compact"
            variant="solo"
            flat
            v-bind="menuProps"
            :label="$t('terms.from')"
            :model-value="modelValue.from ? modelValue.from.toLocaleDateString() : undefined"
          />
        </template>
        <VDialog v-if="app.isMobile" v-model="openFromDateSelector" fullscreen>
          <DateSelector
            :model-value="modelValue.from"
            :max="modelValue.to"
            @close="openFromDateSelector = false"
            @update:model-value="updateDateFrom"
          />
        </VDialog>
        <DateSelector
          v-else
          :model-value="modelValue.from"
          :max="modelValue.to"
          @close="openFromDateSelector = false"
          @update:model-value="updateDateFrom"
        />
      </VMenu>
      <VMenu v-model="openToDateSelector" :close-on-content-click="false" location="end">
        <template #activator="{ props: menuProps }">
          <VTextField
            readonly
            density="compact"
            variant="solo"
            flat
            v-bind="menuProps"
            :label="$t('terms.until')"
            :model-value="modelValue.to ? modelValue.to.toLocaleDateString() : undefined"
          />
        </template>
        <VDialog v-if="app.isMobile" v-model="openToDateSelector" fullscreen>
          <DateSelector
            :model-value="modelValue.to"
            :min="modelValue.from"
            @close="openToDateSelector = false"
            @update:model-value="updateDateTo"
          />
        </VDialog>
        <DateSelector
          v-else
          :model-value="modelValue.to"
          :min="modelValue.from"
          @close="openToDateSelector = false"
          @update:model-value="updateDateTo"
        />
      </VMenu>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue';
import { useAppStore } from '~/stores/app.store';
import { endOfDay, startOfDay } from '~/utils/date.utils';
import DateSelector from './DateSelector.vue';

export type DateRangeModel = { from?: Date; to?: Date };

const app = useAppStore();

const props = withDefaults(
  defineProps<{
    modelValue?: DateRangeModel;
    prependIcon?: string;
    label?: string;
    color?: string;
  }>(),
  {
    label: undefined,
    color: 'primary-variant',
    prependIcon: undefined,
    modelValue: () => ({ from: undefined, to: undefined }),
  },
);

const openFromDateSelector = ref(false);
const openToDateSelector = ref(false);

const emit = defineEmits<{
  (event: 'update:modelValue', payload?: DateRangeModel): void;
}>();

const modelValue = computed({
  get: () => props.modelValue,
  set: value => emit('update:modelValue', value),
});

const updateDateFrom = (date?: Date): void => {
  if (!date) {
    modelValue.value.from = undefined;
    return;
  }

  modelValue.value.from = date;

  updateDateRange({ keep: 'from' });
};

const updateDateTo = (date?: Date): void => {
  if (!date) {
    modelValue.value.to = undefined;
    return;
  }

  modelValue.value.to = date;

  updateDateRange({ keep: 'to' });
};

const updateDateRange = ({ keep }: { keep: 'from' | 'to' }) => {
  if (!modelValue.value.from || !modelValue.value.to) {
    return;
  }

  switch (keep) {
    case 'from':
      if (modelValue.value.from > modelValue.value.to) {
        modelValue.value = { ...modelValue.value, to: endOfDay(modelValue.value.from) };
      }
      break;
    case 'to':
      if (modelValue.value.to < modelValue.value.from) {
        modelValue.value = { ...modelValue.value, from: startOfDay(modelValue.value.to) };
      }
      break;
  }
};
</script>
