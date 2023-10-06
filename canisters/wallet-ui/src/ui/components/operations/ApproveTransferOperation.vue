<template>
  <div class="operation-item__code__title">
    {{ $t(`banks.operations.approve_transfer.title`) }}
    <span v-if="wallet"
      ><small>| {{ wallet.asset_symbol }}</small></span
    >
  </div>
  <div class="operation-item__code__time">
    <VBtn
      v-if="injectedProps.outer && wallet"
      :prepend-icon="mdiWallet"
      :to="{ name: 'WalletDetails', params: { id: wallet.id } }"
      size="x-small"
      variant="tonal"
      :append-icon="mdiOpenInApp"
    >
      {{ wallet?.name?.[0] ? wallet?.name[0] : $t('terms.wallet') }}
    </VBtn>
    <VChip size="x-small" :title="operation.created_at" variant="tonal">
      <VIcon :icon="mdiClockOutline" size="x-small" />&nbsp;
      {{ new Date(operation.created_at).toLocaleDateString() }}
    </VChip>
    <VChip v-for="(detail, idx) in detailChips" :key="idx" size="x-small">
      {{ detail.title }}: {{ detail.description }}
    </VChip>
  </div>
</template>
<script lang="ts" setup>
import { mdiClockOutline, mdiWallet, mdiOpenInApp } from '@mdi/js';
import { computed, inject } from 'vue';
import { Operation } from '~/generated/bank/bank.did';
import { useActiveBankStore } from '~/ui/stores';

const props = defineProps<{
  modelValue: Operation;
}>();

const emit = defineEmits<{
  (event: 'update:modelValue', payload: Operation): void;
  (event: 'read', payload: boolean): void;
}>();

const injectedProps = inject('bankOperationProps', {
  outer: true,
  details: undefined,
});

const detailChips = computed(() => {
  return Object.entries(injectedProps.details ?? {}).map(([k, v]) => {
    return {
      title: k,
      description: v,
    };
  });
});

const operation = computed({
  get: () => props.modelValue,
  set: value => emit('update:modelValue', value),
});

const activeBank = useActiveBankStore();

const wallet = computed(() => {
  const walletId = operation.value.metadata.find(([k, _]) => k === 'wallet_id');

  if (!walletId) {
    return null;
  }

  return activeBank.wallets.items.find(({ id }) => id === walletId[1]) ?? null;
});
</script>
