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
            :model-value="modelValue.from?.toLocaleDateString()"
          />
        </template>
        <VDialog v-if="app.isMobile" v-model="openFromDateSelector" fullscreen>
          <DateSelector
            v-model="modelValue.from"
            :max="modelValue.to"
            @close="openFromDateSelector = false"
            @update:model-value="updateDateRange({ keep: 'from' })"
          />
        </VDialog>
        <DateSelector
          v-else
          v-model="modelValue.from"
          :max="modelValue.to"
          @close="openFromDateSelector = false"
          @update:model-value="updateDateRange({ keep: 'from' })"
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
            :model-value="modelValue.to?.toLocaleDateString()"
          />
        </template>
        <VDialog v-if="app.isMobile" v-model="openToDateSelector" fullscreen>
          <DateSelector
            v-model="modelValue.to"
            :min="modelValue.from"
            @close="openToDateSelector = false"
            @update:model-value="updateDateRange({ keep: 'to' })"
          />
        </VDialog>
        <DateSelector
          v-else
          v-model="modelValue.to"
          :min="modelValue.from"
          @close="openToDateSelector = false"
          @update:model-value="updateDateRange({ keep: 'to' })"
        />
      </VMenu>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { computed } from 'vue';
import DateSelector from './DateSelector.vue';
import { useAppStore } from '~/ui/stores/app';

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

const updateDateRange = ({ keep }: { keep: 'from' | 'to' }) => {
  if (!modelValue.value.from || !modelValue.value.to) {
    return;
  }

  switch (keep) {
    case 'from':
      if (modelValue.value.from > modelValue.value.to) {
        modelValue.value = { ...modelValue.value, to: modelValue.value.from };
      }
      break;
    case 'to':
      if (modelValue.value.to < modelValue.value.from) {
        modelValue.value = { ...modelValue.value, from: modelValue.value.to };
      }
      break;
  }
};
</script>
