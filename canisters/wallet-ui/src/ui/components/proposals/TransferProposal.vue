<template>
  <div class="proposal-item__code__title">
    {{ $t(`banks.proposals.transfer.title`) }}
    <span v-if="account && transfer">
      <small>| {{ account.symbol }}: {{ formatBalance(transfer.amount, account.decimals) }}</small>
    </span>
  </div>
  <div class="proposal-item__code__time">
    <VBtn
      v-if="injectedProps.outer && account"
      :prepend-icon="mdiWallet"
      :to="{ name: 'Account', params: { id: account.id } }"
      size="x-small"
      variant="tonal"
      :append-icon="mdiOpenInApp"
    >
      {{ account?.name?.[0] ? account?.name[0] : $t('terms.account') }}
    </VBtn>
    <VChip size="x-small" :title="proposal.created_at" variant="tonal">
      <VIcon :icon="mdiClockOutline" size="x-small" />&nbsp;
      {{ new Date(proposal.created_at).toLocaleDateString() }}
    </VChip>
    <VChip v-if="transfer && account && !injectedProps.outer" size="x-small">
      {{ $t(`terms.to`) }}: {{ transfer.to }}
    </VChip>
  </div>
</template>
<script lang="ts" setup>
import { mdiClockOutline, mdiWallet, mdiOpenInApp } from '@mdi/js';
import { computed, inject } from 'vue';
import { formatBalance } from '~/core';
import { Proposal } from '~/generated/bank/bank.did';

const props = defineProps<{
  modelValue: Proposal;
}>();

const emit = defineEmits<{
  (event: 'update:modelValue', payload: Proposal): void;
  (event: 'read', payload: boolean): void;
}>();

const injectedProps = inject('bankProposalProps', {
  outer: true,
});

const proposal = computed<Proposal>({
  get: () => props.modelValue,
  set: value => emit('update:modelValue', value),
});

const account = computed(() => proposal.value.operation.Transfer.account);
const transfer = computed(() => proposal.value.operation.Transfer.transfer);
</script>
