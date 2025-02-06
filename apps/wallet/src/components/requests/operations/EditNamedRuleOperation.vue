<template>
  <div v-if="isListMode" class="d-flex flex-column ga-0 text-caption">
    <RequestOperationListRow v-if="operation.input.name[0]">
      <template #name>{{ $t('terms.name') }}</template>
      <template #content>
        {{ operation.input.name[0] }}
      </template>
    </RequestOperationListRow>
    <RequestOperationListRow v-if="operation.input.description[0]?.[0]">
      <template #name>{{ $t('terms.description') }}</template>
      <template #content>
        {{ operation.input.description[0][0] }}
      </template>
    </RequestOperationListRow>
    <RequestOperationListRow v-if="operation.input.rule[0]">
      <template #name>{{ $t('terms.rule') }}</template>
      <template #content>
        <RuleSummary :rule="operation.input.rule[0]" />
      </template>
    </RequestOperationListRow>
  </div>
  <NamedRuleForm v-else :model-value="formValue" mode="view" />
</template>

<script setup lang="ts">
import { computed, onBeforeMount, Ref, ref } from 'vue';
import NamedRuleForm from '~/components/request-policies/NamedRuleForm.vue';
import RuleSummary from '~/components/request-policies/rule/RuleSummary.vue';
import RequestOperationListRow from '~/components/requests/RequestOperationListRow.vue';
import { EditNamedRuleOperation, NamedRule, Request } from '~/generated/station/station.did';

const props = withDefaults(
  defineProps<{
    request: Request;
    operation: EditNamedRuleOperation;
    mode?: 'list' | 'detail';
  }>(),
  {
    mode: 'list',
  },
);

const isListMode = computed(() => props.mode === 'list');

const formValue: Ref<Partial<NamedRule>> = ref({});

onBeforeMount(() => {
  if (props.operation.input.name[0]) {
    formValue.value.name = props.operation.input.name[0];
  }
  if (props.operation.input.description[0]) {
    formValue.value.description = props.operation.input.description[0];
  }
  if (props.operation.input.rule[0]) {
    formValue.value.rule = props.operation.input.rule[0];
  }
});
</script>
