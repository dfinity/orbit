<template>
  <div class="pa-2 bg-background" style="border: var(--ds-border-on-background)">
    <VueJsonPretty
      :data="props.operation?.input ?? props.operation"
      :editable="false"
      :show-line="false"
      :highlight-selected-node="false"
      :show-double-quotes="false"
    />
  </div>
</template>

<script setup lang="ts">
import { Proposal } from '~/generated/wallet/wallet.did';
import VueJsonPretty from 'vue-json-pretty';
import 'vue-json-pretty/lib/styles.css';

const props = defineProps<{
  proposal: Proposal;
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  operation: any;
}>();
</script>
