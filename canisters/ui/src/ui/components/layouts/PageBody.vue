<template>
  <VContainer class="px-8 py-4" fluid>
    <VRow>
      <VCol cols="12" :data-test-id="$props.dataTestId">
        <slot></slot>
      </VCol>
    </VRow>
  </VContainer>
</template>
<script lang="ts" setup>
withDefaults(
  defineProps<{
    dataTestId?: string;
  }>(),
  {
    dataTestId: 'page-body',
  },
);
</script>
