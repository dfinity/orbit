<template>
  <TransferDialog v-model="open" :account-id="props.accountId" />

  <VBtn rounded color="primary-variant" :prepend-icon="mdiSend" @click="open = true">
    {{ $t('terms.new_transfer') }}
  </VBtn>
</template>
<script lang="ts" setup>
import { mdiSend } from '@mdi/js';
import { ref } from 'vue';
import TransferDialog from '~/components/TransferDialog.vue';

const props = defineProps<{
  accountId?: string;
}>();

const open = ref(false);
</script>
