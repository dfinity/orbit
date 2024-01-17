<template>
  <VContainer class="pt-8 pb-0 pl-8 pr-8" fluid>
    <VRow>
      <VCol cols="12" :md="hasActions ? 6 : 12">
        <h1 class="text-h4" data-testid="page-header-title">
          <slot name="title">{{ props.title ?? '' }}</slot>
        </h1>
      </VCol>
      <VCol
        v-if="hasActions"
        cols="12"
        md="6"
        class="d-flex justify-end"
        data-testid="page-header-actions"
      >
        <slot name="actions"></slot>
      </VCol>
      <VCol cols="12">
        <VDivider class="border-opacity-50 pb-2" :thickness="2" />
      </VCol>
    </VRow>
  </VContainer>
</template>

<script lang="ts" setup>
import { useSlots, computed } from 'vue';

const props = defineProps<{
  title?: string;
}>();

const slots = useSlots();

const hasActions = computed(() => !!slots.actions);
</script>
