<template>
  <VDialog
    v-model="openModel"
    :persistent="loading || voting"
    transition="dialog-bottom-transition"
    scrollable
    :max-width="props.dialogMaxWidth.value"
  >
    <!-- <div class="proposal-transition-container"> -->
    <Transition>
      <!-- <div :key="currentProposalId!"> -->
      <DataLoader
        :key="currentProposalId!"
        v-slot="{ data }"
        :load="loadProposal"
        @loading="loading = $event"
        @loaded="onProposalLoaded"
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
            <VSwitch
              label="Load next"
              class="flex-0-1"
              :hide-details="true"
              color="primary"
              v-model="loadNext"
              :disabled="voting"
              v-if="data.privileges.can_vote"
            ></VSwitch>

            <VBtn :disabled="voting" :icon="mdiClose" dark @click="openModel = false" />
          </template>
          <template #bottom-actions v-if="loadNext">
            <VBtn variant="plain" :disabled="voting" class="ma-0" @click="skip">
              {{ $t('terms.skip') }}
            </VBtn>
          </template>
        </ProposalDetailView>
      </DataLoader>
      <!-- </div> -->
    </Transition>
    <!-- </div> -->
  </VDialog>
</template>
<script lang="ts" setup>
import { mdiClose } from '@mdi/js';
import { computed, ref, toRefs } from 'vue';
import { useI18n } from 'vue-i18n';
import DataLoader from '~/components/DataLoader.vue';
import logger from '~/core/logger.core';
import {
  GetProposalResultData,
  ListProposalsOperationType,
  Proposal,
  UUID,
} from '~/generated/wallet/wallet.did';
import { useAppStore } from '~/stores/app.store';
import { useWalletStore } from '~/stores/wallet.store';
import ProposalDetailView from './ProposalDetailView.vue';
import { variantIs } from '~/utils/helper.utils';
import { mapProposalOperationToListProposalsOperationType } from '~/mappers/proposals.mapper';
import { VBtn, VSwitch } from 'vuetify/components';

// type DataType = Awaited<ReturnType<WalletService['getProposal']>>;
type DataType = {
  proposal: GetProposalResultData['proposal'];
  privileges: GetProposalResultData['privileges'];
  additionalInfo: GetProposalResultData['additional_info'];
};

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
const currentProposalId = ref<UUID | null>(props.proposalId.value);
const preloadedData = ref<DataType | null>(null);
const voting = ref(false);
const loading = ref(false);
const skippedProposalIds = ref<UUID[]>([]);

const proposalType = ref<ListProposalsOperationType | undefined>();
const loadNext = ref(false);

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

const loadProposal = async (): Promise<DataType> => {
  wallet.notifications.items.forEach(notification => {
    if (
      !notification.loading &&
      variantIs(notification.data.notification_type, 'ProposalCreated') &&
      !variantIs(notification.data.status, 'Read') &&
      notification.data.notification_type.ProposalCreated.proposal_id === currentProposalId.value
    ) {
      wallet.markNotificationRead(notification.data.id, true);
    }
  });

  if (preloadedData.value && preloadedData.value.proposal.id === currentProposalId.value) {
    return {
      proposal: preloadedData.value.proposal as Proposal,
      privileges: preloadedData.value.privileges,
      additionalInfo: preloadedData.value.additionalInfo,
    };
  } else {
    const result = await wallet.service.getProposal({ proposal_id: currentProposalId.value! });
    return {
      proposal: result.proposal,
      privileges: result.privileges,
      additionalInfo: result.additional_info,
    };
  }
};

const skip = async (): Promise<void> => {
  voting.value = true;
  skippedProposalIds.value.push(currentProposalId.value!);

  preloadedData.value = await loadNextProposal();

  if (preloadedData.value) {
    currentProposalId.value = preloadedData.value.proposal.id;
  } else {
    currentProposalId.value = null;
  }

  voting.value = false;
};

const onProposalLoaded = (data: Awaited<ReturnType<typeof loadProposal>>): void => {
  proposalType.value = mapProposalOperationToListProposalsOperationType(data.proposal.operation);
};

const loadNextProposal = async (): Promise<DataType | null> => {
  const nextProposal = await wallet.service.getNextVotableProposal({
    types: [proposalType.value!],
    excludedProposalIds: skippedProposalIds.value,
  });

  if (nextProposal.length === 0) {
    return null;
  }

  return {
    proposal: nextProposal[0].proposal,
    privileges: nextProposal[0].privileges,
    additionalInfo: nextProposal[0].additional_info,
  };
};

const onVote = async (approve: boolean, reason?: string): Promise<void> => {
  if (currentProposalId.value === null) {
    return;
  }

  voting.value = true;

  // return
  return wallet.service
    .voteOnProposal({
      proposal_id: currentProposalId.value,
      approve,
      reason: reason && reason.length ? [reason] : [],
    })
    .then(async () => {
      //

      app.sendNotification({
        type: 'success',
        message: i18n.t('app.action_save_success'),
      });

      emit('voted');

      if (loadNext.value) {
        // keep open, load next

        preloadedData.value = await loadNextProposal();

        if (preloadedData.value) {
          currentProposalId.value = preloadedData.value.proposal.id;
        } else {
          currentProposalId.value = null;
        }
      } else {
        openModel.value = false;
      }
    })
    .catch(err => {
      logger.error(`Failed to vote on proposal:`, err);

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

<style lang="scss" scoped>
.proposal-transition-container {
  display: grid;
  grid-template-columns: 1fr;
  grid-template-rows: 1fr;

  > * {
    grid-column: 1;
    grid-row: 1;
  }
}

.v-enter-active,
.v-leave-active {
  transition:
    opacity 0.3s ease,
    transform 0.3s ease;
}

.v-enter-from {
  opacity: 0;
  transform: translateX(100%);
}

.v-leave-to {
  opacity: 0;
  transform: translateX(-100%);
}
</style>
