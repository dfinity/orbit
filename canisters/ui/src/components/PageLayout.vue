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
import { VBtn, VLayout, VSnackbar } from 'vuetify/components';
import DesktopLayout from '~/components/layouts/DesktopLayout.vue';
import MobileLayout from '~/components/layouts/MobileLayout.vue';
import OpenProposalOverlay from '~/components/proposals/OpenProposalOverlay.vue';
import { useAppStore } from '~/stores/app.store';
import { useSessionStore } from '~/stores/session.store';
import SessionExpiredOverlay from './SessionExpiredOverlay.vue';

const app = useAppStore();
const session = useSessionStore();

const slotNames = ['sidebar', 'toolbar', 'contextbar', 'main', 'main-header', 'main-body'];

const props = withDefaults(
  defineProps<{
    backgroundColor?: string;
    surfaceColor?: string;
    toolbar?: boolean;
    sidebar?: boolean;
    contextbar?: boolean;
    main?: boolean;
    mainHeader?: boolean;
    mainBody?: boolean;
  }>(),
  {
    backgroundColor: 'bg-background',
    surfaceColor: 'bg-surface',
    toolbar: true,
    sidebar: true,
    contextbar: true,
    main: true,
    mainHeader: true,
    mainBody: true,
  },
);

const backgroundColor = computed(() => {
  if (props.backgroundColor !== undefined) {
    return `${props.backgroundColor}`;
  }

  return 'bg-background';
});

const surfaceColor = computed(() => {
  if (props.surfaceColor !== undefined) {
    return `${props.surfaceColor}`;
  }

  return 'bg-surface';
});

provide('pageLayoutProps', {
  ...props,
  backgroundColor: backgroundColor.value,
  surfaceColor: surfaceColor.value,
});

const { mobile } = useDisplay();

watch(
  () => mobile.value,
  isMobile => app.setIsMobile(isMobile),
  { immediate: true },
);

const layoutDeviceClass = computed(() =>
  app.isMobile ? 'page-layout--mobile' : 'page-layout--desktop',
);

const themeClass = computed(() => (app.isDarkTheme ? 'theme--dark' : 'theme--light'));
</script>
