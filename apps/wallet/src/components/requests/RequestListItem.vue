<template>
  <VCard
    density="compact"
    :min-width="props.minWith"
    :elevation="props.mode === 'grid' ? 0 : undefined"
    :rounded="props.mode === 'grid' ? 0 : undefined"
  >
    <VCardTitle v-if="props.showTitle" class="text-body-2 font-weight-bold">
      {{
        $te(`requests.types.${requestType}.title`)
          ? $t(`requests.types.${requestType}.title`)
          : requestType
      }}
    </VCardTitle>
    <VCardText class="px-4 py-1">
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
import AddAssetOperation from './operations/AddAssetOperation.vue';
import AddAddressBookEntryOperation from './operations/AddAddressBookEntryOperation.vue';
import AddRequestPolicyOperation from './operations/AddRequestPolicyOperation.vue';
import AddUserGroupOperation from './operations/AddUserGroupOperation.vue';
import AddUserOperation from './operations/AddUserOperation.vue';
import CallExternalCanisterOperation from './operations/CallExternalCanisterOperation.vue';
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
import SystemUpgradeOperation from './operations/SystemUpgradeOperation.vue';
import TransferOperation from './operations/TransferOperation.vue';
import UnsupportedOperation from './operations/UnsupportedOperation.vue';
import EditAssetOperation from './operations/EditAssetOperation.vue';
import RemoveAssetOperation from './operations/RemoveAssetOperation.vue';
import AddNamedRuleOperation from './operations/AddNamedRuleOperation.vue';
import EditNamedRuleOperation from './operations/EditNamedRuleOperation.vue';
import RemoveNamedRuleOperation from './operations/RemoveNamedRuleOperation.vue';
import SetDisasterRecoveryOperation from './operations/SetDisasterRecoveryOperation.vue';
const props = withDefaults(
  defineProps<{
    request: Request;
    details: RequestDetails;
    hideColumnBorders?: boolean;
    mode?: 'list' | 'grid';
    showTitle?: boolean;
    minWith?: string;
  }>(),
  {
    hideColumnBorders: false,
    mode: 'list',
    showTitle: true,
    minWith: '240px',
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
  SystemUpgrade: SystemUpgradeOperation,
  EditPermission: EditPermissionOperation,
  ManageSystemInfo: ManageSystemInfoOperation,
  CallExternalCanister: CallExternalCanisterOperation,
  AddAsset: AddAssetOperation,
  EditAsset: EditAssetOperation,
  RemoveAsset: RemoveAssetOperation,
  AddNamedRule: AddNamedRuleOperation,
  EditNamedRule: EditNamedRuleOperation,
  RemoveNamedRule: RemoveNamedRuleOperation,
  SetDisasterRecovery: SetDisasterRecoveryOperation,

  // below variants are not supported yet
  ChangeExternalCanister: UnsupportedOperation,
  CreateExternalCanister: UnsupportedOperation,
  ConfigureExternalCanister: UnsupportedOperation,
  FundExternalCanister: UnsupportedOperation,
  MonitorExternalCanister: UnsupportedOperation,
  PruneExternalCanister: UnsupportedOperation,
  RestoreExternalCanister: UnsupportedOperation,
  SnapshotExternalCanister: UnsupportedOperation,
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

  // if no operation is found, show unsupported operation
  const unknownOperationType = Object.keys(props.request.operation)?.[0];
  return unknownOperationType
    ? {
        component: UnsupportedOperation,
        operation: props.request.operation[unknownOperationType as keyof RequestOperation],
      }
    : null;
});

const requestType = computed(() => {
  const keys = Object.keys(componentsMap) as KeysOfUnion<RequestOperation>[];
  for (const key of keys) {
    if (key in props.request.operation) {
      return key.toLowerCase();
    }
  }
  const unknownOperationType = Object.keys(props.request.operation)?.[0];
  return unknownOperationType ?? 'unknown';
});
</script>
