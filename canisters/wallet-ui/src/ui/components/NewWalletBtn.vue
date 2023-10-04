<template>
  <NewWalletDialog />

  <VBtn rounded color="primary-variant" :prepend-icon="mdiWallet" @click="createWalletForm.open">
    {{ $t('terms.new_wallet') }}
  </VBtn>
</template>
<script lang="ts" setup>
import { mdiWallet } from '@mdi/js';
import NewWalletDialog from '~/ui/components/NewWalletDialog.vue';
import { useCreateWalletFormStore } from '~/ui/stores';

const createWalletForm = useCreateWalletFormStore();
</script>
