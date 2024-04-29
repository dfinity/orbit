<template>
  <VCard :width="app.isMobile ? '100%' : '400px'">
    <VToolbar color="surface">
      <VToolbarTitle class="text-body-1 font-weight-bold">
        {{ $t('app.notifications_panel_title') }}
      </VToolbarTitle>
      <VBtn :icon="mdiClose" size="small" @click="emit('close')" />
    </VToolbar>
    <VDivider />
    <VList density="compact" max-height="500px" class="py-0">
      <VListItem v-if="!station.hasNotifications" class="text-center">
        {{ $t('app.notifications_panel_no_results') }}
      </VListItem>
      <VListItem
        v-for="({ loading, data }, idx) in station.sortedNotifications"
        :key="idx"
        class="py-0 px-0"
      >
        <NotificationListItem
          :loading="loading"
          :notification="station.sortedNotifications[idx].data"
          @read="read => onRead(data, read)"
        />
        <VDivider v-if="station.notifications.items.length - 1 !== idx" />
      </VListItem>
    </VList>
    <template v-if="station.hasNotifications">
      <VDivider />
      <div class="d-flex justify-end flex-column mx-4 my-4">
        <VBtn
          size="x-small"
          :prepend-icon="mdiCheckAll"
          variant="tonal"
          :loading="station.notifications.loading"
          small
          @click="station.markAllNotificationsRead"
        >
          {{ $t('app.notifications_panel_read_all') }}
        </VBtn>
      </div>
    </template>
  </VCard>
</template>

<script lang="ts" setup>
import { mdiCheckAll, mdiClose } from '@mdi/js';
import {
  VBtn,
  VCard,
  VDivider,
  VList,
  VListItem,
  VToolbar,
  VToolbarTitle,
} from 'vuetify/components';
import { Notification } from '~/generated/station/station.did';
import { useAppStore } from '~/stores/app.store';
import { useStationStore } from '~/stores/station.store';
import NotificationListItem from './NotificationListItem.vue';

const app = useAppStore();
const station = useStationStore();

const emit = defineEmits<{
  (event: 'close'): void;
}>();

const onRead = (notification: Notification, read: boolean) =>
  station.markNotificationRead(notification.id, read);
</script>
