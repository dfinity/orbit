<template>
  <VCard>
    <VToolbar color="background" height="auto">
      <VToolbarTitle class="flex">
        <span class="text-body-2 font-weight-light text-wrap">
          {{ $t(`requests.types.${requestType}.request_title`) }}
        </span>
        <br />
        <span v-if="props.request.title" class="text-wrap">
          {{ props.request.title }}
          <VTooltip
            v-model="titleTooltip"
            location="bottom"
            :open-on-hover="false"
            :open-on-click="true"
            @click:outside="titleTooltip = false"
          >
            <template #activator="{ props: infoProps }">
              <VBtn :icon="mdiInformationOutline" size="x-small" v-bind="infoProps" />
            </template>
            {{ $t('requests.title_info_message') }}
          </VTooltip>
        </span>
      </VToolbarTitle>
      <slot name="top-actions"></slot>
    </VToolbar>
    <VCardText class="px-4 pt-2">
      <VContainer class="px-0">
        <VRow v-if="props.request.summary?.[0]">
          <VCol cols="12" class="text-h6 font-weight-bold">
            <VTextarea
              :model-value="props.request.summary[0]"
              :label="$t('terms.summary')"
              variant="plain"
              readonly
              hide-details
              rows="1"
              auto-grow
              class="my-2"
            />
          </VCol>
        </VRow>
        <VRow>
          <VCol cols="12" class="text-body-1 font-weight-bold py-0">
            {{ $t('terms.requested') }}
          </VCol>
        </VRow>
        <VRow>
          <VCol cols="12">
            <component
              :is="detailView?.component"
              v-if="detailView"
              :request="request"
              :operation="detailView.operation"
              mode="detail"
            />
          </VCol>
        </VRow>
      </VContainer>
    </VCardText>

    <table
      v-if="approvals.length > 0"
      class="approvers mx-4 text-body-1"
      data-test-id="request-approvals"
    >
      <thead>
        <tr>
          <th>{{ $t('requests.approvals') }}</th>
          <th></th>
          <th></th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="approval in approvals" :key="approval.approver?.id">
          <td>
            {{ approval.approver.name?.[0] || approval.approver.id }}
          </td>
          <td>
            <RequestApprovalStatusChip
              :status="approval.approval.status"
              size="small"
              class="ml-2"
            />
          </td>
          <td>
            <p v-if="approval.approval.status_reason[0]" class="text-medium-emphasis text-body-2">
              {{ approval.approval.status_reason[0] }}
            </p>
          </td>
        </tr>
      </tbody>
    </table>

    <VCardText v-if="props.details.can_approve || reason" class="px-4 pt-2">
      <VContainer class="px-0">
        <VRow>
          <VCol cols="12">
            <VTextarea
              v-model.trim="reason"
              data-test-id="request-details-comment"
              :label="$t('requests.comment_optional')"
              :variant="props.details.can_approve ? 'underlined' : 'plain'"
              hide-details
              rows="1"
              auto-grow
              :readonly="props.loading || !props.details.can_approve"
            />
          </VCol>
        </VRow>
      </VContainer>
    </VCardText>
    <VDivider />
    <VCardActions class="pa-4 d-flex flex-column-reverse flex-column flex-md-row ga-4">
      <RequestMetadata
        :request="props.request"
        :details="props.details"
        class="flex-grow-1 flex-md-grow-0 align-self-start align-self-md-end"
        :class="{ 'mt-8': props.details.can_approve }"
      />
      <div class="d-flex flex-column flex-md-row ga-1 justify-end flex-grow-1 w-100 w-md-auto">
        <template v-if="props.details.can_approve">
          <VBtn
            data-test-id="request-details-reject"
            variant="elevated"
            color="error"
            class="ma-0"
            :disabled="props.loading"
            @click="$emit('reject', reasonOrUndefined)"
          >
            {{ $t('terms.reject') }}
          </VBtn>
          <VBtn
            data-test-id="request-details-approve"
            variant="elevated"
            color="success"
            class="ma-0"
            :disabled="props.loading"
            @click="$emit('approve', reasonOrUndefined)"
          >
            {{ $t('terms.approve') }}
          </VBtn>
          <slot name="bottom-actions"></slot>
        </template>
        <template v-else>
          <RequestStatusChip :status="request.status" />
          <VDivider class="d-md-none mx-2" />
        </template>
      </div>
    </VCardActions>
  </VCard>
</template>

<script setup lang="ts">
import { mdiInformationOutline } from '@mdi/js';
import type { Component } from 'vue';
import { computed, ref } from 'vue';
import { useI18n } from 'vue-i18n';
import {
  VBtn,
  VCardActions,
  VCardText,
  VCol,
  VContainer,
  VDivider,
  VRow,
  VTextarea,
  VToolbar,
  VToolbarTitle,
  VTooltip,
} from 'vuetify/components';
import { Request, RequestOperation } from '~/generated/station/station.did';
import { RequestDetails } from '~/types/station.types';
import { KeysOfUnion } from '~/utils/helper.utils';
import AddAccountOperation from './operations/AddAccountOperation.vue';
import AddAddressBookEntryOperation from './operations/AddAddressBookEntryOperation.vue';
import AddRequestPolicyOperation from './operations/AddRequestPolicyOperation.vue';
import AddUserGroupOperation from './operations/AddUserGroupOperation.vue';
import AddUserOperation from './operations/AddUserOperation.vue';
import ChangeCanisterOperation from './operations/ChangeCanisterOperation.vue';
import EditPermissionOperation from './operations/EditPermissionOperation.vue';
import EditAccountOperation from './operations/EditAccountOperation.vue';
import EditAddressBookEntryOperation from './operations/EditAddressBookEntryOperation.vue';
import EditRequestPolicyOperation from './operations/EditRequestPolicyOperation.vue';
import EditUserGroupOperation from './operations/EditUserGroupOperation.vue';
import EditUserOperation from './operations/EditUserOperation.vue';
import RemoveAddressBookEntryOperation from './operations/RemoveAddressBookEntryOperation.vue';
import RemoveRequestPolicyOperation from './operations/RemoveRequestPolicyOperation.vue';
import RemoveUserGroupOperation from './operations/RemoveUserGroupOperation.vue';
import TransferOperation from './operations/TransferOperation.vue';
import RequestMetadata from './RequestMetadata.vue';
import RequestStatusChip from './RequestStatusChip.vue';
import RequestApprovalStatusChip from './RequestApprovalStatusChip.vue';

const i18n = useI18n();

const props = withDefaults(
  defineProps<{
    request: Request;
    details: RequestDetails;
    loading?: boolean;
  }>(),
  {
    loading: false,
  },
);

const titleTooltip = ref(false);

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
};

defineEmits<{
  (event: 'approve', reason?: string): void;
  (event: 'reject', reason?: string): void;
}>();

const detailView = computed<{
  component: Component;
  operation: RequestOperation[keyof RequestOperation];
} | null>(() => {
  const keys = Object.keys(componentsMap) as Array<keyof RequestOperation>;
  for (const key of keys) {
    if (key in props.request.operation && key in componentsMap) {
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

const reason = ref('');
const reasonOrUndefined = computed(() => (reason.value.length ? reason.value : undefined));

const approvals = computed(() =>
  props.request.approvals.map(approval => {
    const approver = props.details.approvers.find(approver => approver.id === approval.approver_id);

    if (approver?.id === props.request.requested_by && !approval.status_reason[0]) {
      approval.status_reason[0] = i18n.t('requests.requester_auto_approval');
    }

    return {
      approver: approver || {
        id: approval.approver_id,
        name: [],
      },
      approval,
    };
  }),
);
</script>

<style scoped lang="scss">
.approvers {
  width: 100%;
  border-collapse: collapse;

  th {
    text-align: left;
    padding: 0px 4px 8px;
  }

  td {
    padding: 0px 4px 8px;
    vertical-align: top;
  }
}
</style>