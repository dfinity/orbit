<template>
  <TransferDialog v-model="open" :wallet-id="props.walletId" />

  <VBtn rounded color="primary-variant" :prepend-icon="mdiSend" @click="open = true">
    {{ $t('terms.new_transfer') }}
  </VBtn>
</template>
<script lang="ts" setup>
import { mdiSend } from '@mdi/js';
import { ref } from 'vue';
import TransferDialog from '~/ui/components/TransferDialog.vue';

const props = defineProps<{
  walletId?: string;
}>();

const open = ref(false);
</script>
