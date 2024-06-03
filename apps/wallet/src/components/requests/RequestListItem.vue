<template>
  <VCard
    :elevation="props.mode === 'grid' ? 0 : undefined"
    density="compact"
    class="w-min-25"
    :rounded="props.mode === 'grid' ? 0 : undefined"
    :class="{ 'br-on-background': props.mode === 'grid' }"
  >
    <VCardTitle class="text-body-2 font-weight-bold">
      {{ $t(`requests.types.${requestType}.title`) }}
    </VCardTitle>
    <VCardText class="px-4 pb-1">
      <component
        :is="itemView?.component"
        v-if="itemView"
        :request="props.request"
        :operation="itemView.operation"
        mode="list"
      />
    </VCardText>
    <VCardActions class="px-4">
      <RequestStatusChip size="small" :status="props.request.status" />
      <VSpacer />
      <ReviewRequestBtn
        :request-id="props.request.id"
        :can-approve="props.details.can_approve"
        @approved="$emit('approved')"
        @opened="$emit('opened')"
        @closed="$emit('closed')"
      />
    </VCardActions>
  </VCard>
</template>

<script setup lang="ts">
import type { Component } from 'vue';
import { computed } from 'vue';
import { VCard, VCardActions, VCardText, VCardTitle, VSpacer } from 'vuetify/components';
import { Request, RequestOperation } from '~/generated/station/station.did';
import { RequestDetails } from '~/types/station.types';
import { KeysOfUnion } from '~/utils/helper.utils';
import RequestStatusChip from './RequestStatusChip.vue';
import ReviewRequestBtn from './ReviewRequestBtn.vue';
import AddAccountOperation from './operations/AddAccountOperation.vue';
import AddAddressBookEntryOperation from './operations/AddAddressBookEntryOperation.vue';
import AddRequestPolicyOperation from './operations/AddRequestPolicyOperation.vue';
import AddUserGroupOperation from './operations/AddUserGroupOperation.vue';
import AddUserOperation from './operations/AddUserOperation.vue';
import ChangeCanisterOperation from './operations/ChangeCanisterOperation.vue';
import EditAccountOperation from './operations/EditAccountOperation.vue';
import EditAddressBookEntryOperation from './operations/EditAddressBookEntryOperation.vue';
import EditPermissionOperation from './operations/EditPermissionOperation.vue';
import EditRequestPolicyOperation from './operations/EditRequestPolicyOperation.vue';
import EditUserGroupOperation from './operations/EditUserGroupOperation.vue';
import EditUserOperation from './operations/EditUserOperation.vue';
import ManageSystemInfoOperation from './operations/ManageSystemInfoOperation.vue';
import RemoveAddressBookEntryOperation from './operations/RemoveAddressBookEntryOperation.vue';
import RemoveRequestPolicyOperation from './operations/RemoveRequestPolicyOperation.vue';
import RemoveUserGroupOperation from './operations/RemoveUserGroupOperation.vue';
import TransferOperation from './operations/TransferOperation.vue';
import UnsupportedOperation from './operations/UnsupportedOperation.vue';

const props = withDefaults(
  defineProps<{
    request: Request;
    details: RequestDetails;
    hideColumnBorders?: boolean;
    mode?: 'list' | 'grid';
  }>(),
  {
    hideColumnBorders: false,
    mode: 'list',
  },
);

const componentsMap: {
  [key in KeysOfUnion<RequestOperation>]: Component;
} = {
  AddUserGroup: AddUserGroupOperation,
  AddUser: AddUserOperation,
  EditUser: EditUserOperation,
  EditUserGroup: EditUserGroupOperation,
  AddAccount: AddAccountOperation,
  EditAccount: EditAccountOperation,
  Transfer: TransferOperation,
  AddAddressBookEntry: AddAddressBookEntryOperation,
  EditAddressBookEntry: EditAddressBookEntryOperation,
  RemoveAddressBookEntry: RemoveAddressBookEntryOperation,
  RemoveUserGroup: RemoveUserGroupOperation,
  AddRequestPolicy: AddRequestPolicyOperation,
  EditRequestPolicy: EditRequestPolicyOperation,
  RemoveRequestPolicy: RemoveRequestPolicyOperation,
  ChangeCanister: ChangeCanisterOperation,
  EditPermission: EditPermissionOperation,
  ManageSystemInfo: ManageSystemInfoOperation,
  ChangeManagedCanister: UnsupportedOperation,
};

defineEmits<{
  (event: 'approved'): void;
  (event: 'opened'): void;
  (event: 'closed'): void;
}>();

const itemView = computed<{
  component: Component;
  operation: RequestOperation[keyof RequestOperation];
} | null>(() => {
  const keys = Object.keys(componentsMap) as Array<keyof RequestOperation>;
  for (const key of keys) {
    if (key in props.request.operation) {
      return {
        component: componentsMap[key],
        operation: props.request.operation[key],
      };
    }
  }

  return null;
});

const requestType = computed(() => {
  const keys = Object.keys(componentsMap) as KeysOfUnion<RequestOperation>[];
  for (const key of keys) {
    if (key in props.request.operation) {
      return key.toLowerCase();
    }
  }

  return 'unknown';
});
</script>
