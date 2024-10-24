<template>
  <div v-bind="$attrs" class="d-flex flex-column ga-0 flex-grow-1">
    <div class="d-flex flex-nowrap">
      <VBtnToggle v-model="selectedItemIdx" rounded="0" group density="compact">
        <VBtn
          v-for="(item, idx) in props.items"
          :key="idx"
          :value="idx"
          variant="tonal"
          size="small"
        >
          {{ item.title }}
        </VBtn>
      </VBtnToggle>
    </div>
    <VTextarea
      v-if="selectedItem"
      v-model="selectedItem.content"
      readonly
      hide-details
      :density="props.density"
      :variant="props.variant"
      :rows="2"
      no-resize
      class="mt-0"
    >
      <template #append-inner>
        <VBtn
          size="x-small"
          variant="text"
          :icon="mdiContentCopy"
          @click="
            copyToClipboard({
              textToCopy: selectedItem.content,
              sendNotification: true,
            })
          "
        />
      </template>
    </VTextarea>
  </div>
</template>

<script setup lang="ts">
import { mdiContentCopy } from '@mdi/js';
import { computed, ref } from 'vue';
import { VBtn, VBtnToggle, VTextarea } from 'vuetify/components';
import { copyToClipboard } from '~/utils/app.utils';

const props = withDefaults(
  defineProps<{
    items: { title: string; content: string }[];
    density?: 'comfortable' | 'compact' | 'default';
    variant?: 'filled' | 'outlined' | 'plain' | 'solo' | 'underlined';
  }>(),
  {
    items: () => [],
    density: 'comfortable',
    variant: 'filled',
  },
);

const selectedItemIdx = ref<number>(0);
const selectedItem = computed(() => props.items?.[selectedItemIdx.value]);
</script>
