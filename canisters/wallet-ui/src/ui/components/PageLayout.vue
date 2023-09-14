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
  </VLayout>
</template>

<script lang="ts" setup>
import { useSettingsStore } from '~/ui/stores';
import DesktopLayout from './DesktopLayout.vue';
import MobileLayout from './MobileLayout.vue';
import { computed } from 'vue';

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

const layoutDeviceClass = computed(() => {
  return settings.isMobile ? 'page-layout--mobile' : 'page-layout--desktop';
});

const themeClass = computed(() => {
  return settings.isDarkTheme ? 'theme--dark' : 'theme--light';
});
</script>
