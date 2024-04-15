<template>
  <VLayout
    class="page-layout min-height-100"
    :class="{
      [layoutBaseClasses]: true,
      ['warning-banner--offset']: showWarningBanner,
    }"
  >
    <div
      v-if="showWarningBanner"
      class="warning-banner d-flex ga-1 flex-column justify-center flex-md-row text-body-2 font-weight-bold"
    >
      <div>
        {{ $t('app.test_environment_warning_banner.main') }}
      </div>
      <div>
        {{ $t('app.test_environment_warning_banner.info') }}
      </div>
    </div>
    <slot name="custom">
      <slot name="sidebar">
        <AppSidebar
          v-if="props.sidebar"
          :class="{
            ['warning-banner--offset']: showWarningBanner,
          }"
          :language-selector="mobile"
        />
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
import { appInitConfig } from '~/configs/init.config';

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

const layoutBaseClasses = computed(
  () => `${layoutDeviceClass.value} ${themeClass.value} ${props.backgroundColor}`,
);

const showWalletSelector = computed(
  () => session.isAuthenticated && session.hasWallets && mobile.value,
);

const showWarningBanner = ['playground', 'testing'].includes(appInitConfig.buildMode);
</script>

<style lang="scss">
@use '~/styles/variables.scss' as *;

.warning-banner--offset {
  padding-top: 48px;

  @media (min-width: #{$device-md}) {
    padding-top: 24px;
  }
}

.warning-banner {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  z-index: 9999;
  text-align: center;
  text-wrap: nowrap;
  background-color: rgb(var(--ds-primary));
  color: rgb(var(--ds-on-primary));
  padding: 4px 0;
}
</style>
