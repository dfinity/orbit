<template>
  <VCard
    :elevation="props.mode === 'grid' ? 0 : undefined"
    density="compact"
    class="w-min-25"
    :rounded="props.mode === 'grid' ? 0 : undefined"
    :class="{ 'br-on-background': props.mode === 'grid' }"
  >
    <VCardTitle class="text-body-2 font-weight-bold">
      {{ $t(`proposals.types.${proposalType}.title`) }}
    </VCardTitle>
    <VCardText class="px-4 pb-1">
      <component
        :is="itemView?.component"
        v-if="itemView"
        :proposal="props.proposal"
        :operation="itemView.operation"
        mode="list"
      />
    </VCardText>
    <VCardActions class="px-4">
      <ProposalStatusChip size="small" :status="props.proposal.status" />
      <VSpacer />
      <ReviewProposalBtn
        :proposal-id="props.proposal.id"
        :can-vote="props.details.can_vote"
        @voted="$emit('voted')"
        @opened="$emit('opened')"
        @closed="$emit('closed')"
      />
    </VCardActions>
  </VCard>
</template>

<script setup lang="ts">
import type { Component } from 'vue';
import { computed } from 'vue';
import { Proposal, ProposalOperation } from '~/generated/wallet/wallet.did';
import { ProposalDetails } from '~/types/wallet.types';
import { KeysOfUnion } from '~/utils/helper.utils';
import ProposalStatusChip from './ProposalStatusChip.vue';
import ReviewProposalBtn from './ReviewProposalBtn.vue';
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
import { VCard, VCardActions, VCardText, VCardTitle, VSpacer } from 'vuetify/components';

const props = withDefaults(
  defineProps<{
    proposal: Proposal;
    details: ProposalDetails;
    hideColumnBorders?: boolean;
    mode?: 'list' | 'grid';
  }>(),
  {
    hideColumnBorders: false,
    mode: 'list',
  },
);

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
  (event: 'voted'): void;
  (event: 'opened'): void;
  (event: 'closed'): void;
}>();

const itemView = computed<{
  component: Component;
  operation: ProposalOperation[keyof ProposalOperation];
} | null>(() => {
  const keys = Object.keys(componentsMap) as Array<keyof ProposalOperation>;
  for (const key of keys) {
    if (key in props.proposal.operation) {
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
</script>
