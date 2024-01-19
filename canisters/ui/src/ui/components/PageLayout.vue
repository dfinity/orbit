<template>
  <VLayout class="page-layout" :class="`${layoutDeviceClass} ${themeClass}`" full-height>
    <DesktopLayout v-if="!app.isMobile">
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
      v-model="app.notification.show"
      :absolute="true"
      :close-on-content-click="true"
      :color="app.notification.type"
      style="opacity: 0.9"
      variant="elevated"
      :location="notificationPosition"
      timeout="4000"
      transition="slide-x-reverse-transition"
    >
      {{ app.notification.message }}
      <template #actions>
        <v-btn variant="text" @click="app.notification.show = false">
          {{ $t('terms.close') }}
        </v-btn>
      </template>
    </VSnackbar>
  </VLayout>
</template>

<script lang="ts" setup>
import { computed, provide, watch } from 'vue';
import { useDisplay } from 'vuetify';
import { useAppStore } from '~/ui/stores/app';
import DesktopLayout from './DesktopLayout.vue';
import MobileLayout from './MobileLayout.vue';

const app = useAppStore();
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
  isMobile => app.setIsMobile(isMobile),
  { immediate: true },
);

const layoutDeviceClass = computed(() => {
  return app.isMobile ? 'page-layout--mobile' : 'page-layout--desktop';
});

const themeClass = computed(() => {
  return app.isDarkTheme ? 'theme--dark' : 'theme--light';
});

const notificationPosition = computed(() => {
  return app.isMobile ? 'bottom center' : 'top right';
});
</script>
