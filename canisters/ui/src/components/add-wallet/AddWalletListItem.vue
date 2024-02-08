<template>
  <AddWalletDialog>
    <template #activator="{ props }">
      <VListItem v-bind="props" data-test-id="add-wallet-item">
        <VIcon left class="mr-1">
          {{ mdiPlus }}
        </VIcon>
        <span>{{ $t('wallets.add_wallet_list_item') }}</span>
      </VListItem>
    </template>
  </AddWalletDialog>
</template>

<script lang="ts" setup>
import { mdiPlus } from '@mdi/js';
import AddWalletDialog from './AddWalletDialog.vue';
</script>
