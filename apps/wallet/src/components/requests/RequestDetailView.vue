<template>
  <VCard>
    <VToolbar color="background">
      <VToolbarTitle class="flex">
        <span class="text-body-2 font-weight-light text-wrap">
          {{
            $te(`requests.types.${requestType}.request_title`)
              ? $t(`requests.types.${requestType}.request_title`)
              : requestType
          }}
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
    <VCardText class="px-6 pt-2 pb-0">
      <VContainer class="px-0 pb-0">
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

    <VCardText v-if="props.details.can_approve || reason" class="px-6 pt-0">
      <VTextarea
        v-model.trim="reason"
        data-test-id="request-details-comment"
        :label="$t('requests.comment_optional')"
        :variant="props.details.can_approve ? 'underlined' : 'plain'"
        hide-details
        rows="1"
        auto-grow
        :readonly="props.loading || (!props.details.can_approve && !canCancel)"
      />
    </VCardText>

    <VExpansionPanels data-test-id="request-approvals-and-evaluation">
      <VExpansionPanel :elevation="0">
        <template #title>
          <span class="text-body-1 font-weight-bold">{{
            $t('requests.approvals_and_evaluation')
          }}</span>
        </template>
        <VExpansionPanelText>
          <template v-if="evaulationSummary">
            <div class="text-body-1 font-weight-bold">
              {{ $t('terms.summary') }}
            </div>
            <div class="mb-6 text-medium-emphasis text-body-2">
              {{ evaulationSummary }}
            </div>
          </template>

          <table
            v-if="approvals.length > 0"
            class="approvers text-body-1"
            data-test-id="request-approvals"
          >
            <thead>
              <tr>
                <th class="pl-0">{{ $t('requests.approvals') }}</th>
                <th></th>
                <th class="d-none d-sm-table-cell"></th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="approval in approvals" :key="approval.approver.id">
                <td class="pl-0">
                  {{ approval.approver.name }}
                </td>

                <td>
                  <RequestApprovalStatusChip
                    :status="approval.approval.status"
                    size="small"
                    class="d-sm-none"
                  />
                  <p
                    v-if="approval.approval.status_reason[0]"
                    class="text-medium-emphasis text-body-2"
                  >
                    {{ approval.approval.status_reason[0] }}
                  </p>
                </td>
                <td class="text-right d-none d-sm-table-cell">
                  <RequestApprovalStatusChip
                    :status="approval.approval.status"
                    size="small"
                    class="ml-2"
                  />
                </td>
              </tr>
            </tbody>
          </table>
          <template v-if="props.details.evaluationResult && policyResults">
            <div class="text-body-1 font-weight-bold mt-4">
              {{ $t('requests.evaluation.acceptance_rules') }}
            </div>
            <VList :density="'compact'" data-test-id="request-acceptance-rules">
              <PolicyRuleResultView
                :evaluated-rule="policyResults.evaluated_rule"
                :status="policyResults.status"
                :request-approvals="props.request.approvals"
              ></PolicyRuleResultView>
            </VList>
          </template>
        </VExpansionPanelText>
      </VExpansionPanel>
    </VExpansionPanels>

    <VContainer v-if="!!requestFailed" class="px-6" data-test-id="request-details-failure">
      <VRow>
        <VCol class="text-body-1 font-weight-bold pb-0">
          {{ $t('requests.failure_title') }}
        </VCol>
      </VRow>
      <VRow class="">
        <VCol class="text-body-2 text-medium-emphasis pt-2 pb-0">
          {{ requestFailed }}
        </VCol>
      </VRow>
    </VContainer>

    <VDivider class="mt-4" />
    <VCardActions class="py-4 px-6 d-flex flex-column-reverse flex-column flex-md-row ga-4">
      <RequestMetadata
        :request="props.request"
        :details="props.details"
        class="flex-grow-1 flex-md-grow-0 align-self-start align-self-md-end"
        :class="{ 'mt-8': props.details.can_approve }"
      />
      <div class="d-flex flex-column flex-md-row ga-1 justify-end flex-grow-1 w-100 w-md-auto">
        <VBtn
          v-if="canCancel"
          data-test-id="request-details-cancel"
          variant="plain"
          class="ma-0"
          :disabled="props.loading"
          @click="$emit('cancel', reasonOrUndefined)"
        >
          {{ $t('terms.cancel_request') }}
        </VBtn>

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
          <VDivider class="d-md-none" />
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
  VExpansionPanel,
  VExpansionPanelText,
  VExpansionPanels,
  VList,
  VRow,
  VTextarea,
  VToolbar,
  VToolbarTitle,
  VTooltip,
} from 'vuetify/components';
import {
  Request,
  RequestOperation,
  RequestPolicyRuleResult,
} from '~/generated/station/station.did';
import { RequestDetails } from '~/types/station.types';
import { statusReasonsToTextSummary } from '~/utils/evaluation.utils';
import { KeysOfUnion, variantIs } from '~/utils/helper.utils';
import PolicyRuleResultView from './PolicyRuleResultView.vue';
import RequestApprovalStatusChip from './RequestApprovalStatusChip.vue';
import RequestMetadata from './RequestMetadata.vue';
import RequestStatusChip from './RequestStatusChip.vue';
import AddAccountOperation from './operations/AddAccountOperation.vue';
import AddAddressBookEntryOperation from './operations/AddAddressBookEntryOperation.vue';
import AddAssetOperation from './operations/AddAssetOperation.vue';
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
import { useStationStore } from '~/stores/station.store';

const i18n = useI18n();
const store = useStationStore();

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
  SystemUpgrade: SystemUpgradeOperation,
  EditPermission: EditPermissionOperation,
  ManageSystemInfo: ManageSystemInfoOperation,
  AddAsset: AddAssetOperation,
  EditAsset: EditAssetOperation,
  RemoveAsset: RemoveAssetOperation,
  CallExternalCanister: CallExternalCanisterOperation,
  ChangeExternalCanister: UnsupportedOperation,
  CreateExternalCanister: UnsupportedOperation,
  ConfigureExternalCanister: UnsupportedOperation,
  SetDisasterRecovery: UnsupportedOperation,
  FundExternalCanister: UnsupportedOperation,
  MonitorExternalCanister: UnsupportedOperation,
  PruneExternalCanister: UnsupportedOperation,
  RestoreExternalCanister: UnsupportedOperation,
  SnapshotExternalCanister: UnsupportedOperation,
  AddNamedRule: UnsupportedOperation,
  EditNamedRule: UnsupportedOperation,
  RemoveNamedRule: UnsupportedOperation,
};

defineEmits<{
  (event: 'approve', reason?: string): void;
  (event: 'reject', reason?: string): void;
  (event: 'cancel', reason?: string): void;
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

  // if no operation is found, show unsupported operation
  const unknownOperationType = Object.keys(props.request.operation)?.[0];
  return unknownOperationType
    ? {
        component: UnsupportedOperation,
        operation: props.request.operation[unknownOperationType as keyof RequestOperation],
      }
    : null;
});

const canCancel = computed(() => {
  return props.request.requested_by === store.user.id && variantIs(props.request.status, 'Created');
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

const requestFailed = computed(() => {
  if (variantIs(props.request.status, 'Failed')) {
    return props.request.status.Failed.reason[0] ?? i18n.t('requests.failure_reason_unknown');
  }

  return false;
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
        name: '',
      },
      approval,
    };
  }),
);

// if there are multiple policy results for the request, wrap them in an AnyOf rule for semantic consistency
const policyResults = computed((): RequestPolicyRuleResult | null => {
  if (props.details.evaluationResult) {
    if (props.details.evaluationResult.policy_results.length > 1) {
      return {
        evaluated_rule: {
          AnyOf: props.details.evaluationResult.policy_results,
        },
        status: props.details.evaluationResult.status,
      };
    } else {
      return props.details.evaluationResult.policy_results[0];
    }
  }

  return null;
});

const evaulationSummary = computed(() => {
  if (props.details.evaluationResult && props.details.evaluationResult.result_reasons[0]) {
    return statusReasonsToTextSummary(
      props.details.evaluationResult.status,
      props.details.evaluationResult.result_reasons[0],
    );
  }
  return false;
});
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
