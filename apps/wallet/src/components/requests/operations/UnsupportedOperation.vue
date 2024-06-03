<template>
  <div v-if="isListMode" class="d-flex flex-column ga-0 text-caption">
    {{ $t(`requests.unsupported_operation`) }}
  </div>
  <VAlert type="warning" density="compact" variant="tonal">
    {{ $t(`requests.unsupported_operation`) }}
  </VAlert>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { VAlert } from 'vuetify/components';
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
