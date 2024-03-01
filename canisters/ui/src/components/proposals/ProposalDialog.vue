<template>
  <VDialog
    v-model="openModel"
    :persistent="loading || voting"
    transition="dialog-bottom-transition"
    scrollable
    :max-width="props.dialogMaxWidth.value"
  >
    <DataLoader
      v-slot="{ data }"
      :key="props.proposalId.value"
      :load="loadProposal"
      @loading="loading = $event"
    >
      <ProposalDetailView
        v-if="data"
        :proposal="data.proposal"
        :details="{
          can_vote: data.privileges.can_vote,
          proposer_name: data.additionalInfo.proposer_name?.[0] ?? undefined,
        }"
        :loading="voting || loading"
        @closed="openModel = false"
        @opened="openModel = true"
        @approve="onVote(true)"
        @reject="onVote(false)"
      >
        <template #top-actions>
          <VBtn :disabled="voting" :icon="mdiClose" dark @click="openModel = false" />
        </template>
      </ProposalDetailView>
    </DataLoader>
  </VDialog>
</template>
<script lang="ts" setup>
import { mdiClose } from '@mdi/js';
import { computed, ref, toRefs } from 'vue';
import { useI18n } from 'vue-i18n';
import DataLoader from '~/components/DataLoader.vue';
import logger from '~/core/logger.core';
import {
  Proposal,
  ProposalAdditionalInfo,
  ProposalCallerPrivileges,
  UUID,
} from '~/generated/wallet/wallet.did';
import { useAppStore } from '~/stores/app.store';
import { useWalletStore } from '~/stores/wallet.store';
import ProposalDetailView from './ProposalDetailView.vue';
import { variantIs } from '~/utils/helper.utils';

const input = withDefaults(
  defineProps<{
    proposalId: UUID;
    open?: boolean;
    dialogMaxWidth?: number;
    readonly?: boolean;
  }>(),
  {
    open: false,
    dialogMaxWidth: 800,
    readonly: false,
  },
);
const props = toRefs(input);
const emit = defineEmits<{
  (event: 'update:open', payload: boolean): void;
  (event: 'voted'): void;
  (event: 'closed'): void;
  (event: 'opened'): void;
}>();
const voting = ref(false);
const loading = ref(false);
const openModel = computed({
  get: () => props.open.value,
  set: value => {
    emit('update:open', value);

    if (value) {
      emit('opened');
    } else {
      emit('closed');
    }
  },
});
const i18n = useI18n();
const app = useAppStore();
const wallet = useWalletStore();

const loadProposal = async (): Promise<{
  proposal: Proposal;
  privileges: ProposalCallerPrivileges;
  additionalInfo: ProposalAdditionalInfo;
}> => {
  wallet.notifications.items.forEach(notification => {
    if (
      !notification.loading &&
      variantIs(notification.data.notification_type, 'ProposalCreated') &&
      !variantIs(notification.data.status, 'Read') &&
      notification.data.notification_type.ProposalCreated.proposal_id === props.proposalId.value
    ) {
      wallet.markNotificationRead(notification.data.id, true);
    }
  });

  const result = await wallet.service.getProposal({ proposal_id: props.proposalId.value });
  return {
    proposal: result.proposal,
    privileges: result.privileges,
    additionalInfo: result.additional_info,
  };
};

const onVote = async (approve: boolean, reason?: string): Promise<void> => {
  voting.value = true;

  return wallet.service
    .voteOnProposal({
      proposal_id: props.proposalId.value,
      approve,
      reason: reason && reason.length ? [reason] : [],
    })
    .then(() => {
      openModel.value = false;

      app.sendNotification({
        type: 'success',
        message: i18n.t('app.action_save_success'),
      });

      emit('voted');
    })
    .catch(err => {
      logger.error(`Failed to vote on proposal: ${err}`);

      app.sendNotification({
        type: 'error',
        message: i18n.t('app.action_save_failed'),
      });
    })
    .finally(() => {
      voting.value = false;
    });
};
</script>
