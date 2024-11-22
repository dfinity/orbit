<template>
  <VCard>
    <slot name="title" :title="props.title">
      <VToolbar v-if="props.title" color="background">
        <VToolbarTitle>
          {{ props.title }}
        </VToolbarTitle>
        <VBtn :icon="mdiClose" @click="emit('close')" />
      </VToolbar>
      <VDivider />
    </slot>
    <VCardText>
      <slot name="error" :error="props.error" :details="props.errorDetails">
        <VAlert variant="text" density="comfortable" type="error">{{ props.error }}</VAlert>
        <template v-if="props.errorDetails">
          <VDivider class="mt-4 mb-2" />
          <VExpansionPanels flat variant="accordion">
            <VExpansionPanel
              class="text-body-2"
              data-test-id="error-details-panel"
              :title="$t('terms.details')"
              :text="props.errorDetails"
            />
          </VExpansionPanels>
        </template>
      </slot>
    </VCardText>
  </VCard>
</template>

<script lang="ts" setup>
import { mdiClose } from '@mdi/js';
import {
  VAlert,
  VCard,
  VCardText,
  VDivider,
  VExpansionPanel,
  VExpansionPanels,
  VToolbar,
  VToolbarTitle,
} from 'vuetify/components';

const props = withDefaults(
  defineProps<{
    error: string;
    title?: string;
    errorDetails?: string;
    open?: boolean;
  }>(),
  {
    title: undefined,
    errorDetails: undefined,
    open: true,
  },
);

const emit = defineEmits<{
  (event: 'close'): void;
}>();
</script>
