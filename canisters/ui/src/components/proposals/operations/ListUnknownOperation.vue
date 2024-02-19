<template>
  <TextOverflow :max-length="48" :text="operationText" />
</template>

<script setup lang="ts">
import { computed } from 'vue';
import TextOverflow from '~/components/TextOverflow.vue';
import { Proposal } from '~/generated/wallet/wallet.did';

const props = defineProps<{
  proposal: Proposal;
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  operation: any;
}>();

const operationText = computed(() => {
  return JSON.stringify(props.operation?.input ?? props.operation, (_, value) =>
    typeof value === 'bigint' ? value.toString() : value,
  );
});
</script>
