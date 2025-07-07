<template>
  <VDialog
    v-bind="$attrs"
    v-model="open"
    :persistent="!canClose"
    transition="dialog-bottom-transition"
    scrollable
    :max-width="props.dialogMaxWidth"
  >
    <VCard data-test-id="canister-ic-settings-card">
      <VToolbar color="background">
        <VToolbarTitle>
          {{ dialogTitle }}
        </VToolbarTitle>
        <VBtn :disabled="!canClose" :icon="mdiClose" @click="open = false" />
      </VToolbar>
      <VDivider />

      <VCardText>
        <CanisterIcSettingsForm
          v-model="canisterIcSettingsModel"
          v-model:trigger-submit="triggerFormSubmit"
          :display="{ canisterId: props.canisterId === undefined }"
          @submit="submit"
          @valid="valid = $event"
          @edited="edited = $event"
        />
      </VCardText>
      <VDivider />
      <VCardActions class="pa-3">
        <VSpacer />
        <VBtn
          :disabled="!canSave"
          :loading="submitting"
          color="primary"
          variant="elevated"
          data-test-id="canister-ic-settings-save-button"
          @click="triggerFormSubmit = true"
        >
          {{ $t('terms.save') }}
        </VBtn>
      </VCardActions>
    </VCard>
  </VDialog>
</template>
<script lang="ts" setup>
import { Principal } from '@icp-sdk/core/principal';
import { mdiClose } from '@mdi/js';
import { Ref, computed, ref } from 'vue';
import { useI18n } from 'vue-i18n';
import {
  VBtn,
  VCard,
  VCardActions,
  VCardText,
  VDialog,
  VDivider,
  VSpacer,
  VToolbar,
  VToolbarTitle,
} from 'vuetify/components';
import {
  useOnFailedOperation,
  useOnSuccessfulOperation,
} from '~/composables/notifications.composable';
import logger from '~/core/logger.core';
import { DefiniteCanisterSettings } from '~/generated/station/station.did';
import { useStationStore } from '~/stores/station.store';
import { assertAndReturn, parseToNumberOrUndefined } from '~/utils/helper.utils';
import CanisterIcSettingsForm from './CanisterIcSettingsForm.vue';
import { CanisterIcSettingsModel } from './external-canisters.types';

const props = withDefaults(
  defineProps<{
    open?: boolean;
    canisterId?: Principal;
    canisterSettings?: DefiniteCanisterSettings;
    dialogMaxWidth?: number;
    title?: string;
  }>(),
  {
    open: false,
    canisterId: undefined,
    canisterSettings: undefined,
    dialogMaxWidth: 800,
    title: undefined,
  },
);

const emit = defineEmits<{
  (event: 'update:open', payload: boolean): void;
}>();

const i18n = useI18n();
const valid = ref(true);
const edited = ref(false);
const station = useStationStore();
const submitting = ref(false);
const canClose = computed(() => !submitting.value);
const dialogTitle = computed(() => props.title || i18n.t('external_canisters.ic_settings'));

const initialModel = (): CanisterIcSettingsModel => {
  const model: CanisterIcSettingsModel = {};
  model.canisterId = props.canisterId;
  model.compute_allocation = parseToNumberOrUndefined(props.canisterSettings?.compute_allocation);
  model.controllers = props.canisterSettings?.controllers.map(controller =>
    // Copies the principal to avoid mutating the original object
    Principal.fromUint8Array(controller.toUint8Array()),
  );
  model.freezing_threshold = parseToNumberOrUndefined(props.canisterSettings?.freezing_threshold);
  model.memory_allocation = parseToNumberOrUndefined(props.canisterSettings?.memory_allocation);
  model.reserved_cycles_limit = parseToNumberOrUndefined(
    props.canisterSettings?.reserved_cycles_limit,
  );
  model.log_visibility = props.canisterSettings?.log_visibility;
  model.wasm_memory_limit = parseToNumberOrUndefined(props.canisterSettings?.wasm_memory_limit);

  return model;
};

const open = computed({
  get: () => props.open,
  set: isOpen => {
    if (!isOpen) {
      canisterIcSettingsModel.value = initialModel();
    }

    emit('update:open', isOpen);
  },
});

const triggerFormSubmit = ref(false);
const canSave = computed(() => valid.value && edited.value);
const canisterIcSettingsModel = ref(initialModel()) as Ref<CanisterIcSettingsModel>;

const submit = async (input: CanisterIcSettingsModel) => {
  try {
    submitting.value = true;

    const previousControllers = new Set();
    props.canisterSettings?.controllers?.forEach(controller => {
      previousControllers.add(controller.toText());
    });

    const hasUpdatedControllers =
      props.canisterSettings?.controllers.length !== input.controllers?.length ||
      input.controllers?.some(controller => !previousControllers.has(controller.toText()));

    const request = await station.service.editCanisterIcSettings(
      assertAndReturn(input.canisterId, 'Canister ID is required'),
      {
        compute_allocation:
          input.compute_allocation !== undefined &&
          BigInt(input.compute_allocation) !== props.canisterSettings?.compute_allocation
            ? [BigInt(input.compute_allocation)]
            : [],
        freezing_threshold:
          input.freezing_threshold !== undefined &&
          BigInt(input.freezing_threshold) !== props.canisterSettings?.freezing_threshold
            ? [BigInt(input.freezing_threshold)]
            : [],
        memory_allocation:
          input.memory_allocation !== undefined &&
          BigInt(input.memory_allocation) !== props.canisterSettings?.memory_allocation
            ? [BigInt(input.memory_allocation)]
            : [],
        reserved_cycles_limit:
          input.reserved_cycles_limit !== undefined &&
          BigInt(input.reserved_cycles_limit) !== props.canisterSettings?.reserved_cycles_limit
            ? [BigInt(input.reserved_cycles_limit)]
            : [],
        wasm_memory_limit:
          input.wasm_memory_limit !== undefined &&
          BigInt(input.wasm_memory_limit) !== props.canisterSettings?.wasm_memory_limit
            ? [BigInt(input.wasm_memory_limit)]
            : [],
        log_visibility:
          input.log_visibility &&
          JSON.stringify(input.log_visibility) !==
            JSON.stringify(props.canisterSettings?.log_visibility)
            ? [input.log_visibility]
            : [],
        controllers: input.controllers && hasUpdatedControllers ? [input.controllers] : [],
      },
    );

    useOnSuccessfulOperation(request);

    open.value = false;
  } catch (error) {
    logger.error('Failed to submit update ic settings request', error);

    useOnFailedOperation();
  } finally {
    submitting.value = false;
  }
};
</script>
