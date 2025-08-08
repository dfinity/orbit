<template>
  <VDialog
    v-bind="$attrs"
    v-model="open"
    :persistent="!canClose"
    transition="dialog-bottom-transition"
    scrollable
    :max-width="props.dialogMaxWidth"
  >
    <VCard data-test-id="canister-snapshot-create-card">
      <VToolbar color="background">
        <VToolbarTitle>
          {{ dialogTitle }}
        </VToolbarTitle>
        <VBtn :disabled="!canClose" :icon="mdiClose" @click="open = false" />
      </VToolbar>
      <VDivider />
      <VCardText>
        {{ $t('external_canisters.snapshots.create_snapshot_description') }}
        <VDivider class="my-4" />
        <VTextarea
          v-model="canisterCreateSnapshotModel.comment"
          name="comment"
          class="mt-2"
          :prepend-inner-icon="mdiComment"
          :label="$t(`requests.comment_optional`)"
          variant="filled"
          density="comfortable"
          auto-grow
          rows="2"
          hide-details
        />
      </VCardText>
      <VCardActions class="pa-3">
        <VSpacer />
        <VBtn
          :loading="saving"
          color="primary"
          data-test-id="submit-btn"
          variant="elevated"
          @click="createSnapshot"
        >
          {{ $t('terms.create') }}
        </VBtn>
      </VCardActions>
    </VCard>
  </VDialog>
</template>
<script lang="ts" setup>
import { Principal } from '@icp-sdk/core/principal';
import { mdiClose, mdiComment } from '@mdi/js';
import { Ref, computed, ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import { VBtn, VCard, VDialog, VDivider, VToolbar, VToolbarTitle } from 'vuetify/components';
import { CanisterCreateSnapshotModel } from './external-canisters.types';
import { useStationStore } from '~/stores/station.store';
import { assertAndReturn } from '~/utils/helper.utils';
import {
  useOnFailedOperation,
  useOnSuccessfulOperation,
} from '~/composables/notifications.composable';
import logger from '~/core/logger.core';

const props = withDefaults(
  defineProps<{
    open?: boolean;
    canisterId: Principal;
    dialogMaxWidth?: number;
    title?: string;
  }>(),
  {
    open: false,
    dialogMaxWidth: 800,
    title: undefined,
  },
);

const emit = defineEmits<{
  (event: 'update:open', payload: boolean): void;
}>();

const i18n = useI18n();
const canClose = ref(true);
const dialogTitle = computed(
  () => props.title || i18n.t('external_canisters.snapshots.create_snapshot'),
);

const initialModel = (): CanisterCreateSnapshotModel => {
  const model: CanisterCreateSnapshotModel = {
    canisterId: Principal.fromUint8Array(props.canisterId.toUint8Array()),
  };

  return model;
};

const saving = ref(false);
const station = useStationStore();
const canisterCreateSnapshotModel = ref(initialModel()) as Ref<CanisterCreateSnapshotModel>;

const open = computed({
  get: () => props.open,
  set: isOpen => emit('update:open', isOpen),
});

watch(
  open,
  isOpen => {
    if (isOpen) {
      canisterCreateSnapshotModel.value = initialModel();
    }
  },
  { immediate: true },
);

const createSnapshot = async (): Promise<void> => {
  saving.value = true;

  try {
    const canisterId = assertAndReturn(
      canisterCreateSnapshotModel.value.canisterId,
      'model.canisterId',
    );

    const newRequest = await station.service.createExternalCanisterSnapshot(canisterId, {
      comment: canisterCreateSnapshotModel.value.comment,
    });

    useOnSuccessfulOperation(newRequest);

    open.value = false;
  } catch (error) {
    logger.error(`Failed to request canister snapshot creation ${error}`);

    useOnFailedOperation();
  } finally {
    saving.value = false;
  }
};
</script>
