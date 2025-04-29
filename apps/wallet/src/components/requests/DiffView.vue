<template>
  <div class="d-flex flex-column flex-md-row position-relative pr-6">
    <VSheet
      :class="showBefore ? 'w-md-50 pa-1 mb-1 mb-md-2' : 'flex-1-1'"
      v-if="showBefore"
      :color="'#ff808020'"
    >
      <slot :value="beforeValue" :mode="'before'"></slot>
    </VSheet>

    <VIcon
      v-if="showBefore"
      :icon="mobile ? mdiArrowDown : mdiArrowRight"
      class="mt-2 align-self-center text-grey-lighten-1 position-absolute position-md-static diff-view-icon"
    />

    <VSheet
      :class="showBefore ? 'w-md-50 pa-1 mb-4 mb-md-2' : 'flex-1-1'"
      :color="showBefore ? '#80ff8015' : ''"
    >
      <slot :value="afterValue" :mode="'after'"></slot>
    </VSheet>
  </div>
</template>

<style scoped>
.diff-view-icon {
  top: calc(50% - 24px);
  right: 0;
}
</style>

<script setup lang="ts" generic="T">
import { mdiArrowDown, mdiArrowRight } from '@mdi/js';
import { computed } from 'vue';
import { useDisplay } from 'vuetify';
import { VSheet } from 'vuetify/components';

const { mobile } = useDisplay();

const props = defineProps<{
  beforeValue?: T;
  afterValue?: T;
  compareValues?: (before: T, after: T) => boolean;
}>();

const showBefore = computed(() =>
  props.beforeValue && props.afterValue
    ? props.compareValues
      ? props.compareValues(props.beforeValue, props.afterValue)
      : JSON.stringify(props.beforeValue) !== JSON.stringify(props.afterValue)
    : false,
);
</script>
