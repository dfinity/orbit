<template>
  <VContainer class="px-4 py-4" fluid>
    <VRow>
      <VCol cols="12" :data-test-id="$props.dataTestId">
        <slot></slot>
      </VCol>
    </VRow>
  </VContainer>
</template>
<script lang="ts" setup>
import { VCol, VContainer, VRow } from 'vuetify/components';

withDefaults(
  defineProps<{
    dataTestId?: string;
  }>(),
  {
    dataTestId: 'page-body',
  },
);
</script>
