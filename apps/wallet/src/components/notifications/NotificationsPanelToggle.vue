<template>
  <VMenu
    v-if="!app.isMobile"
    v-model="notificationsPopup"
    location="bottom end"
    :close-on-content-click="false"
  >
    <template #activator="{ props: selectorProps }">
      <VBtn v-bind="selectorProps" variant="text" icon>
        <VBadge dot :color="station.hasNotifications ? 'warning' : 'transparent'">
          <VIcon :icon="toggleIcon" size="small" />
        </VBadge>
      </VBtn>
    </template>
    <NotificationsPanel @close="notificationsPopup = false" />
  </VMenu>
  <VDialog
    v-else
    v-model="notificationsPopup"
    persistent
    fullscreen
    scrollable
    :scrim="false"
    transition="dialog-bottom-transition"
  >
    <template #activator="{ props: selectorProps }">
      <VBtn v-bind="selectorProps" variant="text" icon>
        <VBadge dot :color="station.hasNotifications ? 'warning' : 'transparent'">
          <VIcon :icon="toggleIcon" size="small" />
        </VBadge>
      </VBtn>
    </template>
    <NotificationsPanel @close="notificationsPopup = false" />
  </VDialog>
</template>

<script lang="ts" setup>
import { mdiBellRing, mdiBellRingOutline } from '@mdi/js';
import { computed, ref } from 'vue';
import { useAppStore } from '~/stores/app.store';
import { useStationStore } from '~/stores/station.store';
import NotificationsPanel from './NotificationsPanel.vue';
import { VMenu } from 'vuetify/components';

const props = withDefaults(
  defineProps<{
    variant?: 'outlined' | 'filled';
  }>(),
  {
    variant: 'filled',
  },
);

const toggleIcon = computed(() =>
  props.variant === 'outlined' ? mdiBellRingOutline : mdiBellRing,
);

const notificationsPopup = ref(false);
const station = useStationStore();
const app = useAppStore();
</script>
