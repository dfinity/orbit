<template>
  <div v-if="isListMode" class="d-flex flex-column ga-0 text-caption">
    <ProposalOperationListRow>
      <template #name>{{ $t('terms.name') }}</template>
      <template #content>
        {{ props.operation.input.name }}
      </template>
    </ProposalOperationListRow>
  </div>
  <UserGroupForm v-else :model-value="formValue" mode="view" />
</template>

<script setup lang="ts">
import { Ref, computed, onBeforeMount, ref } from 'vue';
import UserGroupForm from '~/components/users/UserGroupForm.vue';
import { AddUserGroupOperation, Proposal, UserGroup } from '~/generated/wallet/wallet.did';
import ProposalOperationListRow from '../ProposalOperationListRow.vue';

const props = withDefaults(
  defineProps<{
    proposal: Proposal;
    operation: AddUserGroupOperation;
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
    name: props.operation.input.name,
  };
});
</script>
