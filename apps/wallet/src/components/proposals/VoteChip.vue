<template>
  <VChip
    v-bind="$attrs"
    :size="props.size"
    :class="props.class"
    :color="status.color"
    variant="flat"
    data-test-id="vote-status-chip"
  >
    <slot>
      {{ $t(`terms.${status.name}`) }}
    </slot>
  </VChip>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { ProposalVoteStatus } from '~/generated/station/station.did';
import { variantIs } from '~/utils/helper.utils';

const props = withDefaults(
  defineProps<{
    status: ProposalVoteStatus;
    class?: string;
    size?: string | number;
  }>(),
  {
    class: 'text-lowercase',
    size: 'default',
  },
);

const status = computed(() => {
  if (variantIs(props.status, 'Accepted')) {
    return { name: 'approved', color: 'green' };
  }

  if (variantIs(props.status, 'Rejected')) {
    return { name: 'rejected', color: 'error' };
  }

  return { name: 'unknown', color: 'default' };
});
</script>
