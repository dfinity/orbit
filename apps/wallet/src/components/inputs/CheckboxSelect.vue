<template>
  <div class="d-flex flex-column">
    <label v-if="props.label" class="d-flex ga-1 align-center">
      <VIcon v-if="props.prependIcon" :icon="props.prependIcon" size="small" />
      {{ props.label }}
    </label>
    <div class="d-flex flex-column mt-2 flex-wrap">
      <VCheckbox
        v-for="(item, idx) in props.items"
        :key="idx"
        :model-value="modelValue.includes(item.key)"
        :label="item.text"
        hide-details
        class="d-flex checkbox-compact-small"
        density="compact"
        @update:model-value="toggle(item.key)"
      />
    </div>
  </div>
</template>

<script setup lang="ts" generic="T">
import { computed } from 'vue';
import { VCheckbox, VIcon } from 'vuetify/components';

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
<style lang="scss">
.checkbox-compact-small {
  --v-input-control-height: 32px !important;

  .v-selection-control {
    --v-selection-control-size: 22px !important;
  }

  .v-label {
    margin-left: 4px;
  }
}
</style>
