<template>
  <VChip :size="chip.size" :color="chip.color" :variant="chip.variant">{{ chip.title }}</VChip>
</template>
<script lang="ts" setup>
import { TransferStatus } from '~/generated/bank/bank.did';
import { extractTransferStatus } from '~/core';
import { WalletTransferStatus } from '~/types';
import { computed } from 'vue';

const props = defineProps<{
  status: TransferStatus;
  variant?: 'tonal' | 'outlined';
  size?: 'x-small' | 'small' | 'medium' | 'large';
}>();

const chipColor = (status: WalletTransferStatus): string => {
  switch (status) {
    case WalletTransferStatus.Completed:
      return 'success';
    case WalletTransferStatus.Rejected:
    case WalletTransferStatus.Failed:
      return 'error';
    case WalletTransferStatus.Pending:
      return 'warning';
    case WalletTransferStatus.Approved:
      return 'info';
    default:
      return 'tonal';
  }
};

const chip = computed(() => {
  const status = extractTransferStatus(props.status);

  return {
    title: status,
    variant: props.variant || 'tonal',
    size: props.size || 'small',
    color: chipColor(status),
  };
});
</script>
