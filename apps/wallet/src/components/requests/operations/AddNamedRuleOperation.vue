<template>
  <div v-if="isListMode" class="d-flex flex-column ga-0 text-caption">
    <RequestOperationListRow>
      <template #name>{{ $t('terms.name') }}</template>
      <template #content>
        {{ operation.input.name }}
      </template>
    </RequestOperationListRow>
    <RequestOperationListRow>
      <template #name>{{ $t('terms.description') }}</template>
      <template #content>
        {{ operation.input.description[0] }}
      </template>
    </RequestOperationListRow>
    <RequestOperationListRow>
      <template #name>{{ $t('terms.rule') }}</template>
      <template #content>
        <RuleSummary :rule="operation.input.rule" />
      </template>
    </RequestOperationListRow>
  </div>
  <NamedRuleForm v-else :model-value="operation.input" mode="view" />
</template>

<script setup lang="ts">
import { computed } from 'vue';
import NamedRuleForm from '~/components/request-policies/NamedRuleForm.vue';
import RuleSummary from '~/components/request-policies/rule/RuleSummary.vue';
import RequestOperationListRow from '~/components/requests/RequestOperationListRow.vue';
import { AddNamedRuleOperation, Request } from '~/generated/station/station.did';

const props = withDefaults(
  defineProps<{
    request: Request;
    operation: AddNamedRuleOperation;
    mode?: 'list' | 'detail';
  }>(),
  {
    mode: 'list',
  },
);

const isListMode = computed(() => props.mode === 'list');
</script>
