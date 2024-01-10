<template>
  <slot v-if="!isSetAndNotFalse(props.hideSidebar)" name="sidebar">
    <VNavigationDrawer
      v-if="session.isAuthenticated"
      v-model="app.showSidebar"
      class="sidebar"
      width="260"
      color="primary"
      temporary
    >
      <div class="sidebar__header">
        <slot name="sidebar-header">
          <SidenavHeader />
        </slot>
      </div>
      <div class="sidebar__nav">
        <slot name="sidebar-nav">
          <SidenavMenu />
        </slot>
      </div>
      <div class="sidebar__footer">
        <slot name="sidebar-footer">
          <a href="https://internetcomputer.org" target="_blank">
            <img :src="icLogoHorizontal" height="20" />
          </a>
        </slot>
      </div>
    </VNavigationDrawer>
  </slot>
  <slot v-if="!isSetAndNotFalse(props.hideBody)" name="body">
    <VMain class="body" full-height>
      <slot name="toolbar">
        <VToolbar density="compact" class="toolbar">
          <div v-if="!isSetAndNotFalse(props.hideToolbarContext)" class="toolbar__context">
            <slot name="toolbar-context">
              <BrandLogo />
            </slot>
          </div>
          <VSpacer />
          <div class="toolbar__actions">
            <slot name="toolbar-actions">
              <VBtn :icon="themeSwitcherIcon" @click.prevent="app.toogleTheme" />
              <NotificationsPanelToggle v-if="session.isAuthenticated" variant="outlined" />
              <UserAvatarSelector v-if="session.isAuthenticated" variant="outlined" />
              <LanguageSelector />
              <VBtn
                v-if="session.isAuthenticated"
                :icon="mdiMenuOpen"
                @click.prevent="app.toogleSidebar"
              />
            </slot>
          </div>
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

        <div class="alpha-warning">
          <VIcon :icon="mdiAlertOutline" size="medium" />
          {{ $t('app.alpha_warning') }}
        </div>
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
                <a href="https://internetcomputer.org" target="_blank" class="footer__ic">
                  <img :src="icLogoVertical" />
                </a>
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
import { mdiAlertOutline, mdiMenuOpen, mdiWeatherNight, mdiWeatherSunny } from '@mdi/js';
import { computed, inject } from 'vue';
import { isSetAndNotFalse } from '~/core';
import BrandLogo from '~/ui/components/BrandLogo.vue';
import NotificationsPanelToggle from '~/ui/components/NotificationsPanelToggle.vue';
import SidenavHeader from '~/ui/components/SidenavHeader.vue';
import SidenavMenu from '~/ui/components/SidenavMenu.vue';
import UserAvatarSelector from '~/ui/components/UserAvatarSelector.vue';
import WalletSelector from '~/ui/components/WalletSelector.vue';
import { useAppStore } from '~/ui/stores/app';
import { useSessionStore } from '~/ui/stores/session';
import LanguageSelector from './LanguageSelector.vue';
import icLogoHorizontal from '~/static/internet-computer-horizontal-light.png';

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

const icLogoVertical = computed(() => {
  return app.isDarkTheme
    ? '/images/internet-computer-vertical-dark.png'
    : '/images/internet-computer-vertical-light.png';
});

const ghMarkImg = computed(() => {
  return app.isDarkTheme ? '/images/github-mark-dark.png' : '/images/github-mark-light.png';
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
  .sidebar {
    height: 100%;
    min-height: 100%;
    display: flex;
    flex-direction: column;

    &__header {
      width: 100%;
      min-height: var(--ds-toolbar-height);
    }

    &__nav {
      width: 100%;
      flex-grow: 1;
    }

    &__footer {
      min-height: var(--ds-toolbar-height);
      text-align: center;
      justify-content: center;
      line-height: var(--ds-toolbar-height);
    }
  }

  .toolbar {
    display: flex;
    flex-direction: row;
    background-color: rgb(var(--ds-background));
    color: rgb(var(--ds-on-surface));

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
    display: flex;
    flex-direction: row;
    box-sizing: content-box;

    .v-container {
      padding: 0;
      margin: 0;
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
      display: flex;
      flex-grow: 1;
    }

    .v-col.footer__left {
      align-items: start;
      justify-content: start;
    }

    .v-col.footer__right {
      align-items: end;
      justify-content: end;
    }

    &__ic {
      margin-right: calc(var(--ds-bdu));
      align-items: center;
      display: flex;
      height: 100%;
      padding-right: var(--ds-bdu);

      img {
        height: calc(var(--ds-toolbar-height) / 2);
      }
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
