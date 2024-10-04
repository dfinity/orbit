<template>
  <RequestDialog
    v-if="requestId"
    v-model:open="open"
    :request-id="requestId"
    @approved="open = false"
    @request-changed="updateRequestId"
  />
</template>
<script setup lang="ts">
import { ref, watch } from 'vue';
import { useRouter } from 'vue-router';
import RequestDialog from '~/components/requests/RequestDialog.vue';
import { useRequestOverlay } from '~/composables/request.composable';
import { REQUEST_DIALOG_QUERY_PARAM } from '~/core/constants.core';
import { UUID } from '~/generated/control-panel/control_panel.did';
import { isValidUUID } from '~/utils/helper.utils';

const requestOverlay = useRequestOverlay();
const open = ref(false);
const router = useRouter();
const requestId = ref<string | null>(null);

function updateRequestId(requestId: UUID) {
  requestOverlay.replaceQueryId(requestId);
}

watch(
  () => router.currentRoute.value,
  route => {
    const queryParam = route.query?.[REQUEST_DIALOG_QUERY_PARAM];
    if (!queryParam) {
      requestId.value = null;
    }

    const id = Array.isArray(queryParam) ? (queryParam?.[0] ?? '') : queryParam;

    requestId.value = isValidUUID(`${id}`) ? id : null;
  },
  { deep: true, immediate: true },
);

watch(
  () => requestId.value,
  () => {
    open.value = !!requestId.value;
  },
  { immediate: true },
);

watch(
  () => open.value,
  open => {
    if (!open) {
      // Delay to allow the dialog to close before removing the query param
      setTimeout(() => {
        requestOverlay.replaceQueryId(undefined);
      }, 100);
    }
  },
);
</script>
