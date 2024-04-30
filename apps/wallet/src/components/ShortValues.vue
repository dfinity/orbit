<template>
  <span v-if="values.length">
    {{ values.join(', ') }}
    <VChip v-if="remaining" :size="props.size" density="comfortable"> +{{ remaining }} </VChip>
  </span>
  <span v-else>{{ props.empty }}</span>
</template>

<script lang="ts" setup>
import { computed } from 'vue';

const props = withDefaults(
  defineProps<{
    values: string[];
    max?: number;
    empty?: string;
    size?: 'x-small' | 'small' | 'medium' | 'large';
  }>(),
  {
    max: 3,
    empty: '',
    size: 'x-small',
  },
);

const values = computed(() => {
  return props.values.slice(0, props.max);
});

const remaining = computed(() => {
  return Math.max(0, props.values.length - props.max);
});
</script>
