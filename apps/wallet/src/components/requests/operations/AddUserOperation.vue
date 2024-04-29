<template>
  <div v-if="isListMode" class="d-flex flex-column ga-0 text-caption">
    <RequestOperationListRow v-if="formValue.name">
      <template #name>{{ $t('terms.name') }}</template>
      <template #content>
        {{ formValue.name }}
      </template>
    </RequestOperationListRow>
    <RequestOperationListRow v-if="formValue.status">
      <template #name>{{ $t('terms.status') }}</template>
      <template #content>
        {{ fromUserStatusVariantToEnum(formValue.status) }}
      </template>
    </RequestOperationListRow>
  </div>
  <UserForm v-else :model-value="formValue" mode="view" />
</template>

<script setup lang="ts">
import { Ref, computed, onBeforeMount, ref } from 'vue';
import UserForm from '~/components/users/UserForm.vue';
import { AddUserOperation, Request, User } from '~/generated/station/station.did';
import { fromUserStatusVariantToEnum } from '~/mappers/users.mapper';
import RequestOperationListRow from '../RequestOperationListRow.vue';

const props = withDefaults(
  defineProps<{
    request: Request;
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
