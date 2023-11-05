<template>
  <VCard :width="mobile ? '100%' : '400px'">
    <VList density="compact">
      <VListItem density="compact" class="notifications-panel__title">
        {{ $t('banks.pending_proposals') }}
        <VSpacer />
        <VBtn :icon="mdiClose" variant="flat" @click="emit('close')" />
      </VListItem>
    </VList>
    <VDivider />
    <VList density="compact">
      <VListItem v-if="!activeBank.hasPendingProposals" class="text-center">
        {{ $t('terms.all_done') }}
      </VListItem>
      <VListItem v-for="({ loading, data }, idx) in activeBank.sortedPendingProposals" :key="idx">
        <BankProposal
          :loading="loading"
          :proposal="activeBank.sortedPendingProposals[idx].data"
          @read="read => onRead(data, read)"
          @adopted="submitDecision(data, true)"
          @rejected="submitDecision(data, false)"
        />
        <VDivider v-if="activeBank.pendingProposals.items.length - 1 !== idx" class="mt-4" />
      </VListItem>
    </VList>
  </VCard>
</template>

<script lang="ts" setup>
import { mdiClose } from '@mdi/js';
import { useActiveBankStore } from '~/ui/stores';
import BankProposal from './proposals/BankProposal.vue';
import { Proposal } from '~/generated/bank/bank.did';
import { useDisplay } from 'vuetify';

const { mobile } = useDisplay();
const activeBank = useActiveBankStore();

const emit = defineEmits<{
  (event: 'close'): void;
}>();

const onRead = (proposal: Proposal, read: boolean) =>
  activeBank.saveDecision(proposal.id, { read });

const submitDecision = (proposal: Proposal, approve: boolean) =>
  activeBank.saveDecision(proposal.id, { approve });
</script>

<style lang="scss">
.notifications-panel {
  &__title {
    .v-list-item__content {
      display: flex;
      flex-direction: row;
      align-items: center;
    }
  }
}
</style>
