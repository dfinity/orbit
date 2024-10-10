<template>
  <JsonVizualizer :data="operation" :interactive="!isListMode" :rows="isListMode ? 6 : 12" />
</template>

<script setup lang="ts">
import { computed } from 'vue';
import JsonVizualizer from '~/components/ui/JsonVizualizer.vue';
import { Request } from '~/generated/station/station.did';

const props = withDefaults(
  defineProps<{
    request: Request;
    operation: unknown;
    mode?: 'list' | 'detail';
  }>(),
  {
    mode: 'list',
  },
);

const isListMode = computed(() => props.mode === 'list');
</script>
