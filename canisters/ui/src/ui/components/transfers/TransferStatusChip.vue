<template>
  <VChip :size="chip.size" :color="chip.color" :variant="chip.variant">{{ chip.title }}</VChip>
</template>
<script lang="ts" setup>
import { TransferStatus } from '~/generated/wallet/wallet.did';
import { extractTransferStatus } from '~/core';
import { AccountTransferStatus } from '~/types';
import { computed } from 'vue';

const props = defineProps<{
  status: TransferStatus;
  variant?: 'tonal' | 'outlined';
  size?: 'x-small' | 'small' | 'medium' | 'large';
}>();

const chipColor = (status: AccountTransferStatus): string => {
  switch (status) {
    case AccountTransferStatus.Completed:
      return 'success';
    case AccountTransferStatus.Cancelled:
    case AccountTransferStatus.Failed:
      return 'error';
    case AccountTransferStatus.Processing:
      return 'warning';
    case AccountTransferStatus.Created:
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
