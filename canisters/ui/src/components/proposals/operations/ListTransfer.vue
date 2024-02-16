<template>
  <div class="d-flex flex-row ga-2">
    <div class="d-flex flex-no-wrap text-no-wrap align-center">
      <VBtn
        :append-icon="mdiOpenInApp"
        size="x-small"
        variant="text"
        :to="{
          name: Routes.Account,
          params: { id: operation.input.from_account_id },
        }"
      >
        <TextOverflow :text="account?.name ?? operation.input.from_account_id" :max-length="12" />
      </VBtn>
      <VIcon :icon="mdiArrowRight" size="x-small" class="ml-1" />
    </div>
    <div class="d-flex flex-no-wrap text-no-wrap align-center">
      <TextOverflow :text="operation.input.to" />
      <VBtn
        size="x-small"
        variant="text"
        :icon="mdiContentCopy"
        @click="
          copyToClipboard({
            textToCopy: operation.input.to,
            sendNotification: true,
          })
        "
      />
    </div>
    <div class="d-flex flex-no-wrap text-no-wrap align-center justify-end flex-grow-1">
      {{ account ? formatBalance(operation.input.amount, account.decimals) : '-' }}
      {{ account ? account.symbol : '' }}
    </div>
  </div>
</template>

<script setup lang="ts">
import { mdiArrowRight, mdiContentCopy, mdiOpenInApp } from '@mdi/js';
import { toRefs } from 'vue';
import { computed } from 'vue';
import TextOverflow from '~/components/TextOverflow.vue';
import { Routes } from '~/configs/routes.config';
import { Proposal, TransferOperation } from '~/generated/wallet/wallet.did';
import { copyToClipboard } from '~/utils/app.utils';
import { formatBalance } from '~/utils/helper.utils';

const input = defineProps<{
  proposal: Proposal;
  operation: TransferOperation;
}>();

const { operation } = toRefs(input);

const account = computed(() => operation.value.from_account?.[0]);
</script>
