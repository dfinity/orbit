<template>
  <VLayout
    class="page-layout min-height-100"
    :class="`${layoutDeviceClass} ${themeClass} ${props.backgroundColor}`"
  >
    <slot name="custom">
      <slot name="sidebar">
        <AppSidebar v-if="props.sidebar" :language-selector="mobile" />
      </slot>
      <VMain class="body d-flex flex-column" full-height>
        <slot name="toolbar">
          <AppToolbar
            v-if="props.toolbar"
            :expandable-sidebar="mobile"
            :language-selector="!mobile"
            :bg-color="props.surfaceColor"
          />
        </slot>
        <div v-if="props.contextbar" :class="`contextbar d-flex ${props.surfaceColor}`">
          <slot name="contextbar">
            <WalletSelector v-if="showWalletSelector" />
          </slot>
        </div>
        <div v-if="props.main" class="main d-flex flex-column flex-grow-1">
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
    </slot>
    <VSnackbar
      v-model="app.notification.show"
      :absolute="true"
      :close-on-content-click="true"
      :color="app.notification.type"
      style="opacity: 0.9"
      variant="elevated"
      location="bottom center"
      timeout="4000"
      transition="slide-x-reverse-transition"
    >
      {{ app.notification.message }}
      <template #actions>
        <VBtn variant="text" @click="app.notification.show = false">
          {{ $t('terms.close') }}
        </VBtn>
      </template>
    </VSnackbar>
    <SessionExpiredOverlay />
    <OpenProposalOverlay v-if="session.isAuthenticated && session.data.selectedWallet.hasAccess" />
  </VLayout>
</template>

<script lang="ts" setup>
import { computed, watch } from 'vue';
import { useDisplay } from 'vuetify';
import { VBtn, VLayout, VMain, VSnackbar } from 'vuetify/components';
import AppSidebar from '~/components/layouts/AppSidebar.vue';
import AppToolbar from '~/components/layouts/AppToolbar.vue';
import OpenProposalOverlay from '~/components/proposals/OpenProposalOverlay.vue';
import { useAppStore } from '~/stores/app.store';
import { useSessionStore } from '~/stores/session.store';
import SessionExpiredOverlay from './SessionExpiredOverlay.vue';
import WalletSelector from '~/components/WalletSelector.vue';

const app = useAppStore();
const session = useSessionStore();

const props = withDefaults(
  defineProps<{
    backgroundColor?: string;
    surfaceColor?: string;
    toolbar?: boolean;
    sidebar?: boolean;
    contextbar?: boolean;
    main?: boolean;
    mainHeader?: boolean;
    mainBody?: boolean;
  }>(),
  {
    backgroundColor: 'bg-background',
    surfaceColor: 'bg-surface',
    toolbar: true,
    sidebar: true,
    contextbar: true,
    main: true,
    mainHeader: true,
    mainBody: true,
  },
);

const { mobile } = useDisplay();

watch(
  () => mobile.value,
  isMobile => app.setIsMobile(isMobile),
  { immediate: true },
);

const layoutDeviceClass = computed(() =>
  app.isMobile ? 'page-layout--mobile' : 'page-layout--desktop',
);

const themeClass = computed(() => (app.isDarkTheme ? 'theme--dark' : 'theme--light'));
const showWalletSelector = computed(
  () => session.isAuthenticated && session.hasWallets && mobile.value,
);
</script>
