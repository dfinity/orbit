<template>
  <div class="notification d-flex flex-row ga-2 cursor-pointer pa-2" @click="onRowClick">
    <div v-if="props.loading" class="notification__loading"></div>
    <div class="d-flex justify-center align-center">
      <VBtn
        :loading="loading"
        :icon="isRead ? mdiEmailOpenOutline : mdiClose"
        size="small"
        variant="plain"
        density="comfortable"
        @click="onRead"
      />
    </div>
    <div class="d-flex flex-grow-1 align-start justify-center flex-row ga-1">
      <div class="d-flex flex-grow-1 align-start justify-center flex-column ga-1">
        <div class="d-flex text-no-wrap flex-no-wrap">{{ notification.title }}</div>
        <div v-if="message" class="d-flex flex-no-wrap text-body-2 mb-1 text-medium-emphasis">
          <TextOverflow :text="message" :max-length="100" />
        </div>
        <div class="d-flex text-no-wrap flex-no-wrap">
          <VChip
            size="x-small"
            :title="formatLocaleDatetimeString(notification.created_at)"
            variant="tonal"
          >
            <VIcon :icon="mdiClockOutline" size="x-small" />&nbsp;
            {{ new Date(notification.created_at).toLocaleDateString() }}
          </VChip>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { mdiClockOutline, mdiClose, mdiEmailOpenOutline } from '@mdi/js';
import { computed } from 'vue';
import { useRouter } from 'vue-router';
import TextOverflow from '~/components/TextOverflow.vue';
import { PROPOSAL_DIALOG_QUERY_PARAM } from '~/core/constants.core';
import { Notification, UUID } from '~/generated/wallet/wallet.did';
import { formatLocaleDatetimeString } from '~/utils/date.utils';
import { variantIs } from '~/utils/helper.utils';

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

const message = computed(() => notification.value.message?.[0]);

const isRead = computed(() => variantIs(notification.value.status, 'Read'));
const router = useRouter();

const onRowClick = () => {
  if (!variantIs(notification.value.notification_type, 'ProposalCreated')) {
    return;
  }

  openProposal(notification.value.notification_type.ProposalCreated.proposal_id);
};

const openProposal = (proposalId: UUID): void => {
  emit('read', true);

  router.push({
    query: { [PROPOSAL_DIALOG_QUERY_PARAM]: proposalId },
  });
};

const onRead = (event: ClipboardEvent) => {
  event.stopPropagation();
  event.preventDefault();

  emit('read', !isRead.value);
};
</script>

<style lang="scss" scoped>
.notification {
  position: relative;

  &:hover {
    background: rgb(var(--ds-background));
  }

  &__loading {
    position: absolute;
    width: 100%;
    height: 100%;
    background: rgb(var(--ds-background));
    opacity: 0.4;
    z-index: 1;
  }
}
</style>
