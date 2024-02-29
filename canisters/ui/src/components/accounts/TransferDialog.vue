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
      @loaded="transfer = $event.transfer"
    >
      <VCard :loading="loading">
        <VToolbar dark color="surface">
          <VToolbarTitle>{{ $t('terms.transfer') }}</VToolbarTitle>
          <VBtn :disabled="loading || saving" :icon="mdiClose" dark @click="openModel = false" />
        </VToolbar>
        <VCardText>
          <TransferForm
            v-if="data"
            v-model="transfer"
            v-model:trigger-submit="triggerSubmit"
            :account="props.account.value"
            :mode="props.readonly.value ? 'view' : 'edit'"
            @submit="save"
            @valid="valid = $event"
          />

          <VRow v-if="!props.transferId.value">
            <VCol :cols="12">
              <VTextField
                v-model="summary"
                :label="$t('terms.summary')"
                variant="underlined"
                density="compact"
                class="mb-2"
                name="to"
                :disabled="props.readonly.value"
                type="text"
                :prepend-icon="mdiComment"
              />
            </VCol>
          </VRow>
        </VCardText>
        <VCardActions class="pa-3">
          <VSpacer />
          <VBtn
            v-if="!props.readonly.value"
            :disabled="!canSave"
            :loading="saving"
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
import DataLoader from '~/components/DataLoader.vue';
import TransferForm from './TransferForm.vue';
import {
  useOnFailedOperation,
  useOnSuccessfulOperation,
} from '~/composables/notifications.composable';
import logger from '~/core/logger.core';
import { Account, Proposal, Transfer, UUID } from '~/generated/wallet/wallet.did';
import { useWalletStore } from '~/stores/wallet.store';
import { assertAndReturn } from '~/utils/helper.utils';

const input = withDefaults(
  defineProps<{
    account: Account;
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
const proposal = ref<Partial<Proposal>>({});
const openModel = computed({
  get: () => props.open.value,
  set: value => emit('update:open', value),
});

const summary = computed({
  get: () => proposal.value.summary?.[0],
  set: value => {
    proposal.value.summary = !value ? [] : [value];
  },
});

const wallet = useWalletStore();

const loadTransfer = async (): Promise<{
  transfer: Partial<Transfer>;
}> => {
  if (props.transferId.value === undefined) {
    const createModel: Partial<Transfer> = {
      from_account_id: props.account.value.id,
    };

    return { transfer: createModel };
  }

  const result = await wallet.service.getTransfer(props.transferId.value);

  // todo: also load proposal to show summary

  return { transfer: result };
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

    const newProposal = await wallet.service.transfer(
      {
        from_account_id: assertAndReturn(transfer.value.from_account_id, 'from_account_id'),
        amount: assertAndReturn(transfer.value.amount, 'amount'),
        to: assertAndReturn(transfer.value.to, 'to'),
        fee: transfer.value.fee ? [transfer.value.fee] : [],
        metadata: transfer.value.metadata ?? [],
        network: transfer.value.network ? [transfer.value.network] : [],
      },
      summary.value,
    );

    useOnSuccessfulOperation(newProposal);

    openModel.value = false;
  } catch (error) {
    logger.error(`Failed to request transfer ${error}`);

    useOnFailedOperation();
  } finally {
    saving.value = false;
  }
};
</script>
