<template>
  <div class="proposal-item__code__title">{{ proposal.operation }}</div>
  <div class="proposal-item__code__time">
    <VChip size="x-small" :title="proposal.created_at">
      <VIcon :icon="mdiClockOutline" size="x-small" />&nbsp;
      {{ new Date(proposal.created_at).toLocaleDateString() }}
    </VChip>
  </div>
</template>
<script lang="ts" setup>
import { mdiClockOutline } from '@mdi/js';
import { computed } from 'vue';
import { Proposal } from '~/generated/wallet/wallet.did';

const props = defineProps<{
  modelValue: Proposal;
}>();

const emit = defineEmits<{
  (event: 'update:modelValue', payload: Proposal): void;
  (event: 'read', payload: boolean): void;
}>();

const proposal = computed({
  get: () => props.modelValue,
  set: value => emit('update:modelValue', value),
});
</script>
