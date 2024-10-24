<template>
  <VDialog
    v-model="openModel"
    :persistent="working"
    transition="dialog-bottom-transition"
    scrollable
    :max-width="props.dialogMaxWidth"
  >
    <VCard :loading="working" data-test-id="remove-asset-dialog-form">
      <VToolbar color="background">
        <VToolbarTitle>{{ $t('pages.account.remove_asset') }}</VToolbarTitle>
        <VBtn :disabled="working" :icon="mdiClose" @click="openModel = false" />
      </VToolbar>
      <VCardText>
        {{ $t('pages.account.remove_asset_confirm') }}
      </VCardText>

      <VCardActions class="pa-3">
        <VSpacer />
        <VBtn
          :disabled="working"
          data-test-id="remove-asset-dialog-cancel-button"
          @click="openModel = false"
        >
          {{ $t('terms.cancel') }}
        </VBtn>
        <VBtn
          :loading="working"
          color="primary"
          variant="elevated"
          data-test-id="remove-asset-dialog-confirm-button"
          @click="removeAsset"
        >
          {{ $t('terms.remove') }}
        </VBtn>
      </VCardActions>
    </VCard>
  </VDialog>
</template>
<script lang="ts" setup>
import { mdiClose } from '@mdi/js';
import { computed, ref } from 'vue';
import { useRouter } from 'vue-router';
import {
  VBtn,
  VCard,
  VCardActions,
  VCardText,
  VDialog,
  VSpacer,
  VToolbar,
  VToolbarTitle,
} from 'vuetify/components';
import {
  useOnFailedOperation,
  useOnSuccessfulOperation,
} from '~/composables/notifications.composable';
import { Routes } from '~/configs/routes.config';
import logger from '~/core/logger.core';
import { Account, UUID } from '~/generated/station/station.did';
import { services } from '~/plugins/services.plugin';
import { variantIs } from '~/utils/helper.utils';

const props = withDefaults(
  defineProps<{
    account: Account;
    asset: UUID;
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

const emit = defineEmits<{
  (event: 'update:open', payload: boolean): void;
}>();

const openModel = computed({
  get: () => props.open,
  set: value => emit('update:open', value),
});

const working = ref(false);
const stationService = services().station;
const router = useRouter();

const removeAsset = async () => {
  try {
    working.value = true;

    const newRequest = await stationService.editAccount({
      account_id: props.account.id,
      change_assets: [{ Change: { add_assets: [], remove_assets: [props.asset] } }],
      configs_permission: [],
      configs_request_policy: [],
      name: [],
      read_permission: [],
      transfer_permission: [],
      transfer_request_policy: [],
    });

    useOnSuccessfulOperation(newRequest);

    if (variantIs(newRequest.status, 'Approved')) {
      router.push({ name: Routes.Account, params: { id: props.account.id } });
    }

    openModel.value = false;
  } catch (error) {
    logger.error(`Failed to request removing asset ${error}`);

    useOnFailedOperation();
  } finally {
    working.value = false;
  }
};
</script>
