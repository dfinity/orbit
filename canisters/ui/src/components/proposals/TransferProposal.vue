<template>
  <div class="proposal-item__code__title">
    {{ $t(`wallets.proposals.transfer.title`) }}
    <span>
      <small>
        | {{ operation.from_account?.[0]?.symbol }}:
        {{ formatBalance(operation.input.amount, operation.from_account?.[0]?.decimals ?? 0) }}
      </small>
    </span>
  </div>
  <div class="proposal-item__code__time">
    <VBtn
      v-if="injectedProps.outer"
      :prepend-icon="mdiWallet"
      :to="{ name: Routes.Account, params: { id: operation.from_account?.[0]?.id } }"
      size="x-small"
      variant="tonal"
      :append-icon="mdiOpenInApp"
    >
      {{ operation.from_account?.[0]?.name }}
    </VBtn>
    <VChip size="x-small" :title="proposal.created_at" variant="tonal">
      <VIcon :icon="mdiClockOutline" size="x-small" />&nbsp;
      {{ new Date(proposal.created_at).toLocaleDateString() }}
    </VChip>
    <VChip v-if="!injectedProps.outer" size="x-small">
      {{ $t(`terms.to`) }}: {{ operation.input.to }}
    </VChip>
  </div>
</template>
<script lang="ts" setup>
import { mdiClockOutline, mdiOpenInApp, mdiWallet } from '@mdi/js';
import { computed, inject } from 'vue';
import { Routes } from '~/configs/routes.config';
import { Proposal, TransferOperation } from '~/generated/wallet/wallet.did';
import { formatBalance } from '~/utils/helper.utils';

const props = defineProps<{
  modelValue: Proposal;
  operation: TransferOperation;
}>();

const emit = defineEmits<{
  (event: 'update:modelValue', payload: Proposal): void;
  (event: 'read', payload: boolean): void;
}>();

const injectedProps = inject('walletProposalProps', {
  outer: true,
});

const proposal = computed<Proposal>({
  get: () => props.modelValue,
  set: value => emit('update:modelValue', value),
});
</script>
