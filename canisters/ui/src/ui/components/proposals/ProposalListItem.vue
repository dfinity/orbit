<template>
  <tr v-if="props.mode === 'table'">
    <td class="text-body-2 w-25" :class="{ 'bb-none': props.hideColumnBorders }">
      {{ $t(`proposals.types.${proposalType}.short_title`) }}
    </td>
    <td class="w-75" :class="{ 'bb-none': props.hideColumnBorders }">
      <component
        :is="listItemProposalComponent"
        v-if="listItemProposalComponent"
        :proposal="proposal"
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
      {{ $t(`proposals.types.${proposalType}.short_title`) }}
    </VListItemTitle>
    <VListItemSubtitle>
      <component
        :is="listItemProposalComponent"
        v-if="listItemProposalComponent"
        :proposal="proposal"
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
import { VListItem } from 'vuetify/components';
import { KeysOfUnion } from '~/core';
import { Proposal, ProposalOperation } from '~/generated/wallet/wallet.did';
import ListItemAddUserGroup from './user-groups/ListItemAddUserGroup.vue';
import ReviewProposalBtn from '~/ui/components/proposals/ReviewProposalBtn.vue';

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
  RemoveUserGroup: VListItem,
  EditUserGroup: VListItem,
  AddUser: VListItem,
  EditUser: VListItem,
  AddAccount: VListItem,
  EditAccount: VListItem,
  AddAccessPolicy: VListItem,
  RemoveAccessPolicy: VListItem,
  EditAccessPolicy: VListItem,
  AddProposalPolicy: VListItem,
  EditProposalPolicy: VListItem,
  RemoveProposalPolicy: VListItem,
  Transfer: VListItem,
  ChangeCanister: VListItem,
};

defineEmits<{
  (event: 'voted'): void;
  (event: 'opened'): void;
  (event: 'closed'): void;
}>();

const listItemProposalComponent = computed(() => {
  const keys = Object.keys(componentsMap) as KeysOfUnion<ProposalOperation>[];
  for (const key of keys) {
    if (key in props.proposal.operation && key in componentsMap) {
      return componentsMap[key];
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
