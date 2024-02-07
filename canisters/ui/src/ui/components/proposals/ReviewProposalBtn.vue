<template>
  <VBtn
    data-test-id="review-proposal-btn"
    :variant="props.variant"
    :size="props.size"
    @click="open = true"
  >
    {{ props.proposal.info.can_vote ? $t('terms.review') : $t('terms.view') }}
    <VDialog
      v-model="open"
      transition="dialog-bottom-transition"
      :persistent="voting"
      scrollable
      max-width="800"
    >
      <ProposalDetailView
        :proposal="props.proposal"
        :loading="voting"
        @closed="open = false"
        @opened="open = true"
        @approve="onVote(true)"
        @reject="onVote(false)"
      >
        <template #top-actions>
          <VBtn :disabled="voting" :icon="mdiClose" dark @click="open = false" />
        </template>
      </ProposalDetailView>
    </VDialog>
  </VBtn>
</template>
<script setup lang="ts">
import { mdiClose } from '@mdi/js';
import { Proposal } from '~/generated/wallet/wallet.did';
import ProposalDetailView from './ProposalDetailView.vue';
import { ref } from 'vue';
import { useWalletStore } from '~/ui/stores/wallet';
import { logger } from '~/core/logger.core';
import { useAppStore } from '~/ui/stores/app';
import { i18n } from '~/ui/modules';
import { watch } from 'vue';

const wallet = useWalletStore();
const app = useAppStore();
const voting = ref(false);

const props = withDefaults(
  defineProps<{
    proposal: Proposal;
    variant?: 'outlined';
    size?: 'x-small' | 'small' | 'default' | 'large' | 'x-large';
  }>(),
  {
    variant: 'outlined',
    size: 'small',
  },
);

const emit = defineEmits<{
  (event: 'voted'): void;
  (event: 'closed'): void;
  (event: 'opened'): void;
}>();

const open = ref(false);

watch(
  () => open.value,
  open => {
    if (open) {
      emit('opened');
    } else {
      emit('closed');
    }
  },
);

const onVote = async (approve: boolean, reason?: string): Promise<void> => {
  voting.value = true;

  return wallet.service
    .voteOnProposal({
      proposal_id: props.proposal.id,
      approve,
      reason: reason && reason.length ? [reason] : [],
    })
    .then(() => {
      open.value = false;

      emit('voted');

      app.sendNotification({
        type: 'error',
        message: i18n.global.t('app.action_save_success'),
      });
    })
    .catch(err => {
      logger.error(`Failed to vote on proposal: ${JSON.stringify(err as Error)}`);

      app.sendNotification({
        type: 'error',
        message: i18n.global.t('app.action_save_failed'),
      });
    })
    .finally(() => {
      voting.value = false;
    });
};
</script>
