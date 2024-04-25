<template>
  <div v-if="isListMode" class="d-flex flex-column ga-0 text-caption">
    <ProposalOperationListRow v-if="formValue.id">
      <template #name>{{ $t('terms.id') }}</template>
      <template #content>
        {{ formValue.id }}
      </template>
    </ProposalOperationListRow>
    <ProposalOperationListRow v-if="formValue.name">
      <template #name>{{ $t('terms.name') }}</template>
      <template #content>
        {{ formValue.name }}
      </template>
    </ProposalOperationListRow>
  </div>
  <UserGroupForm v-else :model-value="formValue" mode="view" />
</template>

<script setup lang="ts">
import { Ref, computed, onBeforeMount, ref } from 'vue';
import UserGroupForm from '~/components/users/UserGroupForm.vue';
import { EditUserGroupOperation, Proposal, UserGroup } from '~/generated/station/station.did';
import ProposalOperationListRow from '../ProposalOperationListRow.vue';

const props = withDefaults(
  defineProps<{
    proposal: Proposal;
    operation: EditUserGroupOperation;
    mode?: 'list' | 'detail';
  }>(),
  {
    mode: 'list',
  },
);

const isListMode = computed(() => props.mode === 'list');
const formValue: Ref<Partial<UserGroup>> = ref({});

onBeforeMount(() => {
  formValue.value = {
    id: props.operation.input.user_group_id,
    name: props.operation.input.name,
  };
});
</script>
