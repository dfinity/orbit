<template>
  <div v-if="isListMode" class="d-flex flex-column ga-0 text-caption">
    <RequestOperationListRow v-if="formValue.id">
      <template #name>{{ $t('terms.id') }}</template>
      <template #content>
        {{ formValue.id }}
      </template>
    </RequestOperationListRow>
    <RequestOperationListRow v-if="requestPolicyType">
      <template #name>{{ $t('terms.specifier') }}</template>
      <template #content>
        {{ requestPolicyType }}
      </template>
    </RequestOperationListRow>
  </div>
  <VProgressCircular v-else-if="loading" />
  <RequestPolicyForm
    v-else
    :model-value="formValue"
    mode="view"
    :current-request-policy="currentRequestPolicy"
  />
</template>

<script setup lang="ts">
import { Ref, computed, onBeforeMount, ref } from 'vue';
import logger from '~/core/logger.core';
import {
  EditRequestPolicyOperation,
  Request,
  RequestPolicy,
} from '~/generated/station/station.did';
import { useStationStore } from '~/stores/station.store';
import RequestOperationListRow from '../RequestOperationListRow.vue';
import RequestPolicyForm from '~/components/request-policies/RequestPolicyForm.vue';
import { useI18n } from 'vue-i18n';
import { useAppStore } from '~/stores/app.store';

const props = withDefaults(
  defineProps<{
    request: Request;
    operation: EditRequestPolicyOperation;
    mode?: 'list' | 'detail';
  }>(),
  {
    mode: 'list',
  },
);

const isListMode = computed(() => props.mode === 'list');
const formValue: Ref<Partial<RequestPolicy>> = ref({});
const currentRequestPolicy: Ref<RequestPolicy | undefined> = ref();
const loading = ref(false);
const station = useStationStore();
const appStore = useAppStore();

const fetchDetails = async () => {
  try {
    if (loading.value || isListMode.value) {
      return;
    }

    loading.value = true;
    const currentEntry = await station.service.getRequestPolicy(props.operation.input.policy_id);
    if (formValue.value.rule) {
      currentEntry.policy.rule = formValue.value.rule;
    }
    if (formValue.value.specifier) {
      currentEntry.policy.specifier = formValue.value.specifier;
    }

    formValue.value = currentEntry.policy;
  } catch (e) {
    logger.error('Failed to fetch request policy details', e);
  } finally {
    loading.value = false;
  }
};

const i18n = useI18n();
const requestPolicyType = computed(() => {
  const keys = Object.keys(formValue.value.specifier ?? {});
  for (const specifier of keys) {
    return i18n.t(`request_policies.specifier.${specifier.toLowerCase()}`);
  }

  return undefined;
});

onBeforeMount(async () => {
  const policy: Partial<RequestPolicy> = {};
  policy.id = props.operation.input.policy_id;
  if (props.operation.input.rule?.[0]) {
    policy.rule = props.operation.input.rule[0];
  }
  if (props.operation.input.specifier?.[0]) {
    policy.specifier = props.operation.input.specifier[0];
  }

  formValue.value = policy;
  // load full existing policy for diff
  try {
    const { policy } = await station.service.getRequestPolicy(props.operation.input.policy_id);
    currentRequestPolicy.value = policy;
  } catch (e) {
    logger.error('Failed to fetch current request policy', e);
    appStore.sendErrorNotification(e);
  }
  fetchDetails();
});
</script>
