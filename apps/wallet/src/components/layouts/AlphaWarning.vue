<template>
  <div class="alpha-warning d-flex font-weight-bold" :style="{ 'max-width': props.maxWidth }">
    <div class="flex-grow-1">
      <div class="text-left">{{ $t('alpha_warning.version') }}</div>
      <div class="text-left">{{ $t('alpha_warning.caution') }}</div>
    </div>
    <div class="flex-grow-0 d-flex align-center">
      <VIcon :icon="mdiAlertOutline" size="small" />
    </div>
  </div>
</template>

<script lang="ts" setup>
import { mdiAlertOutline } from '@mdi/js';
import { VIcon } from 'vuetify/components';

const props = withDefaults(
  defineProps<{
    maxWidth?: string;
  }>(),
  {
    maxWidth: '300px',
  },
);
</script>
