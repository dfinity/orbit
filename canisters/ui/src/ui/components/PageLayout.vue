<template>
  <VLayout class="page-layout" :class="`${layoutDeviceClass} ${themeClass}`" full-height>
    <DesktopLayout v-if="!settings.isMobile">
      <template v-for="slotName in slotNames" :key="slotName" #[slotName]>
        <slot :name="slotName"></slot>
      </template>
    </DesktopLayout>
    <MobileLayout v-else>
      <template v-for="slotName in slotNames" :key="slotName" #[slotName]>
        <slot :name="slotName"></slot>
      </template>
    </MobileLayout>
    <VSnackbar
      v-model="settings.notification.show"
      :absolute="true"
      :close-on-content-click="true"
      :color="settings.notification.type"
      style="opacity: 0.9"
      variant="elevated"
      :location="notificationPosition"
      timeout="4000"
      transition="slide-x-reverse-transition"
    >
      {{ settings.notification.message }}
      <template #actions>
        <v-btn variant="text" @click="settings.notification.show = false">
          {{ $t('terms.close') }}
        </v-btn>
      </template>
    </VSnackbar>
  </VLayout>
</template>

<script lang="ts" setup>
import { computed, provide, watch } from 'vue';
import { useDisplay } from 'vuetify';
import { useSettingsStore } from '~/ui/stores';
import DesktopLayout from './DesktopLayout.vue';
import MobileLayout from './MobileLayout.vue';

const settings = useSettingsStore();
const slotNames = [
  'sidebar',
  'sidebar-header',
  'sidebar-nav',
  'sidebar-footer',
  'body',
  'toolbar',
  'toolbar-context',
  'toolbar-actions',
  'topnav',
  'main',
  'main-header',
  'main-body',
];

const props = withDefaults(
  defineProps<{
    backgroundColor?: string;
    hideSidebar?: string | boolean;
    hideBody?: string | boolean;
    hideMain?: string | boolean;
    hideMainHeader?: string | boolean;
    hideFooter?: string | boolean;
    hideToolbarContext?: string | boolean;
  }>(),
  {
    backgroundColor: undefined,
    hideSidebar: false,
    hideBody: false,
    hideMain: false,
    hideMainHeader: false,
    hideFooter: false,
    hideToolbarContext: false,
  },
);

provide('pageLayoutProps', props);

const { mobile } = useDisplay();

watch(
  () => mobile.value,
  isMobile => {
    settings.showSidebar = !isMobile;
  },
  { immediate: true },
);

const layoutDeviceClass = computed(() => {
  return settings.isMobile ? 'page-layout--mobile' : 'page-layout--desktop';
});

const themeClass = computed(() => {
  return settings.isDarkTheme ? 'theme--dark' : 'theme--light';
});

const notificationPosition = computed(() => {
  return settings.isMobile ? 'bottom center' : 'top right';
});
</script>
