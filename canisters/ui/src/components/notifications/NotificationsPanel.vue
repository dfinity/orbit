<template>
  <VCard :width="app.isMobile ? '100%' : '400px'">
    <VToolbar dark color="surface">
      <VToolbarTitle class="text-body-1 font-weight-bold">{{
        $t('app.notifications_panel_title')
      }}</VToolbarTitle>
      <VBtn :icon="mdiClose" size="small" variant="flat" @click="emit('close')" />
    </VToolbar>
    <VDivider />
    <VList density="compact">
      <VListItem v-if="!wallet.hasNotifications" class="text-center">
        {{ $t('app.notifications_panel_no_results') }}
      </VListItem>
      <VListItem v-for="({ loading, data }, idx) in wallet.sortedNotifications" :key="idx">
        <NotificationListItem
          :loading="loading"
          :notification="wallet.sortedNotifications[idx].data"
          @read="read => onRead(data, read)"
        />
        <VDivider v-if="wallet.notifications.items.length - 1 !== idx" class="mt-4" />
      </VListItem>
    </VList>
  </VCard>
</template>

<script lang="ts" setup>
import { mdiClose } from '@mdi/js';
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
