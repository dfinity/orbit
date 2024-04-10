<template>
  <slot name="sidebar">
    <AppSidebar v-if="props.sidebar" />
  </slot>
  <slot v-if="!isSetAndNotFalse(props.hideBody)" name="body">
    <VMain class="body" full-height>
      <slot name="toolbar">
        <AppToolbar />
      </slot>
      <nav
        class="topnav"
        :style="
          props.backgroundColor
            ? `background-color: rgb(var(--ds-${props.backgroundColor}));`
            : undefined
        "
      >
        <slot name="topnav"></slot>
      </nav>
      <div v-if="!isSetAndNotFalse(props.hideMain)" class="main">
        <slot name="main">
          <header v-if="!isSetAndNotFalse(props.hideMainHeader)" class="main__header">
            <slot name="main-header"></slot>
          </header>
          <div
            class="main__body"
            :style="
              props.backgroundColor
                ? `background-color: rgb(var(--ds-${props.backgroundColor}));`
                : undefined
            "
          >
            <slot name="main-body"></slot>
          </div>
        </slot>
      </div>
    </VMain>
  </slot>
</template>

<script lang="ts" setup>
import { inject } from 'vue';
import AppSidebar from '~/components/layouts/AppSidebar.vue';
import AppToolbar from '~/components/layouts/AppToolbar.vue';
import { isSetAndNotFalse } from '~/utils/helper.utils';

const props = inject('pageLayoutProps', {
  backgroundColor: undefined,
  sidebar: true,
  hideBody: false,
  hideMain: false,
  hideMainHeader: false,
  hideToolbarContext: false,
});
</script>

<style scoped lang="scss">
.page-layout--desktop {
  .toolbar {
    display: flex;
    flex-direction: row;
    background-color: rgb(var(--ds-surface));
    color: rgb(var(--ds-on-surface));
    border-bottom: var(--ds-border-width) var(--ds-border-style) rgb(var(--ds-background-border));

    &__actions {
      display: flex;
      flex-direction: row;
      align-items: center;
      justify-content: end;
    }
  }

  .topnav {
    display: flex;
    flex-direction: column;
    height: auto;
  }

  .body {
    width: 100%;
    height: 100%;
    flex-grow: 1;
    display: flex;
    flex-direction: column;
  }

  .main {
    width: 100%;
    display: flex;
    flex-grow: 1;
    flex-direction: column;
    align-items: start;
    justify-content: start;

    &__header {
      width: 100%;
      background-color: rgb(var(--ds-surface));
      color: rgb(var(--ds-on-surface));
    }

    &__body {
      width: 100%;
      flex-grow: 1;
    }
  }
}
</style>
