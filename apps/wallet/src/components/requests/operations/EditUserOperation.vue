<template>
  <div v-if="isListMode" class="d-flex flex-column ga-0 text-caption">
    <RequestOperationListRow v-if="formValue.id">
      <template #name>{{ $t('terms.id') }}</template>
      <template #content>
        {{ formValue.id }}
      </template>
    </RequestOperationListRow>
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
  <UserForm v-else :model-value="formValue" mode="view" :current-user="currentUser as User" />
</template>

<script setup lang="ts">
import { Ref, computed, onBeforeMount, ref } from 'vue';
import UserForm from '~/components/users/UserForm.vue';
import { EditUserOperation, Request, User } from '~/generated/station/station.did';
import { fromUserStatusVariantToEnum } from '~/mappers/users.mapper';
import RequestOperationListRow from '../RequestOperationListRow.vue';
import { variantIs } from '~/utils/helper.utils';
import { services } from '~/plugins/services.plugin';
import { useAppStore } from '~/stores/app.store';

const props = withDefaults(
  defineProps<{
    request: Request;
    operation: EditUserOperation;
    mode?: 'list' | 'detail';
  }>(),
  {
    mode: 'list',
  },
);

const isListMode = computed(() => props.mode === 'list');
const formValue: Ref<Partial<User & { cancelPendingRequests?: boolean }>> = ref({});

const stationService = services().station;
const appStore = useAppStore();
const currentUser = ref<User | undefined>(undefined);

onBeforeMount(async () => {
  const user: Partial<User> = {};
  user.id = props.operation.input.id;
  user.name = props.operation.input.name?.[0];
  if (props.operation.input.status?.[0]) {
    user.status = props.operation.input.status?.[0];
  }
  if (props.operation.input.groups?.[0]) {
    user.groups = props.operation.input.groups[0].map(id => ({ id, name: id }));
  }
  if (props.operation.input.identities?.[0]) {
    user.identities = props.operation.input.identities[0];
  }

  formValue.value = {
    ...user,
    cancelPendingRequests:
      props.operation.input.cancel_pending_requests?.[0] !== undefined
        ? props.operation.input.cancel_pending_requests[0]
        : undefined,
  };

  if (!isListMode.value && variantIs(props.request.status, 'Created')) {
    try {
      currentUser.value = (
        await stationService.getUser(
          {
            user_id: props.operation.input.id,
          },
          false,
        )
      ).user;
    } catch (error) {
      // TODO: display an error message for the user that they couldn't fetch the user and diff view is not available
      appStore.sendErrorNotification(error);
    }
  }
});
</script>
