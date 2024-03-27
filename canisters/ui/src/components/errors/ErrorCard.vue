<template>
  <VCard>
    <slot name="title" :title="props.title">
      <VToolbar v-if="props.title" color="background">
        <VToolbarTitle>
          {{ props.title }}
        </VToolbarTitle>
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
    title?: string;
    error: string;
    errorDetails?: string;
  }>(),
  {
    title: undefined,
    error: '',
    errorDetails: undefined,
  },
);
</script>
