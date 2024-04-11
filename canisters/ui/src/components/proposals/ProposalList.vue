<template>
  <VProgressLinear v-if="props.loading" indeterminate color="primary" data-test-id="loading" />
  <div v-else :class="{ 'd-flex flex-row ga-1': props.mode === 'grid' }">
    <ProposalListItem
      v-for="proposal in props.proposals"
      :key="proposal.id"
      :proposal="proposal"
      :details="getDetails(proposal)"
      :mode="props.mode"
      @voted="emit('voted', proposal)"
      @opened="emit('opened', proposal)"
      @closed="emit('closed', proposal)"
    />
    <VListItem
      v-if="!props.proposals.length && !props.hideNotFound"
      data-test-id="proposals-empty-list"
    >
      {{ notFoundText }}
    </VListItem>
  </div>
</template>
<script setup lang="ts">
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import { VListItem, VProgressLinear } from 'vuetify/components';
import {
  Proposal,
  ProposalAdditionalInfo,
  ProposalCallerPrivileges,
} from '~/generated/wallet/wallet.did';
import { ProposalDetails } from '~/types/wallet.types';
import ProposalListItem from './ProposalListItem.vue';

const props = withDefaults(
  defineProps<{
    proposals: Proposal[];
    privileges?: ProposalCallerPrivileges[];
    additionals?: ProposalAdditionalInfo[];
    hideHeaders?: boolean;
    notFoundText?: string;
    loading?: boolean;
    hideNotFound?: boolean;
    mode?: 'list' | 'grid';
  }>(),
  {
    hideHeaders: false,
    notFoundText: undefined,
    privileges: () => [],
    additionals: () => [],
    loading: false,
    hideNotFound: false,
    mode: 'list',
  },
);

const emit = defineEmits<{
  (event: 'voted', payload: Proposal): void;
  (event: 'opened', payload: Proposal): void;
  (event: 'closed', payload: Proposal): void;
}>();

const i18n = useI18n();
const notFoundText = computed(() => props.notFoundText || i18n.t('proposals.no_results_found'));

const getDetails = (proposal: Proposal): ProposalDetails => {
  const privileges = props.privileges.find(privilege => privilege.id === proposal.id);
  const info = props.additionals.find(additional => additional.id === proposal.id);

  return {
    can_vote: !!privileges?.can_vote,
    proposer_name: info?.proposer_name?.[0] ?? '',
    voters: info?.voters ?? [],
  };
};
</script>
