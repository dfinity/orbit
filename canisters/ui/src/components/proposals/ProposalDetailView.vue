<template v-if="proposalReviewComponent">
  <VCard :loading="props.loading">
    <VToolbar color="background">
      <VToolbarTitle class="flex-fill">
        <span class="text-body-2 font-weight-light text-wrap">
          {{ $t(`proposals.types.${proposalType}.request_title`) }}
        </span>
        <br />
        <span v-if="props.proposal.title" class="text-wrap">
          {{ props.proposal.title }}
          <VTooltip
            v-model="titleTooltip"
            location="bottom"
            :open-on-hover="false"
            :open-on-click="true"
            @click:outside="titleTooltip = false"
          >
            <template #activator="{ props: infoProps }">
              <VBtn :icon="mdiInformationOutline" size="x-small" v-bind="infoProps" />
            </template>
            {{ $t('proposals.title_info_message') }}
          </VTooltip>
        </span>
      </VToolbarTitle>
      <slot name="top-actions"></slot>
    </VToolbar>
    <VCardText class="px-4 pt-2">
      <VContainer class="px-0">
        <VRow v-if="props.proposal.summary?.[0]">
          <VCol cols="12" class="text-h6 font-weight-bold">
            <VTextarea
              :model-value="props.proposal.summary[0]"
              :label="$t('terms.summary')"
              variant="plain"
              readonly
              hide-details
              rows="1"
              auto-grow
              class="my-2"
            />
          </VCol>
        </VRow>
        <VRow>
          <VCol cols="12" class="text-body-1 font-weight-bold py-0">
            {{ $t('terms.requested') }}
          </VCol>
        </VRow>
        <VRow>
          <VCol cols="12">
            <component
              :is="detailView?.component"
              v-if="detailView"
              :proposal="proposal"
              :operation="detailView.operation"
              mode="detail"
            />
          </VCol>
        </VRow>
      </VContainer>
    </VCardText>

    <table v-if="votes.length > 0" class="voters mx-4 text-body-1" data-test-id="proposal-votes">
      <thead>
        <tr>
          <th>{{ $t('proposals.votes') }}</th>
          <th></th>
          <th></th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="vote in votes" :key="vote.voter?.id">
          <td>
            {{ vote.voter.name?.[0] || vote.voter.id }}
          </td>
          <td>
            <VoteChip :status="vote.vote.status" size="small" class="ml-2" />
          </td>
          <td>
            <p v-if="vote.vote.status_reason[0]" class="text-medium-emphasis text-body-2">
              {{ vote.vote.status_reason[0] }}
            </p>
          </td>
        </tr>
      </tbody>
    </table>

    <VCardText v-if="props.details.can_vote || reason" class="px-4 pt-2">
      <VContainer class="px-0">
        <VRow>
          <VCol cols="12">
            <VTextarea
              v-model.trim="reason"
              data-test-id="proposal-details-comment"
              :label="$t('proposals.comment_optional')"
              :variant="props.details.can_vote ? 'underlined' : 'plain'"
              hide-details
              rows="1"
              auto-grow
              :readonly="props.loading || !props.details.can_vote"
            />
          </VCol>
        </VRow>
      </VContainer>
    </VCardText>

    <VCardActions class="pa-4 d-flex flex-column-reverse flex-column flex-md-row ga-4">
      <ProposalMetadata
        :proposal="props.proposal"
        :details="props.details"
        class="flex-grow-1 flex-md-grow-0 align-self-start align-self-md-end"
        :class="{ 'mt-8': props.details.can_vote }"
      />
      <div class="d-flex flex-column flex-md-row ga-1 justify-end flex-grow-1 w-100 w-md-auto">
        <template v-if="!props.details.can_vote">
          <VBtn
            data-test-id="proposal-details-reject"
            variant="elevated"
            color="error"
            class="ma-0"
            :disabled="props.loading"
            @click="$emit('reject', reasonOrUndefined)"
          >
            {{ $t('terms.reject') }}
          </VBtn>
          <VBtn
            data-test-id="proposal-details-approve"
            variant="elevated"
            color="success"
            class="ma-0"
            :disabled="props.loading"
            @click="$emit('approve', reasonOrUndefined)"
          >
            {{ $t('terms.approve') }}
          </VBtn>
          <slot name="bottom-actions"></slot>
        </template>
        <template v-else>
          <ProposalStatusChip :status="proposal.status" />
          <VDivider class="d-md-none mx-2" />
        </template>
      </div>
    </VCardActions>
  </VCard>
</template>

<script setup lang="ts">
import { mdiInformationOutline } from '@mdi/js';
import type { Component } from 'vue';
import { computed, ref } from 'vue';
import { useI18n } from 'vue-i18n';
import {
  VBtn,
  VCardActions,
  VCardText,
  VCol,
  VContainer,
  VDivider,
  VRow,
  VTextarea,
  VToolbar,
  VToolbarTitle,
  VTooltip,
} from 'vuetify/components';
import { Proposal, ProposalOperation } from '~/generated/wallet/wallet.did';
import { ProposalDetails } from '~/types/wallet.types';
import { KeysOfUnion } from '~/utils/helper.utils';
import AddAccountOperation from './operations/AddAccountOperation.vue';
import AddAddressBookEntryOperation from './operations/AddAddressBookEntryOperation.vue';
import AddProposalPolicyOperation from './operations/AddProposalPolicyOperation.vue';
import AddUserGroupOperation from './operations/AddUserGroupOperation.vue';
import AddUserOperation from './operations/AddUserOperation.vue';
import ChangeCanisterOperation from './operations/ChangeCanisterOperation.vue';
import EditAccessPolicyOperation from './operations/EditAccessPolicyOperation.vue';
import EditAccountOperation from './operations/EditAccountOperation.vue';
import EditAddressBookEntryOperation from './operations/EditAddressBookEntryOperation.vue';
import EditProposalPolicyOperation from './operations/EditProposalPolicyOperation.vue';
import EditUserGroupOperation from './operations/EditUserGroupOperation.vue';
import EditUserOperation from './operations/EditUserOperation.vue';
import RemoveAddressBookEntryOperation from './operations/RemoveAddressBookEntryOperation.vue';
import RemoveProposalPolicyOperation from './operations/RemoveProposalPolicyOperation.vue';
import RemoveUserGroupOperation from './operations/RemoveUserGroupOperation.vue';
import TransferOperation from './operations/TransferOperation.vue';
import ProposalMetadata from './ProposalMetadata.vue';
import ProposalStatusChip from './ProposalStatusChip.vue';
import VoteChip from './VoteChip.vue';

const i18n = useI18n();

const props = withDefaults(
  defineProps<{
    proposal: Proposal;
    details: ProposalDetails;
    loading?: boolean;
  }>(),
  {
    loading: false,
  },
);

const titleTooltip = ref(false);

const componentsMap: {
  [key in KeysOfUnion<ProposalOperation>]: Component;
} = {
  AddUserGroup: AddUserGroupOperation,
  AddUser: AddUserOperation,
  EditUser: EditUserOperation,
  EditUserGroup: EditUserGroupOperation,
  AddAccount: AddAccountOperation,
  EditAccount: EditAccountOperation,
  Transfer: TransferOperation,
  AddAddressBookEntry: AddAddressBookEntryOperation,
  EditAddressBookEntry: EditAddressBookEntryOperation,
  RemoveAddressBookEntry: RemoveAddressBookEntryOperation,
  RemoveUserGroup: RemoveUserGroupOperation,
  AddProposalPolicy: AddProposalPolicyOperation,
  EditProposalPolicy: EditProposalPolicyOperation,
  RemoveProposalPolicy: RemoveProposalPolicyOperation,
  ChangeCanister: ChangeCanisterOperation,
  EditAccessPolicy: EditAccessPolicyOperation,
};

defineEmits<{
  (event: 'approve', reason?: string): void;
  (event: 'reject', reason?: string): void;
}>();

const detailView = computed<{
  component: Component;
  operation: ProposalOperation[keyof ProposalOperation];
} | null>(() => {
  const keys = Object.keys(componentsMap) as Array<keyof ProposalOperation>;
  for (const key of keys) {
    if (key in props.proposal.operation && key in componentsMap) {
      return {
        component: componentsMap[key],
        operation: props.proposal.operation[key],
      };
    }
  }

  return null;
});

const proposalType = computed(() => {
  const keys = Object.keys(componentsMap) as KeysOfUnion<ProposalOperation>[];
  for (const key of keys) {
    if (key in props.proposal.operation) {
      return key.toLowerCase();
    }
  }

  return 'unknown';
});

const reason = ref('');
const reasonOrUndefined = computed(() => (reason.value.length ? reason.value : undefined));

const votes = computed(() =>
  props.proposal.votes.map(vote => {
    const voter = props.details.voters.find(voter => voter.id === vote.user_id);

    if (voter?.id === props.proposal.proposed_by && !vote.status_reason[0]) {
      vote.status_reason[0] = i18n.t('proposals.proposer_auto_approval');
    }

    return {
      voter: voter || {
        id: vote.user_id,
        name: [],
      },
      vote,
    };
  }),
);
</script>

<style scoped lang="scss">
.voters {
  width: 100%;
  border-collapse: collapse;

  th {
    text-align: left;
    padding: 0px 4px 8px;
  }

  td {
    padding: 0px 4px 8px;
    vertical-align: top;
  }
}
</style>
