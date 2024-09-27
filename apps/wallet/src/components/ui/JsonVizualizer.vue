<template>
  <VCard variant="outlined" density="compact" class="border-dashed">
    <slot name="header" :interactive="props.interactive">
      <VToolbar density="compact" class="background-effect">
        <VToolbarTitle class="text-body-2 font-weight-bold">{{ $t('terms.data') }}</VToolbarTitle>
        <VSpacer />
        <VBtn
          v-if="props.interactive"
          :icon="!removeUndefinedOrNull ? mdiEye : mdiEyeOff"
          size="small"
          density="comfortable"
          @click="removeUndefinedOrNull = !removeUndefinedOrNull"
        />
      </VToolbar>
      <VDivider />
    </slot>
    <VCardText class="px-4 py-2 background-effect">
      <pre class="text-body-2">{{ normalizedDataShown }}</pre>
    </VCardText>
    <VDivider v-if="hasMoreRows" />
    <VBtn v-if="addShowMoreButton" size="x-small" variant="tonal" block @click="showAll = true">
      {{ $t('terms.more') }}
    </VBtn>
    <VBtn
      v-else-if="addShowLessButton"
      size="x-small"
      variant="tonal"
      block
      @click="showAll = false"
    >
      {{ $t('terms.less') }}
    </VBtn>
  </VCard>
</template>
<script setup lang="ts">
import { mdiEye, mdiEyeOff } from '@mdi/js';
import { computed, ref, toRef } from 'vue';
import {
  VBtn,
  VCard,
  VCardText,
  VDivider,
  VSpacer,
  VToolbar,
  VToolbarTitle,
} from 'vuetify/components';
import { transformData } from '~/utils/helper.utils';

const props = withDefaults(
  defineProps<{
    data: unknown;
    rows?: number;
    interactive?: boolean;
  }>(),
  {
    rows: 6,
    interactive: false,
  },
);

const data = toRef(props, 'data');
const showAll = ref(false);
const removeUndefinedOrNull = ref(false);
const normalizedData = computed(() => {
  const transformedData = transformData(data.value, {
    removeFunctions: true,
    transformBufferAsHex: true,
    removeEmptyLists: false,
    removeUndefinedOrNull: removeUndefinedOrNull.value,
  });

  return JSON.stringify(transformedData, null, 2);
});

const normalizedDataRows = computed(() => normalizedData.value.split('\n').length);
const hasMoreRows = computed(() => normalizedDataRows.value > props.rows);

const addShowMoreButton = computed(() => !showAll.value && hasMoreRows.value);
const addShowLessButton = computed(() => showAll.value && hasMoreRows.value);

const normalizedDataShown = computed(() => {
  if (showAll.value) {
    return normalizedData.value;
  }

  return normalizedData.value.split('\n').slice(0, props.rows).join('\n');
});
</script>
<style scoped>
pre {
  white-space: pre-wrap;
}

.background-effect {
  background-color: rgb(var(--v-theme-neutral), 0.04) !important;
}
</style>
