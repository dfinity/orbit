<template>
  <VTooltip :text="text" location="bottom">
    <template #activator="{ props: activatorProps }">
      <VIcon class="ml-2" v-bind="activatorProps" :icon="icon" />
    </template>
  </VTooltip>
</template>

<script setup lang="ts">
import { mdiAlertCircleOutline } from '@mdi/js';
import { VIcon } from 'vuetify/components';

withDefaults(
  defineProps<{
    text: string;
    icon?: string;
  }>(),
  {
    icon: mdiAlertCircleOutline,
  },
);
</script>
