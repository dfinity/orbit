<template>
  <template v-if="complexRuleSummary">
    <VTooltip location="bottom" content-class="white-space-pre-wrap" :text="complexRuleSummary">
      <template #activator="{ props }">
        <span v-bind="props" class="underline-dotted font-weight-bold">
          {{ $t('request_policies.rule_rich_summary.complex_rule') }}
        </span>
      </template>
    </VTooltip>
  </template>
  <template v-else>
    <RuleSummaryItem v-if="populatedRule" :rule="populatedRule" />
  </template>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { usePopulatedRule } from '~/composables/request-policies.composable';
import { RequestPolicyRule } from '~/generated/station/station.did';
import RuleSummaryItem from './RuleSummaryItem.vue';

const input = defineProps<{
  rule: RequestPolicyRule;
}>();

const rule = ref(input.rule);

const { populatedRule, complexRuleSummary } = usePopulatedRule(rule);
</script>
