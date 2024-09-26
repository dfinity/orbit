<template>
  <VDialog
    v-bind="$attrs"
    v-model="open"
    :persistent="!canClose"
    transition="dialog-bottom-transition"
    scrollable
    :max-width="props.dialogMaxWidth"
  >
    <VCard data-test-id="canister-install-card">
      <VToolbar color="background">
        <VToolbarTitle>
          {{ dialogTitle }}
        </VToolbarTitle>
        <VBtn :disabled="!canClose" :icon="mdiClose" @click="open = false" />
      </VToolbar>
      <VDivider />

      <VCardText>
        <VCol cols="12" class="py-0 px-0">
          <p class="text-h6 mb-2">
            {{ $t('terms.settings') }}
          </p>
          <VDivider />
        </VCol>

        <CanisterInstallForm
          v-model="canisterInstallModel"
          v-model:trigger-submit="triggerFormSubmit"
          :display="{ canisterId: props.canisterId === undefined }"
          @submit="submit"
          @valid="valid = $event"
        />

        <VCol cols="12" class="py-0 mt-2 px-0">
          <p class="text-h6 mb-2">
            {{ $t('terms.comment') }}
          </p>
          <VDivider />
        </VCol>

        <VCol cols="12" class="px-0">
          <VTextarea
            v-model="requestComment"
            name="comment"
            :label="$t(`requests.comment_optional`)"
            variant="filled"
            density="comfortable"
            auto-grow
            rows="2"
            hide-details
          />
        </VCol>
      </VCardText>
      <VDivider />
      <VCardActions class="pa-3">
        <VSpacer />
        <VBtn
          :disabled="!canSave"
          :loading="submitting"
          color="primary"
          variant="elevated"
          data-test-id="canister-install-save-button"
          @click="triggerFormSubmit = true"
        >
          {{ $t('terms.submit') }}
        </VBtn>
      </VCardActions>
    </VCard>
  </VDialog>
</template>
<script lang="ts" setup>
import { Principal } from '@dfinity/principal';
import { mdiClose } from '@mdi/js';
import { Ref, computed, ref } from 'vue';
import { useI18n } from 'vue-i18n';
import {
  VBtn,
  VCard,
  VCardActions,
  VCardText,
  VCol,
  VDialog,
  VDivider,
  VSpacer,
  VTextarea,
  VToolbar,
  VToolbarTitle,
} from 'vuetify/components';
import CanisterInstallForm from '~/components/external-canisters/CanisterInstallForm.vue';
import {
  useOnFailedOperation,
  useOnSuccessfulOperation,
} from '~/composables/notifications.composable';
import logger from '~/core/logger.core';
import { useStationStore } from '~/stores/station.store';
import { assertAndReturn } from '~/utils/helper.utils';
import { CanisterInstallModel } from './external-canisters.types';

const props = withDefaults(
  defineProps<{
    open?: boolean;
    canisterId?: Principal;
    canisterModuleHash?: string;
    dialogMaxWidth?: number;
    title?: string;
  }>(),
  {
    open: false,
    canisterId: undefined,
    canisterModuleHash: undefined,
    dialogMaxWidth: 800,
    title: undefined,
  },
);

const emit = defineEmits<{
  (event: 'update:open', payload: boolean): void;
}>();

const i18n = useI18n();
const valid = ref(true);
const station = useStationStore();
const submitting = ref(false);
const canClose = computed(() => !submitting.value);
const dialogTitle = computed(() => props.title || i18n.t('external_canisters.install'));

const initialModel = (): CanisterInstallModel => {
  const model: CanisterInstallModel = {};
  model.mode = props.canisterModuleHash ? { upgrade: null } : { install: null };
  model.canisterId = props.canisterId
    ? Principal.fromUint8Array(props.canisterId.toUint8Array())
    : undefined;

  return model;
};

const open = computed({
  get: () => props.open,
  set: isOpen => {
    if (!isOpen) {
      canisterInstallModel.value = initialModel();
    }

    emit('update:open', isOpen);
  },
});

const triggerFormSubmit = ref(false);
const canSave = computed(() => valid.value);
const canisterInstallModel = ref(initialModel()) as Ref<CanisterInstallModel>;
const requestComment = ref<string>();

const submit = async (input: CanisterInstallModel) => {
  try {
    submitting.value = true;

    const request = await station.service.changeExternalCanister(
      {
        canister_id: assertAndReturn(input.canisterId, 'canister id required'),
        mode: assertAndReturn(input.mode, 'install mode required'),
        module: assertAndReturn(input.wasmModule, 'wasm module required'),
        arg: input.wasmInstallArg !== undefined ? [input.wasmInstallArg] : [],
      },
      {
        comment:
          requestComment.value && requestComment.value.trim().length > 0
            ? requestComment.value.trim()
            : undefined,
      },
    );

    useOnSuccessfulOperation(request);

    open.value = false;
  } catch (error) {
    logger.error('Failed to submit change canister request', error);

    useOnFailedOperation();
  } finally {
    submitting.value = false;
  }
};
</script>
