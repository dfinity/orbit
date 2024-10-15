<template>
  <VProgressCircular v-if="!reviewContext" indeterminate :size="20" :width="3" />
  <div v-else-if="isListMode" class="d-flex flex-column ga-0 text-caption">
    <RequestOperationListRow>
      <ReviewCallExternalCanisterOperation :review="reviewContext" />
    </RequestOperationListRow>
  </div>
  <div v-else>
    <ReviewCallExternalCanisterOperation :review="reviewContext" full-review-context />
  </div>
</template>

<script setup lang="ts">
import { Ref, computed, ref, watch } from 'vue';
import { VProgressCircular } from 'vuetify/components';
import { CanisterCallReviewContext } from '~/components/external-canisters/external-canisters.types';
import logger from '~/core/logger.core';
import { CallExternalCanisterOperation, Request } from '~/generated/station/station.did';
import { useStationStore } from '~/stores/station.store';
import { toUint8Array, variantIs } from '~/utils/helper.utils';
import RequestOperationListRow from '../RequestOperationListRow.vue';
import ReviewCallExternalCanisterOperation from '../review/ReviewCallExternalCanisterOperation.vue';

const props = withDefaults(
  defineProps<{
    request: Request;
    operation: CallExternalCanisterOperation;
    mode?: 'list' | 'detail';
  }>(),
  {
    mode: 'list',
  },
);

const isListMode = computed(() => props.mode === 'list');
const reviewContext: Ref<CanisterCallReviewContext | null> = ref(null);
const station = useStationStore();

const fillReviewContext = (operation: CallExternalCanisterOperation): CanisterCallReviewContext => {
  return {
    canisterId: operation.execution_method.canister_id,
    methodName: operation.execution_method.method_name,
    cycles: operation.execution_method_cycles?.[0] ?? undefined,
    argChecksum: operation.arg_checksum?.[0] ?? undefined,
    reply: operation.execution_method_reply?.[0]
      ? toUint8Array(operation.execution_method_reply[0])
      : undefined,
    arg: operation.arg?.[0] ? toUint8Array(operation.arg[0]) : undefined,
    argValidationRendering: operation.arg_rendering?.[0],
  };
};

const loadWithFullInformation = async (): Promise<void> => {
  try {
    const result = await station.service.getRequest({
      request_id: props.request.id,
      with_full_info: [true],
    });

    if (variantIs(result.request.operation, 'CallExternalCanister')) {
      reviewContext.value = fillReviewContext(result.request.operation.CallExternalCanister);
    }
  } catch (err) {
    logger.error(`Error loading full CallExternalCanister request ${props.request.id}`, err);
  }
};

watch(
  () => props.operation,
  operation => {
    reviewContext.value = fillReviewContext(operation);
  },
  { immediate: true },
);

watch(
  isListMode,
  isListMode => {
    if (!isListMode) {
      loadWithFullInformation();
    }
  },
  { immediate: true },
);
</script>
