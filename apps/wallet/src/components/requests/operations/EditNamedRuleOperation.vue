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
  <template v-else>
    <VAlert
      v-if="currentNamedRuleFailed"
      type="error"
      variant="tonal"
      density="compact"
      class="mb-4"
    >
      {{ $t('requests.failed_to_fetch_details') }}
      <div>{{ currentNamedRuleFailed }}</div>
    </VAlert>
    <NamedRuleForm :model-value="formValue" mode="view" :current-named-rule="currentNamedRule" />
  </template>
</template>

<script setup lang="ts">
import { computed, onBeforeMount, Ref, ref } from 'vue';
import NamedRuleForm from '~/components/request-policies/NamedRuleForm.vue';
import RuleSummary from '~/components/request-policies/rule/RuleSummary.vue';
import RequestOperationListRow from '~/components/requests/RequestOperationListRow.vue';
import { EditNamedRuleOperation, NamedRule, Request } from '~/generated/station/station.did';
import { useStationStore } from '~/stores/station.store';
import { useAppStore } from '~/stores/app.store';
import { variantIs } from '~/utils/helper.utils';
import { getErrorMessage } from '~/utils/error.utils';

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
const isDiffMode = computed(() => !isListMode.value && variantIs(props.request.status, 'Created'));
const formValue: Ref<Partial<NamedRule>> = ref({});
const currentNamedRule: Ref<NamedRule | undefined> = ref();
const currentNamedRuleFailed = ref<string | undefined>();
const station = useStationStore();
const appStore = useAppStore();

onBeforeMount(async () => {
  formValue.value = {
    id: props.operation.input.named_rule_id,
    ...(props.operation.input.name[0] && { name: props.operation.input.name[0] }),
    ...(props.operation.input.description[0] && {
      description: props.operation.input.description[0],
    }),
    ...(props.operation.input.rule[0] && { rule: props.operation.input.rule[0] }),
  };

  if (isDiffMode.value) {
    try {
      const { named_rule } = await station.service.getNamedRule(
        props.operation.input.named_rule_id,
      );

      if (!formValue.value.description) {
        formValue.value.description = named_rule.description;
      }

      currentNamedRule.value = named_rule;
    } catch (e) {
      currentNamedRuleFailed.value = getErrorMessage(e);
      appStore.sendErrorNotification(e);
    }
  }
});
</script>
