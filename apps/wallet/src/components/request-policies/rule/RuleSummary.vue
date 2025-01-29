<template>
  <template v-if="isComplex">
    <VTooltip v-if="tooltip" location="bottom" content-class="white-space-pre-wrap" :text="tooltip">
      <template #activator="{ props }">
        <span v-bind="props" class="underline-dotted cursor-help"> Complex rule </span>
      </template>
    </VTooltip>
  </template>
  <template v-else>
    <span>{{ summary }}</span>
  </template>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { useRuleToShortSummary } from '~/composables/request-policies.composable';
import { RequestPolicyRule } from '~/generated/station/station.did';

const input = defineProps<{
  rule: RequestPolicyRule;
}>();

const rule = ref(input.rule);

const { summary, isComplex, tooltip } = useRuleToShortSummary(rule);
</script>
