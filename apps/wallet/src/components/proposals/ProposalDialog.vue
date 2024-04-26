<template>
  <VDialog
    v-model="openModel"
    :persistent="loading || voting"
    transition="dialog-bottom-transition"
    scrollable
    :max-width="props.dialogMaxWidth.value"
  >
    <DataLoader
      v-if="currentProposalId"
      :key="currentProposalId"
      v-slot="{ data }"
      :load="loadProposal"
      @loading="loading = $event"
      @loaded="onProposalLoaded"
    >
      <VCard v-if="loading">
        <VToolbar color="background">
          <VToolbarTitle>{{ $t('terms.request') }}</VToolbarTitle>
          <VBtn :icon="mdiClose" @click="openModel = false" />
        </VToolbar>
        <VCardText v-if="loading" class="py-8">
          <LoadingMessage />
        </VCardText>
      </VCard>
      <ProposalDetailView
        v-else-if="data"
        :proposal="data.proposal"
        :details="{
          can_vote: data.privileges.can_vote,
          proposer_name: data.additionalInfo.proposer_name[0],
          voters: data.additionalInfo.voters,
        }"
        :loading="voting || loading"
        @closed="openModel = false"
        @opened="openModel = true"
        @approve="reason => onVote(true, reason)"
        @reject="reason => onVote(false, reason)"
      >
        <template #top-actions>
          <VSwitch
            v-if="data.privileges.can_vote"
            v-model="loadNext"
            data-test-id="load-next-proposal-switch"
            :label="$t('proposals.load_next')"
            class="flex-0-1"
            :hide-details="true"
            color="primary"
            :disabled="voting"
          />

          <VBtn :disabled="voting" :icon="mdiClose" @click="openModel = false" />
        </template>
        <template v-if="loadNext" #bottom-actions>
          <VBtn variant="plain" :disabled="voting" class="ma-0" @click="skip">
            {{ $t('terms.skip') }}
          </VBtn>
        </template>
      </ProposalDetailView>
    </DataLoader>
    <div v-else>
      <VCard class="text-center" flat data-test-id="no-more-proposals">
        <VCardText class="text-body-1 mt-10">
          <VIcon :icon="mdiCheckCircle" size="x-large" />
          {{ $t('proposals.no_more_requests_to_approve') }}
        </VCardText>
        <VCardActions class="pa-4 d-flex flex-md-row ga-2 justify-end">
          <VBtn variant="outlined" :disabled="loading" @click="openModel = false">
            {{ $t('terms.close') }}
          </VBtn>
        </VCardActions>
      </VCard>
    </div>
  </VDialog>
</template>
<script lang="ts" setup>
import { mdiCheckCircle, mdiClose } from '@mdi/js';
import { computed, ref, toRefs, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import { VBtn, VCard, VCardActions, VCardText, VDialog, VIcon, VSwitch } from 'vuetify/components';
import DataLoader from '~/components/DataLoader.vue';
import logger from '~/core/logger.core';
import {
  GetProposalResultData,
  ListProposalsOperationType,
  Proposal,
  UUID,
} from '~/generated/station/station.did';
import { mapProposalOperationToListProposalsOperationType } from '~/mappers/proposals.mapper';
import { services } from '~/plugins/services.plugin';
import { useAppStore } from '~/stores/app.store';
import { useStationStore } from '~/stores/station.store';
import { variantIs } from '~/utils/helper.utils';
import ProposalDetailView from './ProposalDetailView.vue';
import LoadingMessage from '~/components/LoadingMessage.vue';

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
  (event: 'proposal-changed', payload: UUID): void;
}>();
const currentProposalId = ref<UUID | null>(props.proposalId.value);
const preloadedData = ref<DataType | null>(null);
const voting = ref(false);
const loading = ref(false);
const skippedProposalIds = ref<UUID[]>([]);

const proposalType = ref<ListProposalsOperationType | undefined>();
const loadNext = ref(false);

watch(props.open, isOpen => {
  if (isOpen) {
    currentProposalId.value = props.proposalId.value;
    skippedProposalIds.value = [];
    preloadedData.value = null;
    loadNext.value = false;
  }
});

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
const station = useStationStore();

const loadProposal = async (): Promise<DataType> => {
  station.notifications.items.forEach(notification => {
    if (
      !notification.loading &&
      variantIs(notification.data.notification_type, 'ProposalCreated') &&
      !variantIs(notification.data.status, 'Read') &&
      notification.data.notification_type.ProposalCreated.proposal_id === currentProposalId.value
    ) {
      station.markNotificationRead(notification.data.id, true);
    }
  });

  if (preloadedData.value && preloadedData.value.proposal.id === currentProposalId.value) {
    return {
      proposal: preloadedData.value.proposal as Proposal,
      privileges: preloadedData.value.privileges,
      additionalInfo: preloadedData.value.additionalInfo,
    };
  } else {
    const result = await services().station.getProposal(
      { proposal_id: currentProposalId.value! },
      true,
    );
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
    emit('proposal-changed', currentProposalId.value);
  } else {
    currentProposalId.value = null;
  }

  voting.value = false;
};

const onProposalLoaded = (data: Awaited<ReturnType<typeof loadProposal>>): void => {
  proposalType.value = mapProposalOperationToListProposalsOperationType(data.proposal.operation);
};

const loadNextProposal = async (): Promise<DataType | null> => {
  const nextProposal = await services().station.getNextVotableProposal({
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

  return station.service
    .voteOnProposal({
      proposal_id: currentProposalId.value,
      approve,
      reason: reason && reason.length ? [reason] : [],
    })
    .then(async () => {
      app.sendNotification({
        type: 'success',
        message: i18n.t('app.action_save_success'),
      });

      if (loadNext.value) {
        // keep open, load next

        preloadedData.value = await loadNextProposal();

        if (preloadedData.value) {
          currentProposalId.value = preloadedData.value.proposal.id;
          emit('proposal-changed', currentProposalId.value);
        } else {
          currentProposalId.value = null;
        }
      } else {
        emit('voted');
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
