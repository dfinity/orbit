<template>
  <div class="notification d-flex flex-row ga-2">
    <div v-if="props.loading" class="notification__loading"></div>
    <div class="d-flex justify-center align-center">
      <VBtn
        :icon="isRead ? mdiCheckCircle : mdiCheckCircleOutline"
        size="x-small"
        :variant="isRead ? 'text' : 'plain'"
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
      <div class="d-flex align-center justify-center flex-column h-100">
        <VProgressCircular v-if="loading" indeterminate color="primary" size="small" class="mx-4" />
        <VBtn
          v-else-if="
            variantIs(notification.notification_type, 'ProposalCreated') &&
            notification.notification_type.ProposalCreated.account_id?.[0]
          "
          size="small"
          variant="text"
          :icon="mdiOpenInApp"
          @click="openProposal(notification.notification_type.ProposalCreated.proposal_id)"
        />
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { mdiCheckCircle, mdiCheckCircleOutline, mdiClockOutline, mdiOpenInApp } from '@mdi/js';
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

const openProposal = (proposalId: UUID): void => {
  emit('read', true);

  router.push({
    query: { [PROPOSAL_DIALOG_QUERY_PARAM]: proposalId },
  });
};

const onRead = () => {
  emit('read', !isRead.value);
};
</script>

<style lang="scss" scoped>
.notification {
  position: relative;

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
