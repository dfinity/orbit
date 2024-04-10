<template>
  <slot name="sidebar">
    <AppSidebar v-if="props.sidebar" />
  </slot>
  <slot v-if="!isSetAndNotFalse(props.hideBody)" name="body">
    <VMain class="body" full-height>
      <slot name="toolbar">
        <VToolbar density="compact" class="toolbar">
          <slot name="toolbar-actions">
            <AppToolbar variant="outlined" expandable-sidebar />
            <!-- <VBtn :icon="themeSwitcherIcon" @click.prevent="app.toogleTheme" />
              <NotificationsPanelToggle v-if="session.isAuthenticated" variant="outlined" />
              <UserAvatarSelector v-if="session.isAuthenticated" variant="outlined" />
              <LanguageSelector />
              <VBtn
                v-if="session.isAuthenticated && !isSetAndNotFalse(props.hideSidebar)"
                :icon="mdiMenuOpen"
                @click.prevent="app.toogleSidebar"
              /> -->
          </slot>
        </VToolbar>
      </slot>
      <nav
        class="topnav"
        :style="
          props.backgroundColor
            ? `background-color: rgb(var(--ds-${props.backgroundColor}));`
            : undefined
        "
      >
        <slot name="topnav">
          <WalletSelector v-if="session.isAuthenticated" />
        </slot>
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
import { mdiMenuOpen, mdiWeatherNight, mdiWeatherSunny } from '@mdi/js';
import { computed, inject } from 'vue';
import BrandLogo from '~/components/BrandLogo.vue';
import AppSidebar from '~/components/layouts/AppSidebar.vue';
import NotificationsPanelToggle from '~/components/notifications/NotificationsPanelToggle.vue';
import UserAvatarSelector from '~/components/UserAvatarSelector.vue';
import WalletSelector from '~/components/WalletSelector.vue';
import { useAppStore } from '~/stores/app.store';
import { useSessionStore } from '~/stores/session.store';
import { isSetAndNotFalse } from '~/utils/helper.utils';
import LanguageSelector from './LanguageSelector.vue';
import AppToolbar from '~/components/layouts/AppToolbar.vue';

const app = useAppStore();
const session = useSessionStore();

const props = inject('pageLayoutProps', {
  backgroundColor: undefined,
  sidebar: true,
  hideBody: false,
  hideMain: false,
  hideMainHeader: false,
  hideToolbarContext: false,
});

const themeSwitcherIcon = computed(() => {
  return app.isDarkTheme ? mdiWeatherNight : mdiWeatherSunny;
});
</script>

<style lang="scss">
.sidebar {
  .v-navigation-drawer__content {
    display: flex;
    flex-direction: column;
  }
}
</style>

<style scoped lang="scss">
.page-layout--mobile {
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
