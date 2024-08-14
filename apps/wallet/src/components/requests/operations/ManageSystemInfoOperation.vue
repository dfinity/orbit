<template>
  <div v-if="isListMode" class="d-flex flex-column ga-0 text-caption">
    <RequestOperationListRow v-if="name">
      <template #name>{{ $t('terms.name') }}</template>
      <template #content>
        {{ name }}
      </template>
    </RequestOperationListRow>
    <RequestOperationListRow v-if="cycleObtainStartegySummary">
      <template #name>{{ $t('terms.cycle_obtain_strategy') }}</template>
      <template #content>
        {{ cycleObtainStartegySummary }}
      </template>
    </RequestOperationListRow>
  </div>
  <ManageSystemInfoForm v-else :model-value="props.operation.input" mode="view" />
</template>

<script setup lang="ts">
import { computed } from 'vue';
import ManageSystemInfoForm from '~/components/settings/ManageSystemInfoForm.vue';
import { ManageSystemInfoOperation, Request } from '~/generated/station/station.did';
import { cycleObtainStrategyToSummary } from '~/mappers/obtain-cycles.mapper';
import RequestOperationListRow from '../RequestOperationListRow.vue';

const props = withDefaults(
  defineProps<{
    request: Request;
    operation: ManageSystemInfoOperation;
    mode?: 'list' | 'detail';
  }>(),
  {
    mode: 'list',
  },
);

const isListMode = computed(() => props.mode === 'list');

const name = computed(() => props.operation.input.name?.[0] ?? '');
const cycleObtainStartegySummary = computed(() =>
  props.operation.input.cycle_obtain_strategy[0]
    ? cycleObtainStrategyToSummary(props.operation.input.cycle_obtain_strategy[0])
    : null,
);
</script>
