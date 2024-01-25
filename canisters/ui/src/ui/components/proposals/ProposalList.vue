<template>
  <VTable hover class="bg-transparent" density="compact">
    <thead v-if="!props.hideHeaders">
      <tr>
        <th class="font-weight-bold">{{ $t('terms.type') }}</th>
        <th class="font-weight-bold">{{ $t('terms.requested') }}</th>
        <th class="bb-none">&nbsp;</th>
      </tr>
    </thead>
    <tbody>
      <ProposalListItem
        v-for="proposal in props.proposals"
        :key="proposal.id"
        :proposal="proposal"
        class="px-1"
        lines="one"
        hide-column-borders
        @voted="$emit('voted', proposal)"
        @opened="$emit('opened', proposal)"
        @closed="$emit('closed', proposal)"
      />
    </tbody>
  </VTable>
</template>
<script setup lang="ts">
import { Proposal } from '~/generated/wallet/wallet.did';
import ProposalListItem from './ProposalListItem.vue';

const props = withDefaults(
  defineProps<{
    proposals: Proposal[];
    hideHeaders?: boolean;
  }>(),
  {
    hideHeaders: false,
  },
);

defineEmits<{
  (event: 'voted', payload: Proposal): void;
  (event: 'opened', payload: Proposal): void;
  (event: 'closed', payload: Proposal): void;
}>();
</script>
