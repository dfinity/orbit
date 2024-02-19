<template>
  <div v-if="isListMode" class="d-flex flex-column ga-0 text-caption">
    <ProposalOperationListColumn v-if="formValue.name">
      <template #name>{{ $t('terms.name') }}</template>
      <template #content>
        {{ formValue.name?.[0] ?? '-' }}
      </template>
    </ProposalOperationListColumn>
    <ProposalOperationListColumn v-if="formValue.status">
      <template #name>{{ $t('terms.status') }}</template>
      <template #content>
        {{ fromUserStatusVariantToEnum(formValue.status) }}
      </template>
    </ProposalOperationListColumn>
  </div>
  <UserForm v-else :model-value="formValue" mode="view" />
</template>

<script setup lang="ts">
import { Ref, computed, onBeforeMount, ref } from 'vue';
import UserForm from '~/components/users/UserForm.vue';
import { AddUserOperation, Proposal, User } from '~/generated/wallet/wallet.did';
import { fromUserStatusVariantToEnum } from '~/mappers/users.mapper';
import ProposalOperationListColumn from '../ProposalOperationListColumn.vue';

const props = withDefaults(
  defineProps<{
    proposal: Proposal;
    operation: AddUserOperation;
    mode?: 'list' | 'detail';
  }>(),
  {
    mode: 'list',
  },
);

const isListMode = computed(() => props.mode === 'list');
const formValue: Ref<Partial<User>> = ref({});

onBeforeMount(() => {
  const user: Partial<User> = {};
  user.name = props.operation.input.name;
  user.status = props.operation.input.status;
  user.groups = props.operation.input.groups.map(id => ({ id, name: id }));
  user.identities = props.operation.input.identities;

  formValue.value = user;
});
</script>
