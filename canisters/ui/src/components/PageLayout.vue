<template>
  <VLayout
    class="page-layout min-height-100"
    :class="`${layoutDeviceClass} ${themeClass} ${backgroundColor}`"
  >
    <slot name="custom">
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
    </slot>
    <VSnackbar
      v-model="app.notification.show"
      :absolute="true"
      :close-on-content-click="true"
      :color="app.notification.type"
      style="opacity: 0.9"
      variant="elevated"
      location="bottom center"
      timeout="4000"
      transition="slide-x-reverse-transition"
    >
      {{ app.notification.message }}
      <template #actions>
        <VBtn variant="text" @click="app.notification.show = false">
          {{ $t('terms.close') }}
        </VBtn>
      </template>
    </VSnackbar>
    <SessionExpiredOverlay />
    <OpenProposalOverlay v-if="session.isAuthenticated && session.data.selectedWallet.hasAccess" />
  </VLayout>
</template>

<script lang="ts" setup>
import { computed, provide, watch } from 'vue';
import { useDisplay } from 'vuetify';
import { useAppStore } from '~/stores/app.store';
import DesktopLayout from './DesktopLayout.vue';
import MobileLayout from './MobileLayout.vue';
import SessionExpiredOverlay from './SessionExpiredOverlay.vue';
import OpenProposalOverlay from '~/components/proposals/OpenProposalOverlay.vue';
import { useSessionStore } from '~/stores/session.store';
import { VBtn, VLayout, VSnackbar } from 'vuetify/components';

const app = useAppStore();
const session = useSessionStore();

const slotNames = [
  'sidebar',
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
    hideToolbarContext?: string | boolean;
  }>(),
  {
    backgroundColor: undefined,
    hideSidebar: false,
    hideBody: false,
    hideMain: false,
    hideMainHeader: false,
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

const backgroundColor = computed(() => {
  if (props.backgroundColor === 'transparent') {
    return '';
  }

  if (props.backgroundColor !== undefined) {
    return `bg-${props.backgroundColor}`;
  }

  return 'bg-background';
});

const themeClass = computed(() => {
  return app.isDarkTheme ? 'theme--dark' : 'theme--light';
});
</script>
