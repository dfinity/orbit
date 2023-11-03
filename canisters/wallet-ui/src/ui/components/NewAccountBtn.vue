<template>
  <NewAccountDialog />

  <VBtn rounded color="primary-variant" :prepend-icon="mdiWallet" @click="createAccountForm.open">
    {{ $t('terms.new_account') }}
  </VBtn>
</template>
<script lang="ts" setup>
import { mdiWallet } from '@mdi/js';
import NewAccountDialog from '~/ui/components/NewAccountDialog.vue';
import { useCreateAccountFormStore } from '~/ui/stores';

const createAccountForm = useCreateAccountFormStore();
</script>
