<template>
  <VCard :width="app.isMobile ? '100%' : '400px'">
    <VToolbar dark color="surface">
      <VToolbarTitle class="text-body-1 font-weight-bold">
        {{ $t('app.notifications_panel_title') }}
      </VToolbarTitle>
      <VBtn :icon="mdiClose" size="small" variant="flat" @click="emit('close')" />
    </VToolbar>
    <VDivider />
    <VList density="compact" max-height="500px" class="py-0">
      <VListItem v-if="!wallet.hasNotifications" class="text-center">
        {{ $t('app.notifications_panel_no_results') }}
      </VListItem>
      <VListItem
        v-for="({ loading, data }, idx) in wallet.sortedNotifications"
        :key="idx"
        class="py-0 px-0"
      >
        <NotificationListItem
          :loading="loading"
          :notification="wallet.sortedNotifications[idx].data"
          @read="read => onRead(data, read)"
        />
        <VDivider v-if="wallet.notifications.items.length - 1 !== idx" />
      </VListItem>
    </VList>
    <template v-if="wallet.hasNotifications">
      <VDivider />
      <div class="d-flex justify-end flex-column mx-4 my-4">
        <VBtn
          size="x-small"
          :prepend-icon="mdiCheckAll"
          variant="tonal"
          :loading="wallet.notifications.loading"
          small
          @click="wallet.markAllNotificationsRead"
        >
          {{ $t('app.notifications_panel_read_all') }}
        </VBtn>
      </div>
    </template>
  </VCard>
</template>

<script lang="ts" setup>
import { mdiCheckAll, mdiClose } from '@mdi/js';
import { Notification } from '~/generated/wallet/wallet.did';
import { useAppStore } from '~/stores/app.store';
import { useWalletStore } from '~/stores/wallet.store';
import NotificationListItem from './NotificationListItem.vue';

const app = useAppStore();
const wallet = useWalletStore();

const emit = defineEmits<{
  (event: 'close'): void;
}>();

const onRead = (notification: Notification, read: boolean) =>
  wallet.markNotificationRead(notification.id, read);
</script>
