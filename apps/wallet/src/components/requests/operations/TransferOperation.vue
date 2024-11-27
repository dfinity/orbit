<template>
  <div v-if="isListMode" class="d-flex flex-column ga-0 text-caption">
    <RequestOperationListRow v-if="formValue.from_account_id && formValue.to && formValue.amount">
      <div class="d-flex flex-row flex-wrap ga-2">
        <div class="d-flex align-center text-no-wrap">
          <VBtn
            :append-icon="mdiOpenInApp"
            size="x-small"
            class="px-1"
            variant="text"
            :to="{
              name: Routes.Account,
              params: { id: formValue.from_account_id },
            }"
          >
            <TextOverflow :text="account?.name ?? formValue.from_account_id" :max-length="12" />
          </VBtn>
          <VIcon :icon="mdiArrowRight" size="x-small" class="ml-1" />
        </div>
        <div class="d-flex align-center text-no-wrap">
          <ShortenedAddress :address="formValue.to!" :format="format" />
          <VBtn
            size="x-small"
            variant="text"
            :icon="mdiContentCopy"
            @click="
              copyToClipboard({
                textToCopy: formValue.to!,
                sendNotification: true,
              })
            "
          />
        </div>
        <div class="d-flex align-center text-no-wrap flex-grow-1">
          {{ account ? formatBalance(formValue.amount, asset.decimals) : '-' }}
          {{ account ? asset.symbol : '' }}
        </div>
      </div>
    </RequestOperationListRow>
  </div>
  <div v-else-if="account">
    <VTextField
      :model-value="account.name"
      variant="plain"
      :label="$t('terms.account')"
      :prepend-icon="mdiWallet"
      readonly
    />
    <TransferForm :model-value="formValue" :account="account" mode="view" :asset="asset" />
  </div>
</template>

<script setup lang="ts">
import { Ref, computed, onBeforeMount, ref } from 'vue';
import TransferForm from '~/components/accounts/TransferForm.vue';
import { Request, Transfer, TransferOperation } from '~/generated/station/station.did';
import RequestOperationListRow from '../RequestOperationListRow.vue';
import { mdiArrowRight, mdiContentCopy, mdiOpenInApp, mdiWallet } from '@mdi/js';
import { Routes } from '~/configs/routes.config';
import TextOverflow from '~/components/TextOverflow.vue';
import { copyToClipboard } from '~/utils/app.utils';
import { formatBalance } from '~/utils/helper.utils';
import ShortenedAddress from '~/components/ShortenedAddress.vue';
import { AddressFormat } from '~/types/chain.types';
import { detectAddressFormat } from '~/utils/asset.utils';

const props = withDefaults(
  defineProps<{
    request: Request;
    operation: TransferOperation;
    mode?: 'list' | 'detail';
  }>(),
  {
    mode: 'list',
  },
);

const isListMode = computed(() => props.mode === 'list');
const formValue: Ref<Partial<Transfer>> = ref({});
const account = computed(() => props.operation.from_account?.[0]);
const asset = computed(() => props.operation.from_asset);
const format = ref<AddressFormat | string | undefined>(undefined);

onBeforeMount(() => {
  const transfer: Partial<Transfer> = {};
  transfer.to = props.operation.input.to;
  transfer.amount = props.operation.input.amount;
  transfer.from_account_id = props.operation.input.from_account_id;
  if (props.operation.input.fee?.[0]) {
    transfer.fee = props.operation.input.fee[0];
  }
  if (props.operation.input.network?.[0]) {
    transfer.network = props.operation.input.network[0];
  }
  transfer.metadata = props.operation.input.metadata;

  format.value = detectAddressFormat(props.operation.from_asset.blockchain, transfer.to);

  formValue.value = transfer;
});
</script>
