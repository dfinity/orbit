<template>
  <VCard :width="mobile ? '100%' : '400px'">
    <VList density="compact">
      <VListItem density="compact" class="notifications-panel__title">
        {{ $t('wallets.pending_proposals') }}
        <VSpacer />
        <VBtn :icon="mdiClose" variant="flat" @click="emit('close')" />
      </VListItem>
    </VList>
    <VDivider />
    <VList density="compact">
      <VListItem v-if="!wallet.hasNotifications" class="text-center">
        {{ $t('terms.all_done') }}
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
import { useWalletStore } from '~/ui/stores/wallet';
import { Notification } from '~/generated/wallet/wallet.did';
import { useDisplay } from 'vuetify';
import NotificationListItem from '~/ui/components/NotificationListItem.vue';

const { mobile } = useDisplay();
const wallet = useWalletStore();

const emit = defineEmits<{
  (event: 'close'): void;
}>();

const onRead = (notification: Notification, read: boolean) =>
  wallet.markNotificationRead(notification.id, read);
</script>

<style lang="scss">
.notifications-panel {
  &__title {
    .v-list-item__content {
      display: flex;
      flex-direction: row;
      align-items: center;
    }
  }
}
</style>
