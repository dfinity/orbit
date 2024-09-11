<template>
  <VDialog
    v-bind="$attrs"
    v-model="open"
    :persistent="!canClose"
    transition="dialog-bottom-transition"
    scrollable
    :max-width="props.dialogMaxWidth"
  >
    <VCard data-test-id="canister-top-up-card">
      <VToolbar color="background">
        <VToolbarTitle>
          {{ dialogTitle }}
        </VToolbarTitle>
        <VBtn :disabled="!canClose" :icon="mdiClose" @click="open = false" />
      </VToolbar>
      <VDivider />

      <VCardText v-if="loading" class="py-8">
        <LoadingMessage />
      </VCardText>

      <template v-else>
        <VCardText>
          <CanisterTopUpForm
            v-model="topUpModel"
            v-model:trigger-submit="triggerFormSubmit"
            :display="{ canisterId: props.canisterId === undefined }"
            @submit="submit"
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
            data-test-id="canister-top-up-save-button"
            @click="triggerFormSubmit = true"
          >
            {{ $t('external_canisters.send_cycles') }}
          </VBtn>
        </VCardActions>
      </template>
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
import { useStationStore } from '~/stores/station.store';
import { assertAndReturn } from '~/utils/helper.utils';
import LoadingMessage from '../LoadingMessage.vue';
import CanisterTopUpForm from './CanisterTopUpForm.vue';
import { CanisterTopUpModel } from './external-canisters.types';

const props = withDefaults(
  defineProps<{
    open?: boolean;
    canisterId?: Principal;
    dialogMaxWidth?: number;
    title?: string;
  }>(),
  {
    open: false,
    canisterId: undefined,
    dialogMaxWidth: 800,
    title: undefined,
  },
);

const emit = defineEmits<{
  (event: 'update:open', payload: boolean): void;
}>();

const i18n = useI18n();
const loading = ref(false);
const valid = ref(true);
const station = useStationStore();
const submitting = ref(false);
const canClose = computed(() => !loading.value && !submitting.value);
const dialogTitle = computed(() => props.title || i18n.t('external_canisters.top_up'));

const buildModel = (): CanisterTopUpModel => ({
  canisterId: props.canisterId,
  cycles: undefined,
});

const open = computed({
  get: () => props.open,
  set: value => {
    if (!value) {
      topUpModel.value = buildModel();
    }

    emit('update:open', value);
  },
});

const triggerFormSubmit = ref(false);
const canSave = computed(() => valid.value && !loading.value);
const topUpModel = ref(buildModel()) as Ref<CanisterTopUpModel>;

const submit = async (input: CanisterTopUpModel) => {
  try {
    submitting.value = true;

    const request = await station.service.fundExternalCanister({
      canister_id: assertAndReturn(input.canisterId, 'canisterId'),
      kind: {
        Send: {
          cycles: assertAndReturn(input.cycles, 'cycles'),
        },
      },
    });

    useOnSuccessfulOperation(request);
  } catch (error) {
    logger.error('Failed to submit fund request', error);

    useOnFailedOperation();
  } finally {
    submitting.value = false;
  }
};
</script>
