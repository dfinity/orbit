<template>
  <div v-if="isListMode" class="d-flex flex-column ga-0 text-caption">
    <RequestOperationListRow v-if="formValue.name?.[0]">
      <template #name>{{ $t('terms.name') }}</template>
      <template #content>
        {{ formValue.name[0] }}
      </template>
    </RequestOperationListRow>
  </div>
  <ManageSystemInfoForm v-else :model-value="formValue" mode="view" />
</template>

<script setup lang="ts">
import { Ref, computed, onBeforeMount, ref } from 'vue';
import ManageSystemInfoForm from '~/components/settings/ManageSystemInfoForm.vue';
import {
  ManageSystemInfoOperation,
  ManageSystemInfoOperationInput,
  Request,
} from '~/generated/station/station.did';
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
const formValue: Ref<Partial<ManageSystemInfoOperationInput>> = ref({});

onBeforeMount(() => {
  const operation: Partial<ManageSystemInfoOperationInput> = {};
  operation.name = props.operation.input.name;

  formValue.value = operation;
});
</script>
