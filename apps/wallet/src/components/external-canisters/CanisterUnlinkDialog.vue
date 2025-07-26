<template>
  <VDialog
    v-bind="$attrs"
    v-model="open"
    :persistent="!canClose"
    transition="dialog-bottom-transition"
    scrollable
    :max-width="props.dialogMaxWidth"
  >
    <VCard data-test-id="canister-unlink-card">
      <VToolbar color="background">
        <VToolbarTitle>
          {{ dialogTitle }}
        </VToolbarTitle>
        <VBtn :disabled="!canClose" :icon="mdiClose" @click="open = false" />
      </VToolbar>
      <VDivider />

      <VCardText>
        {{ $t('app.dialog_confirmation_question') }}

        <VSwitch
          v-model="softDelete"
          class="my-4"
          name="soft_delete"
          :label="$t('external_canisters.unlink_soft_delete')"
          hide-details
          inset
          color="success"
        />
      </VCardText>
      <VDivider />
      <VCardActions class="pa-3">
        <VSpacer />
        <VBtn
          :loading="submitting"
          color="primary"
          variant="elevated"
          data-test-id="canister-unlink-save-button"
          @click="submit"
        >
          {{ $t('external_canisters.unlink') }}
        </VBtn>
      </VCardActions>
    </VCard>
  </VDialog>
</template>
<script lang="ts" setup>
import { Principal } from '@icp-sdk/core/principal';
import { mdiClose } from '@mdi/js';
import { computed, ref } from 'vue';
import { useI18n } from 'vue-i18n';
import { useRouter } from 'vue-router';
import {
  VBtn,
  VCard,
  VCardActions,
  VCardText,
  VDialog,
  VDivider,
  VSpacer,
  VSwitch,
  VToolbar,
  VToolbarTitle,
} from 'vuetify/components';
import {
  useOnFailedOperation,
  useOnSuccessfulOperation,
} from '~/composables/notifications.composable';
import { Routes } from '~/configs/routes.config';
import logger from '~/core/logger.core';
import { useStationStore } from '~/stores/station.store';
import { variantIs } from '~/utils/helper.utils';

const props = withDefaults(
  defineProps<{
    /**
     * Whether the dialog is open or not.
     */
    open?: boolean;
    /**
     * The canister ID to target the unlink operation.
     */
    canisterId: Principal;
    /**
     * Wether or not to also delete the canister from the Internet Computer Subnet, or just delete the local reference
     * to it in the Orbit Station.
     */
    softDelete?: boolean;
    /**
     * The maximum width of the dialog.
     */
    dialogMaxWidth?: number;
    /**
     * The title of the dialog, if not provided, it will default to the locale key of `external_canisters.unlink_title`.
     */
    title?: string;
    /**
     * Redirect when unlink is successful.
     */
    redirect?: boolean;
  }>(),
  {
    open: false,
    softDelete: true,
    dialogMaxWidth: 800,
    title: undefined,
    redirect: true,
  },
);

const i18n = useI18n();
const station = useStationStore();
const submitting = ref(false);
const canClose = computed(() => !submitting.value);
const dialogTitle = computed(() => props.title || i18n.t('external_canisters.unlink_title'));
const softDelete = ref(props.softDelete);
const router = useRouter();

const emit = defineEmits<{
  (event: 'update:open', payload: boolean): void;
}>();

const open = computed({
  get: () => props.open,
  set: value => emit('update:open', value),
});

const submit = async (): Promise<void> => {
  try {
    submitting.value = true;

    const request = await station.service.unlinkExternalCanister({
      canisterId: props.canisterId,
      softDelete: softDelete.value,
    });

    useOnSuccessfulOperation(request);

    open.value = false;

    // Redirect to the external canisters page if the request is marked as approved directly.
    if (props.redirect && variantIs(request.status, 'Approved')) {
      await router.push({ name: Routes.ExternalCanisters });
    }
  } catch (error) {
    logger.error('Failed to submit unlink canister request', error);

    useOnFailedOperation();
  } finally {
    submitting.value = false;
  }
};
</script>
