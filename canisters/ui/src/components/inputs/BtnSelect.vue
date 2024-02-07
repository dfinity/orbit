<template>
  <div class="d-flex flex-column">
    <label v-if="props.label" class="d-flex ga-1 align-center">
      <VIcon v-if="props.prependIcon" :icon="props.prependIcon" size="small" />
      {{ props.label }}
    </label>
    <div class="d-flex flex-row ga-1 mt-2 flex-wrap">
      <VBtn
        v-for="(item, idx) in props.items"
        :key="idx"
        :variant="modelValue.includes(item.key) ? 'flat' : 'tonal'"
        size="small"
        density="comfortable"
        :color="props.color"
        class="flex-grow-1"
        @click="toggle(item.key)"
      >
        {{ item.text }}
      </VBtn>
    </div>
  </div>
</template>

<script setup lang="ts" generic="T">
import { computed } from 'vue';

const props = withDefaults(
  defineProps<{
    modelValue?: T[];
    prependIcon?: string;
    label?: string;
    color?: string;
    items?: { key: T; text: string }[];
  }>(),
  {
    label: undefined,
    color: 'primary-variant',
    prependIcon: undefined,
    modelValue: () => [],
    items: () => [],
  },
);

const emit = defineEmits<{
  (event: 'update:modelValue', payload: T[]): void;
}>();

const modelValue = computed({
  get: () => props.modelValue,
  set: value => emit('update:modelValue', value),
});

const toggle = (key: T) => {
  if (props.modelValue.includes(key)) {
    modelValue.value = modelValue.value.filter(item => item !== key);
  } else {
    modelValue.value = [...modelValue.value, key];
  }
};
</script>
