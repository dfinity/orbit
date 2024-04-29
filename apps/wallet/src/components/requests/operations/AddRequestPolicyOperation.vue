<template>
  <div v-if="isListMode" class="d-flex flex-column ga-0 text-caption">
    <RequestOperationListRow>
      <template #name>{{ $t('terms.specifier') }}</template>
      <template #content>
        {{ requestPolicyType }}
      </template>
    </RequestOperationListRow>
  </div>
  <RequestPolicyForm v-else :model-value="formValue" mode="view" />
</template>

<script setup lang="ts">
import { Ref, computed, onBeforeMount, ref } from 'vue';
import { useI18n } from 'vue-i18n';
import RequestPolicyForm from '~/components/request-policies/RequestPolicyForm.vue';
import RequestOperationListRow from '~/components/requests/RequestOperationListRow.vue';
import { AddRequestPolicyOperation, Request, RequestPolicy } from '~/generated/station/station.did';

const props = withDefaults(
  defineProps<{
    request: Request;
    operation: AddRequestPolicyOperation;
    mode?: 'list' | 'detail';
  }>(),
  {
    mode: 'list',
  },
);

const i18n = useI18n();
const isListMode = computed(() => props.mode === 'list');
const formValue: Ref<Partial<RequestPolicy>> = ref({});

const requestPolicyType = computed(() => {
  const keys = Object.keys(props.operation.input.specifier);
  for (const specifier of keys) {
    return i18n.t(`request_policies.specifier.${specifier.toLowerCase()}`);
  }

  return '-';
});

onBeforeMount(() => {
  const policy: Partial<RequestPolicy> = {};
  policy.specifier = props.operation.input.specifier;
  policy.rule = props.operation.input.rule;

  formValue.value = policy;
});
</script>
