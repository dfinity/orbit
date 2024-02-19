<template>
  <slot v-if="!isSetAndNotFalse(props.hideSidebar)" name="sidebar">
    <VNavigationDrawer v-model="app.showSidebar" class="sidebar" width="260" color="primary">
      <div class="sidebar__header">
        <slot name="sidebar-header">
          <SidenavHeader v-if="session.isAuthenticated" />
        </slot>
      </div>
      <div class="sidebar__nav">
        <slot name="sidebar-nav">
          <SidenavMenu v-if="session.isAuthenticated" />
        </slot>
      </div>
      <div class="px-4 py-4">
        <slot name="sidebar-footer">
          <a href="https://internetcomputer.org" target="_blank">
            <img :src="poweredByBadge" height="20" />
          </a>
        </slot>
      </div>
    </VNavigationDrawer>
  </slot>
  <slot v-if="!isSetAndNotFalse(props.hideBody)" name="body">
    <VMain class="body" full-height>
      <slot name="toolbar">
        <VToolbar density="compact" class="toolbar">
          <div
            v-if="!isSetAndNotFalse(props.hideToolbarContext)"
            class="toolbar__context d-flex align-center"
          >
            <slot name="toolbar-context">
              <BrandLogo v-if="!session.isAuthenticated" height="26px" class="ml-4" />
              <VBtn
                v-if="session.isAuthenticated && !isSetAndNotFalse(props.hideSidebar)"
                :icon="app.showSidebar ? mdiMenuOpen : mdiMenuClose"
                @click.prevent="app.toogleSidebar"
              />
            </slot>
          </div>
          <VSpacer />
          <div class="toolbar__actions mr-4">
            <slot name="toolbar-actions">
              <VBtn :icon="themeSwitcherIcon" @click.prevent="app.toogleTheme" />
              <LanguageSelector />
              <template v-if="session.isAuthenticated">
                <NotificationsPanelToggle />
                <UserAvatarSelector />
              </template>
            </slot>
          </div>
        </VToolbar>
        <div class="alpha-warning">
          <VIcon :icon="mdiAlertOutline" size="medium" />
          {{ $t('app.alpha_warning') }}
        </div>
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
      <VFooter
        v-if="!isSetAndNotFalse(props.hideFooter)"
        class="footer"
        :color="props.backgroundColor ? props.backgroundColor : `surface`"
      >
        <slot name="footer">
          <VContainer fluid>
            <VRow>
              <VCol class="footer__left text-left">
                <slot name="footer-left">
                  <slot name="footer-right">{{ $t('footer.copyright') }}</slot>
                </slot>
              </VCol>
              <VCol class="footer__right text-right">
                <a href="https://github.com/dfinity/orbit-wallet" target="_blank">
                  <img :src="ghMarkImg" class="footer__gh-mark" />{{
                    $t('footer.github.description')
                  }}
                </a>
              </VCol>
            </VRow>
          </VContainer>
        </slot>
      </VFooter>
    </VMain>
  </slot>
</template>

<script lang="ts" setup>
import {
  mdiAlertOutline,
  mdiMenuClose,
  mdiMenuOpen,
  mdiWeatherNight,
  mdiWeatherSunny,
} from '@mdi/js';
import { computed, inject } from 'vue';
import { isSetAndNotFalse } from '~/utils/helper.utils';
import BrandLogo from '~/components/BrandLogo.vue';
import NotificationsPanelToggle from '~/components/notifications/NotificationsPanelToggle.vue';
import SidenavHeader from '~/components/SidenavHeader.vue';
import SidenavMenu from '~/components/SidenavMenu.vue';
import UserAvatarSelector from '~/components/UserAvatarSelector.vue';
import { useAppStore } from '~/stores/app.store';
import { useSessionStore } from '~/stores/session.store';
import LanguageSelector from './LanguageSelector.vue';

const app = useAppStore();
const session = useSessionStore();

const props = inject('pageLayoutProps', {
  backgroundColor: undefined,
  hideSidebar: false,
  hideBody: false,
  hideMain: false,
  hideMainHeader: false,
  hideFooter: false,
  hideToolbarContext: false,
});

const ghMarkImg = computed(() => {
  return app.isDarkTheme ? '/images/github-mark-dark.png' : '/images/github-mark-light.png';
});

const poweredByBadge = `/images/powered-by-badge.svg`;

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
.page-layout--desktop {
  .sidebar {
    height: 100%;
    min-height: 100%;
    display: flex;
    flex-direction: column;

    &__header {
      width: 100%;
      flex-shrink: 0;
      min-height: var(--ds-toolbar-height);
    }

    &__nav {
      width: 100%;
      flex-grow: 1;
    }
  }

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

  .footer {
    height: var(--ds-toolbar-height);
    min-height: var(--ds-toolbar-height);
    max-height: var(--ds-toolbar-height);
    line-height: var(--ds-toolbar-height);
    font-size: var(--ds-font-size-xxs);
    background-color: transparent;
    box-sizing: content-box;

    .v-container {
      padding: 0;
      border-top: var(--ds-border-width) var(--ds-border-style) rgb(var(--ds-background-border));

      > .v-row {
        margin: 0;
      }
    }

    .v-col.footer__left,
    .v-col.footer__right {
      padding: 0;
      height: var(--ds-toolbar-height);
      min-height: var(--ds-toolbar-height);
      max-height: var(--ds-toolbar-height);
      line-height: var(--ds-toolbar-height);
    }

    &__gh-mark {
      height: var(--ds-font-size-xxs);
      width: var(--ds-font-size-xxs);
      vertical-align: middle;
      margin-right: calc(var(--ds-bdu) / 2);
    }
  }
}
</style>
