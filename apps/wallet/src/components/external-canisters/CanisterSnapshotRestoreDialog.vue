<template>
  <VDialog
    v-bind="$attrs"
    v-model="open"
    :persistent="!canClose"
    transition="dialog-bottom-transition"
    scrollable
    :max-width="props.dialogMaxWidth"
  >
    <VCard data-test-id="canister-snapshot-restore-card">
      <VToolbar color="background">
        <VToolbarTitle>
          {{ dialogTitle }}
        </VToolbarTitle>
        <VBtn :disabled="!canClose" :icon="mdiClose" @click="open = false" />
      </VToolbar>
      <VDivider />
      <VCardText>
        {{ $t('external_canisters.snapshots.restore_snapshot_confirmation') }}
        <VDivider class="my-4" />
        <VTextarea
          v-model="canisterRestoreSnapshotModel.comment"
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
          @click="restoreSnapshot"
        >
          {{ $t('terms.restore') }}
        </VBtn>
      </VCardActions>
    </VCard>
  </VDialog>
</template>
<script lang="ts" setup>
import { Principal } from '@dfinity/principal';
import { mdiClose, mdiComment } from '@mdi/js';
import { Ref, computed, ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import { VBtn, VCard, VDialog, VDivider, VToolbar, VToolbarTitle } from 'vuetify/components';
import { CanisterSnapshot, CanisterRestoreSnapshotModel } from './external-canisters.types';
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
    snapshot: CanisterSnapshot;
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
  () => props.title || i18n.t('external_canisters.snapshots.restore_snapshot_title'),
);

const initialModel = (): CanisterRestoreSnapshotModel => {
  const model: CanisterRestoreSnapshotModel = {
    canisterId: Principal.fromUint8Array(props.canisterId.toUint8Array()),
    snapshotId: props.snapshot.snapshotId,
  };

  return model;
};

const saving = ref(false);
const station = useStationStore();
const canisterRestoreSnapshotModel = ref(initialModel()) as Ref<CanisterRestoreSnapshotModel>;

const open = computed({
  get: () => props.open,
  set: isOpen => emit('update:open', isOpen),
});

watch(
  open,
  isOpen => {
    if (isOpen) {
      canisterRestoreSnapshotModel.value = initialModel();
    }
  },
  { immediate: true },
);

const restoreSnapshot = async (): Promise<void> => {
  saving.value = true;

  try {
    const canisterId = assertAndReturn(
      canisterRestoreSnapshotModel.value.canisterId,
      'model.canisterId',
    );
    const snapshotId = assertAndReturn(
      canisterRestoreSnapshotModel.value.snapshotId,
      'model.snapshotId',
    );

    const newRequest = await station.service.restoreExternalCanisterSnapshot(
      canisterId,
      snapshotId,
      {
        comment: canisterRestoreSnapshotModel.value.comment,
      },
    );

    useOnSuccessfulOperation(newRequest);

    open.value = false;
  } catch (error) {
    logger.error(`Failed to request canister snapshot restoration ${error}`);

    useOnFailedOperation();
  } finally {
    saving.value = false;
  }
};
</script>
