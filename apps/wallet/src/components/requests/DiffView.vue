<template>
  <div class="d-flex flex-column flex-md-row position-relative">
    <VSheet
      v-if="showDiff"
      :class="showDiff ? 'w-md-50 pa-1 mb-1 mb-md-2 has-min-h' : 'flex-1-1'"
      :color="app.theme === SupportedTheme.Light ? '#ff808040' : '#ff808020'"
    >
      <slot :value="beforeValue" :diff-mode="'before'" :show-diff="showDiff"></slot>
    </VSheet>

    <div class="mobile-container">
      <VIcon
        v-if="showDiff"
        :icon="mobile ? mdiArrowDown : mdiArrowRight"
        :class="[
          'mb-md-2 align-self-center position-absolute position-md-static diff-view-icon',
          {
            'diff-view-icon-light': app.theme === SupportedTheme.Light,
            'diff-view-icon-dark': app.theme === SupportedTheme.Dark,
          },
        ]"
        :color="
          app.theme === SupportedTheme.Light
            ? mobile
              ? '#fff'
              : '#00000080'
            : mobile
              ? '#fff'
              : '#fff'
        "
      />

      <VSheet
        :class="showDiff ? 'w-md-50 pa-1 mb-4 mb-md-2 has-min-h' : 'flex-1-1'"
        :color="showDiff ? (app.theme === SupportedTheme.Light ? '#40b04035' : '#80ff8015') : ''"
      >
        <slot :value="afterValue" :diff-mode="'after'" :show-diff="showDiff"></slot>
      </VSheet>
    </div>
  </div>
</template>

<script setup lang="ts" generic="T">
import { mdiArrowDown, mdiArrowRight } from '@mdi/js';
import { computed } from 'vue';
import { useDisplay } from 'vuetify';
import { VIcon, VSheet } from 'vuetify/components';
import { useAppStore } from '~/stores/app.store';
import { SupportedTheme } from '~/types/app.types';

const { mobile } = useDisplay();

const app = useAppStore();

const props = defineProps<{
  beforeValue?: T;
  afterValue?: T;
  compareValues?: (before: T | undefined, after: T) => boolean;
  hasBefore?: boolean;
}>();

const showDiff = computed(() =>
  (props.beforeValue !== undefined || props.hasBefore) && props.afterValue !== undefined
    ? props.compareValues
      ? !props.compareValues(props.beforeValue, props.afterValue)
      : JSON.stringify(props.beforeValue) !== JSON.stringify(props.afterValue)
    : false,
);
</script>

<style scoped lang="scss">
@use '../../styles/variables' as *;

.diff-view-icon {
  top: -16px;
  right: 0;
  border-radius: 50%;
  padding: 4px;
  width: 32px;
  height: 32px;
}

.diff-view-icon-light {
  background-color: #00000080;
}

.diff-view-icon-dark {
  background-color: #ffffff40;
}
.has-min-h {
  min-height: 32px;
}
.mobile-container {
  position: relative;
}

@media (min-width: #{$device-md}) {
  .mobile-container {
    display: contents;
  }
  .diff-view-icon-light {
    background-color: transparent;
  }

  .diff-view-icon-dark {
    background-color: transparent;
  }
}
</style>
