<template>
  <slot name="sidebar">
    <AppSidebar v-if="props.sidebar" />
  </slot>
  <VMain class="body" full-height>
    <slot name="toolbar">
      <AppToolbar v-if="props.toolbar" :bg-color="props.surfaceColor" />
    </slot>
    <div v-if="props.contextbar" :class="`contextbar d-flex ${props.surfaceColor}`">
      <slot name="contextbar"></slot>
    </div>
    <div v-if="props.main" class="main">
      <slot name="main">
        <header v-if="props.mainHeader" :class="`main__header ${props.surfaceColor}`">
          <slot name="main-header"></slot>
        </header>
        <div v-if="props.mainBody" class="main__body">
          <slot name="main-body"></slot>
        </div>
      </slot>
    </div>
  </VMain>
</template>

<script lang="ts" setup>
import { inject } from 'vue';
import { VMain } from 'vuetify/components';
import AppSidebar from '~/components/layouts/AppSidebar.vue';
import AppToolbar from '~/components/layouts/AppToolbar.vue';

const props = inject('pageLayoutProps', {
  backgroundColor: 'bg-background',
  surfaceColor: 'bg-surface',
  sidebar: true,
  toolbar: true,
  contextbar: true,
  main: true,
  mainHeader: true,
  mainBody: true,
});
</script>
