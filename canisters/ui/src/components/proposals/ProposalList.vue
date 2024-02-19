<template>
  <VTable v-if="!app.isMobile" hover class="bg-transparent" density="compact">
    <thead v-if="!props.hideHeaders">
      <tr>
        <th class="font-weight-bold bb-none">{{ $t('terms.type') }}</th>
        <th class="font-weight-bold bb-none">{{ $t('terms.requested') }}</th>
        <th class="bb-none">&nbsp;</th>
      </tr>
    </thead>
    <tbody>
      <template v-if="props.loading">
        <tr>
          <td colspan="3" class="bb-none">
            <VProgressLinear indeterminate color="primary" data-test-id="loading" />
          </td>
        </tr>
      </template>
      <template v-else>
        <ProposalListItem
          v-for="proposal in props.proposals"
          :key="proposal.id"
          :proposal="proposal"
          :details="getDetails(proposal)"
          class="px-1"
          lines="one"
          hide-column-borders
          mode="table"
          @voted="emit('voted', proposal)"
          @opened="emit('opened', proposal)"
          @closed="emit('closed', proposal)"
        />
        <tr v-if="!props.proposals.length && !props.hideNotFound">
          <td colspan="3" class="bb-none" data-test-id="proposals-empty-list">
            {{ notFoundText }}
          </td>
        </tr>
      </template>
    </tbody>
  </VTable>
  <VProgressLinear v-else-if="props.loading" indeterminate color="primary" data-test-id="loading" />
  <VList v-else bg-color="transparent">
    <ProposalListItem
      v-for="proposal in props.proposals"
      :key="proposal.id"
      :proposal="proposal"
      :details="getDetails(proposal)"
      class="px-1"
      lines="one"
      mode="list"
      @voted="emit('voted', proposal)"
      @opened="emit('opened', proposal)"
      @closed="emit('closed', proposal)"
    />
    <VListItem v-if="!props.proposals.length && !props.hideNotFound">
      {{ notFoundText }}
    </VListItem>
  </VList>
</template>
<script setup lang="ts">
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import {
  Proposal,
  ProposalAdditionalInfo,
  ProposalCallerPrivileges,
} from '~/generated/wallet/wallet.did';
import { useAppStore } from '~/stores/app.store';
import ProposalListItem from './ProposalListItem.vue';
import { ProposalDetails } from '~/types/wallet.types';

const app = useAppStore();

const props = withDefaults(
  defineProps<{
    proposals: Proposal[];
    privileges?: ProposalCallerPrivileges[];
    additionals?: ProposalAdditionalInfo[];
    hideHeaders?: boolean;
    notFoundText?: string;
    loading?: boolean;
    hideNotFound?: boolean;
  }>(),
  {
    hideHeaders: false,
    notFoundText: undefined,
    privileges: () => [],
    additionals: () => [],
    loading: false,
    hideNotFound: false,
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
  };
};
</script>
