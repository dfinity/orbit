<template>
  <VMenu v-if="!mobile" v-model="notificationsPopup" location="end" :close-on-content-click="false">
    <template #activator="{ props: selectorProps }">
      <VBtn v-bind="selectorProps" variant="text" icon>
        <VBadge dot :color="activeBank.hasPendingOperations ? 'warning' : 'transparent'">
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
        <VBadge dot :color="activeBank.hasPendingOperations ? 'warning' : 'transparent'">
          <VIcon :icon="toggleIcon" size="small" />
        </VBadge>
      </VBtn>
    </template>
    <NotificationsPanel @close="notificationsPopup = false" />
  </VDialog>
</template>

<script lang="ts" setup>
import { mdiBellRing, mdiBellRingOutline } from '@mdi/js';
import { ref, computed } from 'vue';
import { useDisplay } from 'vuetify';
import { useActiveBankStore } from '~/ui/stores';
import NotificationsPanel from './NotificationsPanel.vue';

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

const { mobile } = useDisplay();
const notificationsPopup = ref(false);
const activeBank = useActiveBankStore();
</script>
