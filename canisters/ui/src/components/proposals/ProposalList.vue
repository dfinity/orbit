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
          class="px-1"
          lines="one"
          hide-column-borders
          mode="table"
          @voted="$emit('voted', proposal)"
          @opened="$emit('opened', proposal)"
          @closed="$emit('closed', proposal)"
        />
        <tr v-if="!props.proposals.length && !props.hideNotFound">
          <td colspan="3" class="bb-none" data-test-id="proposals-empty-list">
            {{ props.notFoundText }}
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
      class="px-1"
      lines="one"
      mode="list"
      @voted="$emit('voted', proposal)"
      @opened="$emit('opened', proposal)"
      @closed="$emit('closed', proposal)"
    />
    <VListItem v-if="!props.proposals.length && !props.hideNotFound">
      {{ props.notFoundText }}
    </VListItem>
  </VList>
</template>
<script setup lang="ts">
import { Proposal } from '~/generated/wallet/wallet.did';
import { i18n } from '~/plugins/i18n.plugin';
import { useAppStore } from '~/stores/app.store';
import ProposalListItem from './ProposalListItem.vue';

const app = useAppStore();

const props = withDefaults(
  defineProps<{
    proposals: Proposal[];
    hideHeaders?: boolean;
    notFoundText?: string;
    loading?: boolean;
    hideNotFound?: boolean;
  }>(),
  {
    hideHeaders: false,
    notFoundText: i18n.global.t('proposals.no_results_found'),
    loading: false,
    hideNotFound: false,
  },
);

defineEmits<{
  (event: 'voted', payload: Proposal): void;
  (event: 'opened', payload: Proposal): void;
  (event: 'closed', payload: Proposal): void;
}>();
</script>
