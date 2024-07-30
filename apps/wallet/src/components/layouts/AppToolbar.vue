<template>
  <VToolbar density="compact" :class="`py-2 px-4 ${bgColor}`">
    <div class="d-flex">
      <slot name="context">
        <BrandLogo v-if="showLogo" height="26px" />
      </slot>
    </div>
    <VSpacer />
    <div class="d-flex ga-0 ga-md-1 align-center">
      <slot name="actions">
        <VBtn
          v-if="props.themeSelector"
          :icon="app.isDarkTheme ? mdiWeatherNight : mdiWeatherSunny"
          @click.prevent="app.toogleTheme"
        />
        <LanguageSelector v-if="props.languageSelector" />
        <NotificationsPanelToggle v-if="session.isAuthenticated" :variant="props.variant" />
        <UserAvatarSelector v-if="session.isAuthenticated" :variant="props.variant" />
        <ChangeCanisterActionBtn v-if="showUpdateBtn" mode="highlight" class="mr-1" />
        <VBtn
          v-if="props.expandableSidebar"
          density="compact"
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
import { useStationStore } from '~/stores/station.store';
import ChangeCanisterActionBtn from '../change-canister/ChangeCanisterActionBtn.vue';
import { variantIs } from '~/utils/helper.utils';

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
const station = useStationStore();

const showLogo = computed(() => {
  if (props.logo !== undefined) {
    return props.logo;
  }

  return app.isMobile;
});

const bgColor = computed(() => (props.bgColor !== undefined ? props.bgColor : ''));
const showUpdateBtn = computed(
  () =>
    session.isAuthenticated &&
    station.hasNewVersion &&
    !station.versionManagement.updateRequested &&
    station.privileges.some(p => variantIs(p, 'ChangeCanister')),
);
</script>
