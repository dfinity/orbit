<template>
  <VContainer class="pt-8 pb-0 pl-8 pr-8" fluid :data-test-id="$props.dataTestId">
    <VRow>
      <VCol cols="12" :md="hasActions ? 6 : 12">
        <h1
          class="text-h4"
          :data-test-id="$props.dataTestId ? `${$props.dataTestId}-title` : undefined"
        >
          <slot name="title">{{ $props.title ?? '' }}</slot>
        </h1>
      </VCol>
      <VCol
        v-if="hasActions"
        cols="12"
        md="6"
        class="d-flex justify-end flex-column flex-md-row ga-2"
        :data-test-id="$props.dataTestId ? `${$props.dataTestId}-actions` : undefined"
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

withDefaults(
  defineProps<{
    title?: string;
    dataTestId?: string;
  }>(),
  {
    title: undefined,
    dataTestId: 'page-header',
  },
);

const slots = useSlots();

const hasActions = computed(() => !!slots.actions);
</script>
