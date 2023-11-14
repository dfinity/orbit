<template>
  <div class="proposal-item">
    <div v-if="props.loading" class="proposal-item__loading"></div>
    <div class="proposal-item__code">
      <TransferProposal
        v-if="WalletProposalType.Transfer in proposal.operation"
        v-model="proposal"
      />
      <UnknownProposal v-else v-model="proposal" />
    </div>
    <div v-if="props.loading" class="proposal-item__action">
      <VProgressCircular indeterminate color="primary" size="small" class="mx-4" />
    </div>
    <div v-else class="proposal-item__action">
      <VMenu v-if="!voteState.decided && !props.outer" :close-on-content-click="false">
        <template #activator="{ props: actionProps }">
          <VBtn v-bind="actionProps" :prepend-icon="mdiCogs" size="small" variant="text" block>
            {{ $t(`terms.edit`) }}
          </VBtn>
        </template>
        <VList density="compact" :lines="false" class="py-0">
          <VListItem density="compact" class="px-1">
            <VBtn
              :prepend-icon="mdiClose"
              size="small"
              color="error"
              variant="tonal"
              block
              @click="onReject"
            >
              {{ $t(`terms.reject`) }}
            </VBtn>
          </VListItem>
          <VListItem density="compact" class="px-1">
            <VBtn
              :prepend-icon="mdiCheck"
              size="small"
              color="success"
              variant="tonal"
              block
              @click="onApprove"
            >
              {{ $t(`terms.approve`) }}
            </VBtn>
          </VListItem>
        </VList>
      </VMenu>
      <VChip
        v-if="voteState.decided || props.outer"
        :prepend-icon="proposalState.chip.icon"
        size="x-small"
        :color="proposalState.chip.color"
        variant="tonal"
      >
        {{ proposalState.chip.text }}
      </VChip>
    </div>
  </div>
</template>
<script lang="ts" setup>
import { mdiCheck, mdiClose, mdiCogs, mdiCog, mdiHelp } from '@mdi/js';
import { computed, provide } from 'vue';
import { Proposal } from '~/generated/wallet/wallet.did';
import { i18n } from '~/ui/modules';
import UnknownProposal from './UnknownProposal.vue';
import { WalletProposalType } from '~/types';
import TransferProposal from './TransferProposal.vue';
import { useActiveWalletStore } from '~/ui/stores';

const activeWallet = useActiveWalletStore();
const props = withDefaults(
  defineProps<{
    proposal: Proposal;
    outer?: boolean;
    loading?: boolean;
  }>(),
  {
    outer: true,
    loading: false,
  },
);

provide('walletProposalProps', { outer: props.outer });

const emit = defineEmits<{
  (event: 'update:proposal', payload: Proposal): void;
  (event: 'adopted'): void;
  (event: 'rejected'): void;
}>();

const proposal = computed({
  get: () => props.proposal,
  set: value => emit('update:proposal', value),
});

const vote = computed({
  get: () => proposal.value.votes.find(d => d.user_id === activeWallet.user.id),
  set: value => {
    proposal.value.votes.forEach(d => {
      if (d.user_id === activeWallet.user.id && value) {
        d = value;
      }
    });
  },
});

const onApprove = () => {
  emit('adopted');
};

const onReject = () => {
  emit('rejected');
};

const proposalState = computed(() => {
  let chip: { color: string; text: string; icon: string } = {
    color: 'info',
    text: i18n.global.t('terms.abstained'),
    icon: mdiHelp,
  };
  if ('Adopted' in proposal.value.status) {
    chip = {
      color: 'success',
      text: i18n.global.t('terms.approved'),
      icon: mdiCheck,
    };
  } else if ('Rejected' in proposal.value.status) {
    chip = {
      color: 'error',
      text: i18n.global.t('terms.rejected'),
      icon: mdiClose,
    };
  } else if ('Pending' in proposal.value.status) {
    chip = {
      color: 'warning',
      text: i18n.global.t('terms.pending'),
      icon: mdiCog,
    };
  }

  return {
    isPending: 'Pending' in proposal.value.status,
    chip,
  };
});

const voteState = computed(() => {
  const state: { decided: boolean } = {
    decided: false,
  };

  if (!vote.value) {
    return { decided: true };
  }

  if (vote.value && !('Pending' in vote.value.status)) {
    state.decided = true;
  }

  return state;
});
</script>
<style lang="scss">
.proposal-item {
  display: flex;
  flex-direction: row;
  align-items: center;
  gap: var(--ds-bdu);
  position: relative;

  &__loading {
    position: absolute;
    width: 100%;
    height: 100%;
    background: rgb(var(--ds-background));
    opacity: 0.4;
    z-index: 1;
  }

  &__action {
    flex: 0 0 auto;
  }

  &__action {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: var(--ds-bdu);
  }

  &__code {
    flex: 1 1 auto;
    display: flex;
    flex-direction: column;
    justify-content: center;
    gap: calc(var(--ds-bdu) / 2);
    border-right: var(--ds-border-width) var(--ds-border-style) rgb(var(--ds-background));

    &__title {
      font-weight: 500;
      text-transform: capitalize;
      font-size: var(--ds-font-size-xs);
      line-height: 20px;
      color: var(--ds-text-primary);
      white-space: nowrap;
    }

    &__time {
      display: flex;
      white-space: nowrap;
      font-weight: 400;
      font-size: var(--ds-font-size-xxs);
      line-height: var(--ds-font-size-xxs);
      color: var(--ds-text-secondary);
      gap: calc(var(--ds-bdu) / 2);
    }
  }
}
</style>
