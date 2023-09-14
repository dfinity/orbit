<template>
  <slot name="sidebar">
    <VNavigationDrawer class="sidebar" width="340" color="primary">
      <div class="sidebar__header">
        <slot name="sidebar-header"></slot>
      </div>
      <div class="sidebar__nav">
        <slot name="sidebar-nav"></slot>
      </div>
      <div class="sidebar__footer">
        <slot name="sidebar-footer">
          <a href="https://internetcomputer.org" target="_blank">
            <img src="/images/internet-computer-horizontal-dark.png" height="20" />
          </a>
        </slot>
      </div>
    </VNavigationDrawer>
  </slot>
  <slot name="body">
    <VMain class="body" full-height>
      <slot name="toolbar">
        <VToolbar density="compact" class="toolbar">
          <div class="toolbar__context">
            <slot name="toolbar-context"></slot>
          </div>
          <VSpacer />
          <div class="toolbar__actions">
            <slot name="toolbar-actions">
              <VBtn :icon="themeSwitcherIcon" @click.prevent="settings.toogleTheme" />
              <LanguageSelector />
            </slot>
          </div>
        </VToolbar>
      </slot>
      <nav class="topnav">
        <slot name="topnav"> </slot>
      </nav>
      <div class="main">
        <slot name="main">
          <header class="main__header">
            <slot name="main-header"></slot>
          </header>
          <div class="main__body">
            <slot name="main-body"></slot>
          </div>
        </slot>
      </div>
      <VFooter class="footer">
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
import { mdiWeatherNight, mdiWeatherSunny } from '@mdi/js';
import { computed } from 'vue';
import { useSettingsStore } from '~/ui/stores';
import LanguageSelector from './LanguageSelector.vue';

const settings = useSettingsStore();

const icLogoVertical = computed(() => {
  return settings.isDarkTheme
    ? '/images/internet-computer-vertical-dark.png'
    : '/images/internet-computer-vertical-light.png';
});

const ghMarkImg = computed(() => {
  return settings.isDarkTheme ? '/images/github-mark-dark.png' : '/images/github-mark-light.png';
});

const themeSwitcherIcon = computed(() => {
  return settings.isDarkTheme ? mdiWeatherNight : mdiWeatherSunny;
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
    background: transparent;

    &__actions {
      display: flex;
      flex-direction: row;
      align-items: end;
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
      border-top: var(--ds-border-primary-variant);

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
