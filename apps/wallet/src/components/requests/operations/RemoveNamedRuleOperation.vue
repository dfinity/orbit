<template>
  <div v-if="isListMode" class="d-flex flex-column ga-0 text-caption">
    <RequestOperationListRow v-if="formValue.name">
      <template #name>{{ $t('terms.name') }}</template>
      <template #content>
        {{ formValue.name }}
      </template>
    </RequestOperationListRow>
    <RequestOperationListRow v-if="formValue.description">
      <template #name>{{ $t('terms.description') }}</template>
      <template #content>
        {{ formValue.description[0] }}
      </template>
    </RequestOperationListRow>
    <RequestOperationListRow v-if="formValue.rule">
      <template #name>{{ $t('terms.rule') }}</template>
      <template #content>
        <RuleSummary :rule="formValue.rule" />
      </template>
    </RequestOperationListRow>
    <RequestOperationListRow v-if="isCompleted">
      <template #name>{{ $t('terms.id') }}</template>
      <template #content>
        {{ formValue.id }}
      </template>
    </RequestOperationListRow>
  </div>
  <VProgressCircular v-else-if="loading" />
  <NamedRuleForm v-else :model-value="formValue" mode="view" />
</template>

<script setup lang="ts">
import { Ref, computed, onBeforeMount, ref } from 'vue';
import logger from '~/core/logger.core';
import { Request, NamedRule, RemoveNamedRuleOperation } from '~/generated/station/station.did';
import { useStationStore } from '~/stores/station.store';
import RequestOperationListRow from '../RequestOperationListRow.vue';
import NamedRuleForm from '~/components/request-policies/NamedRuleForm.vue';
import RuleSummary from '~/components/request-policies/rule/RuleSummary.vue';
import { variantIs } from '~/utils/helper.utils';

const props = withDefaults(
  defineProps<{
    request: Request;
    operation: RemoveNamedRuleOperation;
    mode?: 'list' | 'detail';
  }>(),
  {
    mode: 'list',
  },
);

const isListMode = computed(() => props.mode === 'list');
const formValue: Ref<Partial<NamedRule>> = ref({});
const loading = ref(false);
const station = useStationStore();

const isCompleted = computed(() => variantIs(props.request.status, 'Completed'));

const fetchDetails = async () => {
  try {
    if (loading.value || isCompleted.value) {
      return;
    }

    loading.value = true;
    const { named_rule } = await station.service.getNamedRule(props.operation.input.named_rule_id);

    formValue.value = named_rule;
  } catch (e) {
    logger.error('Failed to fetch named rule details', e);
  } finally {
    loading.value = false;
  }
};

onBeforeMount(() => {
  formValue.value = {
    id: props.operation.input.named_rule_id,
  };

  fetchDetails();
});
</script>
