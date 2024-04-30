<template>
  {{ props.label }}
  <VTooltip
    v-if="props.tooltip"
    v-model="tooltipOpen"
    location="bottom"
    :open-on-hover="false"
    :open-on-click="true"
    @click:outside="tooltipOpen = false"
  >
    <template #activator="{ props: infoProps }">
      <VBtn
        :icon="mdiInformationVariantCircleOutline"
        data-test-id="open-tooltip-button"
        density="compact"
        variant="text"
        size="small"
        v-bind="infoProps"
      />
    </template>
    {{ props.tooltip }}
  </VTooltip>
</template>
<script lang="ts" setup>
import { mdiInformationVariantCircleOutline } from '@mdi/js';
import { ref } from 'vue';

const props = withDefaults(
  defineProps<{
    label: string;
    tooltip?: string;
  }>(),
  {
    tooltip: undefined,
  },
);

const tooltipOpen = ref(false);
</script>
