<template>
  <VToolbar density="compact" :class="`py-2 ${bgColor}`">
    <div class="d-flex">
      <slot name="context">
        <BrandLogo v-if="showLogo" height="26px" class="ml-4" />
      </slot>
    </div>
    <VSpacer />
    <div
      class="d-flex"
      :class="{
        'ga-0': app.isMobile,
        'ga-2': !app.isMobile,
        'mr-2': !app.isMobile || !session.isAuthenticated,
      }"
    >
      <slot name="actions">
        <VBtn
          v-if="props.themeSelector"
          :icon="app.isDarkTheme ? mdiWeatherNight : mdiWeatherSunny"
          @click.prevent="app.toogleTheme"
        />
        <LanguageSelector v-if="props.languageSelector" />
        <NotificationsPanelToggle v-if="session.isAuthenticated" :variant="props.variant" />
        <UserAvatarSelector v-if="session.isAuthenticated" :variant="props.variant" />
        <VBtn
          v-if="props.expandableSidebar"
          :icon="mdiMenuOpen"
          @click.prevent="app.toogleSidebar"
        />
      </slot>
    </div>
  </VToolbar>
</template>

<script lang="ts" setup>
import { mdiMenuOpen, mdiWeatherNight, mdiWeatherSunny } from '@mdi/js';
import { computed } from 'vue';
import { VBtn, VSpacer, VToolbar } from 'vuetify/components';
import BrandLogo from '~/components/BrandLogo.vue';
import LanguageSelector from '~/components/LanguageSelector.vue';
import NotificationsPanelToggle from '~/components/notifications/NotificationsPanelToggle.vue';
import UserAvatarSelector from '~/components/UserAvatarSelector.vue';
import { useAppStore } from '~/stores/app.store';
import { useSessionStore } from '~/stores/session.store';

const props = withDefaults(
  defineProps<{
    logo?: boolean;
    languageSelector?: boolean;
    sidebar?: boolean;
    bgColor?: string;
    themeSelector?: boolean;
    variant?: 'outlined' | 'filled';
    expandableSidebar?: boolean;
  }>(),
  {
    logo: undefined,
    languageSelector: true,
    sidebar: true,
    themeSelector: true,
    bgColor: 'bg-surface',
    variant: 'filled',
    expandableSidebar: false,
  },
);

const session = useSessionStore();
const app = useAppStore();

const showLogo = computed(() => {
  if (props.logo !== undefined) {
    return props.logo;
  }

  return app.isMobile;
});

const bgColor = computed(() => (props.bgColor !== undefined ? props.bgColor : ''));
</script>
