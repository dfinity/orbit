<template>
  <VDialog
    v-model="openModel"
    :persistent="loading || approving"
    transition="dialog-bottom-transition"
    scrollable
    :max-width="props.dialogMaxWidth.value"
  >
    <DataLoader
      v-if="currentRequestId"
      :key="currentRequestId"
      v-slot="{ data }"
      :load="loadRequest"
      @loading="loading = $event"
      @loaded="onRequestLoaded"
    >
      <VCard v-if="loading">
        <VToolbar color="background">
          <VToolbarTitle>{{ $t('terms.request') }}</VToolbarTitle>
          <VBtn :icon="mdiClose" @click="openModel = false" />
        </VToolbar>
        <VCardText v-if="loading" class="py-8">
          <LoadingMessage />
        </VCardText>
      </VCard>
      <RequestDetailView
        v-else-if="data"
        :request="data.request"
        :details="{
          can_approve: data.privileges.can_approve,
          requester_name: data.additionalInfo.requester_name,
          approvers: data.additionalInfo.approvers,
          evaluationResult: data.additionalInfo.evaluation_result[0],
        }"
        :loading="approving || loading"
        @closed="openModel = false"
        @opened="openModel = true"
        @approve="reason => onApproval(RequestApprovalStatusEnum.Approved, reason)"
        @reject="reason => onApproval(RequestApprovalStatusEnum.Rejected, reason)"
      >
        <template #top-actions>
          <VSwitch
            v-if="data.privileges.can_approve"
            v-model="loadNext"
            data-test-id="load-next-request-switch"
            :label="$t('requests.load_next')"
            class="flex-0-1"
            :hide-details="true"
            color="primary"
            :disabled="approving"
          />

          <VBtn :disabled="approving" :icon="mdiClose" @click="openModel = false" />
        </template>
        <template v-if="loadNext" #bottom-actions>
          <VBtn variant="plain" :disabled="approving" class="ma-0" @click="skip">
            {{ $t('terms.skip') }}
          </VBtn>
        </template>
      </RequestDetailView>
    </DataLoader>
    <div v-else>
      <VCard class="text-center" flat data-test-id="no-more-requests">
        <VCardText class="text-body-1 mt-10">
          <VIcon :icon="mdiCheckCircle" size="x-large" />
          {{ $t('requests.no_more_requests_to_approve') }}
        </VCardText>
        <VCardActions class="pa-4 d-flex flex-md-row ga-2 justify-end">
          <VBtn variant="outlined" :disabled="loading" @click="openModel = false">
            {{ $t('terms.close') }}
          </VBtn>
        </VCardActions>
      </VCard>
    </div>
  </VDialog>
</template>
<script lang="ts" setup>
import { mdiCheckCircle, mdiClose } from '@mdi/js';
import { computed, ref, toRefs, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import { VBtn, VCard, VCardActions, VCardText, VDialog, VIcon, VSwitch } from 'vuetify/components';
import DataLoader from '~/components/DataLoader.vue';
import logger from '~/core/logger.core';
import {
  GetRequestResultData,
  ListRequestsOperationType,
  Request,
  UUID,
} from '~/generated/station/station.did';
import {
  mapRequestApprovalStatusEnumToVariant,
  mapRequestOperationToListRequestsOperationType,
} from '~/mappers/requests.mapper';
import { services } from '~/plugins/services.plugin';
import { useAppStore } from '~/stores/app.store';
import { useStationStore } from '~/stores/station.store';
import { variantIs } from '~/utils/helper.utils';
import RequestDetailView from './RequestDetailView.vue';
import LoadingMessage from '~/components/LoadingMessage.vue';
import { RequestApprovalStatusEnum } from '~/types/requests.types';

type DataType = {
  request: GetRequestResultData['request'];
  privileges: GetRequestResultData['privileges'];
  additionalInfo: GetRequestResultData['additional_info'];
};

const input = withDefaults(
  defineProps<{
    requestId: UUID;
    open?: boolean;
    dialogMaxWidth?: number;
    readonly?: boolean;
  }>(),
  {
    open: false,
    dialogMaxWidth: 800,
    readonly: false,
  },
);
const props = toRefs(input);
const emit = defineEmits<{
  (event: 'update:open', payload: boolean): void;
  (event: 'approved'): void;
  (event: 'closed'): void;
  (event: 'opened'): void;
  (event: 'request-changed', payload: UUID): void;
}>();
const currentRequestId = ref<UUID | null>(props.requestId.value);
const preloadedData = ref<DataType | null>(null);
const approving = ref(false);
const loading = ref(false);
const skippedRequestIds = ref<UUID[]>([]);

const requestType = ref<ListRequestsOperationType | undefined>();
const loadNext = ref(false);

watch(props.open, isOpen => {
  if (isOpen) {
    currentRequestId.value = props.requestId.value;
    skippedRequestIds.value = [];
    preloadedData.value = null;
    loadNext.value = false;
  }
});

const openModel = computed({
  get: () => props.open.value,
  set: value => {
    emit('update:open', value);

    if (value) {
      emit('opened');
    } else {
      emit('closed');
    }
  },
});
const i18n = useI18n();
const app = useAppStore();
const station = useStationStore();

const loadRequest = async (): Promise<DataType> => {
  station.notifications.items.forEach(notification => {
    if (
      !notification.loading &&
      !variantIs(notification.data.status, 'Read') &&
      ((variantIs(notification.data.notification_type, 'RequestCreated') &&
        notification.data.notification_type.RequestCreated.request_id === currentRequestId.value) ||
        (variantIs(notification.data.notification_type, 'RequestFailed') &&
          notification.data.notification_type.RequestFailed.request_id ===
            currentRequestId.value) ||
        (variantIs(notification.data.notification_type, 'RequestRejected') &&
          notification.data.notification_type.RequestRejected.request_id ===
            currentRequestId.value))
    ) {
      station.markNotificationRead(notification.data.id, true);
    }
  });

  if (preloadedData.value && preloadedData.value.request.id === currentRequestId.value) {
    return {
      request: preloadedData.value.request as Request,
      privileges: preloadedData.value.privileges,
      additionalInfo: preloadedData.value.additionalInfo,
    };
  } else {
    const result = await services().station.getRequest(
      { request_id: currentRequestId.value!, with_full_info: [] },
      true,
    );

    return {
      request: result.request,
      privileges: result.privileges,
      additionalInfo: result.additional_info,
    };
  }
};

const skip = async (): Promise<void> => {
  approving.value = true;
  skippedRequestIds.value.push(currentRequestId.value!);

  preloadedData.value = await loadNextRequest();

  if (preloadedData.value) {
    currentRequestId.value = preloadedData.value.request.id;
    emit('request-changed', currentRequestId.value);
  } else {
    currentRequestId.value = null;
  }

  approving.value = false;
};

const onRequestLoaded = (data: Awaited<ReturnType<typeof loadRequest>>): void => {
  requestType.value = mapRequestOperationToListRequestsOperationType(data.request.operation);
};

const loadNextRequest = async (): Promise<DataType | null> => {
  const nextRequest = await services().station.getNextApprovableRequest({
    types: [requestType.value!],
    excludedRequestIds: skippedRequestIds.value,
  });

  if (nextRequest.length === 0) {
    return null;
  }

  return {
    request: nextRequest[0].request,
    privileges: nextRequest[0].privileges,
    additionalInfo: nextRequest[0].additional_info,
  };
};

const onApproval = async (decision: RequestApprovalStatusEnum, reason?: string): Promise<void> => {
  if (currentRequestId.value === null) {
    return;
  }

  approving.value = true;

  return station.service
    .submitRequestApproval({
      request_id: currentRequestId.value,
      decision: mapRequestApprovalStatusEnumToVariant(decision),
      reason: reason && reason.length ? [reason] : [],
    })
    .then(async () => {
      app.sendNotification({
        type: 'success',
        message: i18n.t('app.action_save_success'),
      });

      if (loadNext.value) {
        // keep open, load next

        preloadedData.value = await loadNextRequest();

        if (preloadedData.value) {
          currentRequestId.value = preloadedData.value.request.id;
          emit('request-changed', currentRequestId.value);
        } else {
          currentRequestId.value = null;
        }
      } else {
        emit('approved');
        openModel.value = false;
      }
    })
    .catch(err => {
      logger.error(`Failed to approval on request:`, err);

      app.sendNotification({
        type: 'error',
        message: i18n.t('app.action_save_failed'),
      });
    })
    .finally(() => {
      approving.value = false;
    });
};
</script>
