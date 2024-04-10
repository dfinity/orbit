<template>
  <VToolbar density="compact" :class="toolbarClasses">
    <div class="d-flex">
      <slot name="context">
        <BrandLogo v-if="showLogo" height="26px" class="ml-4" />
        <VBtn
          v-if="session.isAuthenticated && !props.sidebar"
          :icon="app.showSidebar ? mdiMenuOpen : mdiMenuClose"
          @click.prevent="app.toogleSidebar"
        />
      </slot>
    </div>
    <VSpacer />
    <div class="d-flex ga-1 mr-4">
      <slot name="actions">
        <VBtn
          v-if="props.themeSelector"
          :icon="app.isDarkTheme ? mdiWeatherNight : mdiWeatherSunny"
          @click.prevent="app.toogleTheme"
        />
        <LanguageSelector :bg-color="props.bgColor" />
        <NotificationsPanelToggle v-if="session.isAuthenticated" />
        <UserAvatarSelector v-if="session.isAuthenticated" />
      </slot>
    </div>
  </VToolbar>
</template>

<script lang="ts" setup>
import { mdiMenuClose, mdiMenuOpen, mdiWeatherNight, mdiWeatherSunny } from '@mdi/js';
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
    sidebar?: boolean;
    bgColor?: string;
    themeSelector?: boolean;
  }>(),
  {
    logo: undefined,
    sidebar: true,
    themeSelector: true,
    bgColor: 'surface',
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
const toolbarClasses = computed(() => {
  if (props.bgColor) {
    return {
      [`bg-${props.bgColor}`]: true,
      'py-2': true,
    };
  }

  return {};
});
</script>
