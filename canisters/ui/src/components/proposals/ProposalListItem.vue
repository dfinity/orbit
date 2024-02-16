<template>
  <tr v-if="props.mode === 'table'">
    <td class="text-body-2 w-25" :class="{ 'bb-none': props.hideColumnBorders }">
      {{ $t(`proposals.types.${proposalType}.title`) }}
    </td>
    <td class="w-75" :class="{ 'bb-none': props.hideColumnBorders }">
      <component
        :is="itemView?.component"
        v-if="itemView"
        :proposal="proposal"
        :operation="itemView.operation"
      />
    </td>
    <td class="d-flex justify-end align-center" :class="{ 'bb-none': props.hideColumnBorders }">
      <ReviewProposalBtn
        :proposal="proposal"
        @voted="$emit('voted')"
        @opened="$emit('opened')"
        @closed="$emit('closed')"
      />
    </td>
  </tr>
  <VListItem v-else>
    <VListItemTitle class="text-body-2 font-weight-bold">
      {{ $t(`proposals.types.${proposalType}.title`) }}
    </VListItemTitle>
    <VListItemSubtitle>
      <component
        :is="itemView?.component"
        v-if="itemView"
        :proposal="proposal"
        :operation="itemView.operation"
      />
    </VListItemSubtitle>
    <template #append>
      <ReviewProposalBtn
        :proposal="proposal"
        @voted="$emit('voted')"
        @opened="$emit('opened')"
        @closed="$emit('closed')"
      />
    </template>
  </VListItem>
</template>

<script setup lang="ts">
import type { Component } from 'vue';
import { computed } from 'vue';
import ReviewProposalBtn from './ReviewProposalBtn.vue';
import { Proposal, ProposalOperation } from '~/generated/wallet/wallet.did';
import { KeysOfUnion } from '~/utils/helper.utils';
import ListItemAddUserGroup from './user-groups/ListItemAddUserGroup.vue';
import ListUnknownOperation from './operations/ListUnknownOperation.vue';
import ListTransfer from './operations/ListTransfer.vue';

const props = withDefaults(
  defineProps<{
    proposal: Proposal;
    hideColumnBorders?: boolean;
    mode?: 'list' | 'table';
  }>(),
  {
    hideColumnBorders: false,
    mode: 'table',
  },
);

const componentsMap: {
  [key in KeysOfUnion<ProposalOperation>]: Component;
} = {
  AddUserGroup: ListItemAddUserGroup,
  RemoveUserGroup: ListUnknownOperation,
  EditUserGroup: ListUnknownOperation,
  AddUser: ListUnknownOperation,
  EditUser: ListUnknownOperation,
  AddAccount: ListUnknownOperation,
  EditAccount: ListUnknownOperation,
  AddAccessPolicy: ListUnknownOperation,
  RemoveAccessPolicy: ListUnknownOperation,
  EditAccessPolicy: ListUnknownOperation,
  AddProposalPolicy: ListUnknownOperation,
  EditProposalPolicy: ListUnknownOperation,
  RemoveProposalPolicy: ListUnknownOperation,
  Transfer: ListTransfer,
  ChangeCanister: ListUnknownOperation,
  AddAddressBookEntry: ListUnknownOperation,
  EditAddressBookEntry: ListUnknownOperation,
  RemoveAddressBookEntry: ListUnknownOperation,
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
