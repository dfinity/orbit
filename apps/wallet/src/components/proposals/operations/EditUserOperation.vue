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
        {{ formValue.name?.[0] ?? '-' }}
      </template>
    </ProposalOperationListRow>
    <ProposalOperationListRow v-if="formValue.status">
      <template #name>{{ $t('terms.status') }}</template>
      <template #content>
        {{ fromUserStatusVariantToEnum(formValue.status) }}
      </template>
    </ProposalOperationListRow>
  </div>
  <UserForm v-else :model-value="formValue" mode="view" />
</template>

<script setup lang="ts">
import { Ref, computed, onBeforeMount, ref } from 'vue';
import UserForm from '~/components/users/UserForm.vue';
import { EditUserOperation, Proposal, User } from '~/generated/station/station.did';
import { fromUserStatusVariantToEnum } from '~/mappers/users.mapper';
import ProposalOperationListRow from '../ProposalOperationListRow.vue';

const props = withDefaults(
  defineProps<{
    proposal: Proposal;
    operation: EditUserOperation;
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
  user.id = props.operation.input.id;
  user.name = props.operation.input.name;
  if (props.operation.input.status?.[0]) {
    user.status = props.operation.input.status?.[0];
  }
  if (props.operation.input.groups?.[0]) {
    user.groups = props.operation.input.groups[0].map(id => ({ id, name: id }));
  }
  if (props.operation.input.identities?.[0]) {
    user.identities = props.operation.input.identities[0];
  }

  formValue.value = user;
});
</script>
