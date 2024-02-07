<template>
  <div class="notification-item">
    <div v-if="props.loading" class="notification-item__loading"></div>
    <div class="notification-item__read">
      <VBtn
        :icon="isRead ? mdiCheckCircle : mdiCheckCircleOutline"
        size="x-small"
        :variant="isRead ? 'text' : 'plain'"
        @click="onRead"
      />
    </div>
    <div class="notification-item__code">
      <div class="notification-item__code__title">{{ notification.title }}</div>
      <div v-if="notification.message.length" class="notification-item__code__message">
        {{ notification.message }}
      </div>
      <div class="notification-item__code__time">
        <VChip size="x-small" :title="notification.created_at" variant="tonal">
          <VIcon :icon="mdiClockOutline" size="x-small" />&nbsp;
          {{ new Date(notification.created_at).toLocaleDateString() }}
        </VChip>
      </div>
    </div>
    <div v-if="props.loading" class="notification-item__action">
      <VProgressCircular indeterminate color="primary" size="small" class="mx-4" />
    </div>
    <div
      v-else-if="
        'ProposalCreated' in notification.notification_type &&
        notification.notification_type.ProposalCreated.account_id
      "
      class="notification-item__action"
    >
      <VBtn
        :to="{
          name: Routes.Account,
          params: { id: notification.notification_type.ProposalCreated.account_id },
        }"
        size="x-small"
        variant="tonal"
        :icon="mdiOpenInApp"
      />
    </div>
  </div>
</template>
<script lang="ts" setup>
import { mdiCheckCircle, mdiCheckCircleOutline, mdiClockOutline, mdiOpenInApp } from '@mdi/js';
import { computed } from 'vue';
import { Routes } from '~/configs/routes.config';
import { Notification } from '~/generated/wallet/wallet.did';

const props = withDefaults(
  defineProps<{
    notification: Notification;
    loading?: boolean;
  }>(),
  {
    loading: false,
  },
);

const emit = defineEmits<{
  (event: 'update:notification', payload: Notification): void;
  (event: 'read', payload: boolean): void;
}>();

const notification = computed({
  get: () => props.notification,
  set: value => emit('update:notification', value),
});

const isRead = computed(() => 'Read' in notification.value.status);

const onRead = () => {
  emit('read', !isRead.value);
};
</script>
<style lang="scss">
.notification-item {
  display: flex;
  flex-direction: row;
  align-items: center;
  gap: var(--ds-bdu);
  position: relative;

  &__loading {
    position: absolute;
    width: 100%;
    height: 100%;
    background: rgb(var(--ds-background));
    opacity: 0.4;
    z-index: 1;
  }

  &__action {
    flex: 0 0 auto;
  }

  &__action {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: var(--ds-bdu);
  }

  &__code {
    flex: 1 1 auto;
    display: flex;
    flex-direction: column;
    justify-content: center;
    overflow: hidden;
    gap: calc(var(--ds-bdu) / 2);
    border-right: var(--ds-border-width) var(--ds-border-style) rgb(var(--ds-background));

    &__title {
      font-weight: 500;
      font-size: var(--ds-font-size-xs);
      text-overflow: ellipsis;
      overflow: hidden;
      padding-right: var(--ds-bdu);
      line-height: 20px;
      color: var(--ds-text-primary);
      white-space: nowrap;
    }

    &__message {
      font-weight: normal;
      font-size: var(--ds-font-size-xxs);
      color: var(--ds-text-primary);
    }

    &__time {
      display: flex;
      white-space: nowrap;
      font-weight: 400;
      font-size: var(--ds-font-size-xxs);
      line-height: var(--ds-font-size-xxs);
      color: var(--ds-text-secondary);
      gap: calc(var(--ds-bdu) / 2);
    }
  }
}
</style>
