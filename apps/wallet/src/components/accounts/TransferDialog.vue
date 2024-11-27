<template>
  <VDialog
    v-model="openModel"
    :persistent="loading || saving"
    transition="dialog-bottom-transition"
    scrollable
    :max-width="props.dialogMaxWidth.value"
  >
    <DataLoader
      v-slot="{ data }"
      :load="loadTransfer"
      @loading="loading = $event"
      @loaded="
        transfer = $event.transfer;
        request = $event.request;
      "
    >
      <VCard :loading="loading" data-test-id="transfer-dialog-form">
        <VToolbar color="background">
          <VToolbarTitle>{{
            $t('terms.transfer_asset', { asset: props.asset.value.symbol })
          }}</VToolbarTitle>
          <VBtn :disabled="loading || saving" :icon="mdiClose" @click="openModel = false" />
        </VToolbar>
        <VCardText>
          <TransferForm
            v-if="data"
            v-model="transfer"
            v-model:trigger-submit="triggerSubmit"
            :account="props.account.value"
            :asset="props.asset.value"
            :mode="props.readonly.value ? 'view' : 'edit'"
            @submit="save"
            @valid="valid = $event"
          />

          <VRow>
            <VCol :cols="12">
              <VTextField
                v-model="summary"
                :label="$t('terms.comment_optional')"
                density="comfortable"
                class="mb-2"
                name="to"
                :disabled="props.readonly.value"
                type="text"
                :prepend-icon="mdiComment"
                data-test-id="transfer-dialog-request-summary"
              />
            </VCol>
          </VRow>
        </VCardText>
        <VDivider />
        <VCardActions class="pa-3">
          <VSpacer />
          <VBtn
            v-if="!props.readonly.value"
            :disabled="!canSave"
            :loading="saving"
            color="primary"
            variant="elevated"
            data-test-id="transfer-dialog-save-button"
            @click="triggerSubmit = true"
          >
            {{ props.transferId.value ? $t('terms.save') : $t('terms.create') }}
          </VBtn>
        </VCardActions>
      </VCard>
    </DataLoader>
  </VDialog>
</template>
<script lang="ts" setup>
import { mdiClose, mdiComment } from '@mdi/js';
import { computed, ref, toRefs } from 'vue';
import {
  VBtn,
  VCard,
  VCardActions,
  VCardText,
  VCol,
  VDialog,
  VDivider,
  VRow,
  VSpacer,
  VTextField,
  VToolbar,
  VToolbarTitle,
} from 'vuetify/components';
import DataLoader from '~/components/DataLoader.vue';
import {
  useOnFailedOperation,
  useOnSuccessfulOperation,
} from '~/composables/notifications.composable';
import logger from '~/core/logger.core';
import { Account, Asset, Request, Transfer, UUID } from '~/generated/station/station.did';
import { services } from '~/plugins/services.plugin';
import { maybeTransformBlockchainAddress } from '~/utils/app.utils';
import { assertAndReturn } from '~/utils/helper.utils';
import TransferForm from './TransferForm.vue';
import { detectAddressStandard } from '~/utils/asset.utils';
import { useStationStore } from '~/stores/station.store';

const input = withDefaults(
  defineProps<{
    account: Account;
    asset: Asset;
    transferId?: UUID;
    open?: boolean;
    dialogMaxWidth?: number;
    readonly?: boolean;
  }>(),
  {
    transferId: undefined,
    open: false,
    dialogMaxWidth: 800,
    readonly: false,
  },
);

const emit = defineEmits<{
  (event: 'update:open', payload: boolean): void;
}>();

const props = toRefs(input);
const valid = ref(true);
const loading = ref(false);
const saving = ref(false);
const transfer = ref<Partial<Transfer>>({});
const request = ref<Partial<Request>>({});
const openModel = computed({
  get: () => props.open.value,
  set: value => emit('update:open', value),
});

const summary = computed({
  get: () => request.value.summary?.[0],
  set: value => {
    request.value.summary = !value ? [] : [value];
  },
});

const stationService = services().station;

const stationStore = useStationStore();

const loadTransfer = async (): Promise<{
  transfer: Partial<Transfer>;
  request: Partial<Request>;
}> => {
  if (props.transferId.value === undefined) {
    const createModel: Partial<Transfer> = {
      from_account_id: props.account.value.id,
    };

    return { transfer: createModel, request: {} };
  }

  const transfer = await stationService.getTransfer(props.transferId.value);

  const { request } = await stationService.getRequest({
    request_id: transfer.request_id,
    with_full_info: [],
  });

  return { transfer, request };
};

const canSave = computed(() => {
  return valid.value && !loading.value;
});

const triggerSubmit = ref(false);

const save = async (): Promise<void> => {
  if (!canSave.value || transfer.value.id) {
    return;
  }

  try {
    saving.value = true;

    const toAddress = assertAndReturn(transfer.value.to, 'to');

    const maybeStandard = detectAddressStandard(
      props.asset.value,
      toAddress,
      stationStore.configuration.details.supported_blockchains,
    );

    if (!maybeStandard) {
      throw new Error('Invalid address');
    }

    const newRequest = await stationService.transfer(
      {
        from_account_id: assertAndReturn(transfer.value.from_account_id, 'from_account_id'),
        from_asset_id: props.asset.value.id,
        with_standard: maybeStandard.standard,
        amount: assertAndReturn(transfer.value.amount, 'amount'),
        to: maybeTransformBlockchainAddress(
          props.asset.value.blockchain,
          maybeStandard.standard,
          toAddress,
        ),
        fee: transfer.value.fee ? [transfer.value.fee] : [],
        metadata: transfer.value.metadata ?? [],
        network: transfer.value.network ? [transfer.value.network] : [],
      },
      summary.value,
    );

    useOnSuccessfulOperation(newRequest);

    openModel.value = false;
  } catch (error) {
    logger.error(`Failed to request transfer ${error}`);

    useOnFailedOperation();
  } finally {
    saving.value = false;
  }
};
</script>
