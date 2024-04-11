<template>
  <VContainer class="pt-8 pb-0 pl-8 pr-8" fluid :data-test-id="$props.dataTestId" v-bind="$attrs">
    <VRow>
      <VCol v-if="breadcrumbs.length" cols="12" class="pa-0">
        <VBreadcrumbs :items="breadcrumbs" density="comfortable">
          <template #divider>
            <VIcon :icon="mdiChevronRight" size="small" />
          </template>
          <template #title="{ item }">
            <span class="text-body-2" :class="{ 'text-decoration-underline': item.to }">
              {{ item.title }}
            </span>
          </template>
        </VBreadcrumbs>
      </VCol>
      <VCol cols="12" :md="hasActions ? 6 : 12">
        <slot name="title-toolbar"></slot>
        <h1
          class="text-h4"
          :data-test-id="$props.dataTestId ? `${$props.dataTestId}-title` : undefined"
        >
          <slot name="title">{{ $props.title ?? '' }}</slot>
        </h1>
        <slot name="subtitle">{{ props.subtitle }}</slot>
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
    </VRow>
  </VContainer>
  <VDivider class="border-opacity-25 mt-4" :thickness="1" />
</template>

<script lang="ts" setup>
import { useSlots, computed } from 'vue';
import { mdiChevronRight } from '@mdi/js';
import { BreadCrumbItem } from '~/types/navigation.types';
import { VBreadcrumbs, VCol, VContainer, VDivider, VIcon, VRow } from 'vuetify/components';

const props = withDefaults(
  defineProps<{
    title?: string;
    subtitle?: string;
    dataTestId?: string;
    breadcrumbs?: BreadCrumbItem[];
  }>(),
  {
    title: undefined,
    subtitle: undefined,
    dataTestId: 'page-header',
    breadcrumbs: () => [],
  },
);

const slots = useSlots();

const hasActions = computed(() => !!slots.actions);
const breadcrumbs = computed(() =>
  props.breadcrumbs.map(item => ({
    ...item,
    disabled: item.disabled !== undefined ? item.disabled : false,
  })),
);
</script>
