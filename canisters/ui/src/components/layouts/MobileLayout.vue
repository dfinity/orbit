<template>
  <slot name="sidebar">
    <AppSidebar v-if="props.sidebar" :language-selector="true" />
  </slot>
  <VMain class="body" full-height>
    <slot name="toolbar">
      <AppToolbar
        v-if="props.toolbar"
        expandable-sidebar
        :language-selector="false"
        :bg-color="props.surfaceColor"
      />
    </slot>
    <div v-if="props.contextbar" :class="`contextbar d-flex ${props.surfaceColor}`">
      <slot name="contextbar">
        <WalletSelector v-if="showWalletSelector" />
      </slot>
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
import { computed, inject } from 'vue';
import { VMain } from 'vuetify/components';
import AppSidebar from '~/components/layouts/AppSidebar.vue';
import AppToolbar from '~/components/layouts/AppToolbar.vue';
import WalletSelector from '~/components/WalletSelector.vue';
import { useSessionStore } from '~/stores/session.store';

const session = useSessionStore();

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

const showWalletSelector = computed(() => session.isAuthenticated && session.hasWallets);
</script>
