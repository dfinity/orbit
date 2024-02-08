<template>
  <AccountConfigurationDialog v-model="show" mode="add" @saved="emit('created')" />

  <VBtn rounded color="primary-variant" :prepend-icon="mdiWallet" @click="show = true">
    {{ $t('terms.new_account') }}
  </VBtn>
</template>
<script lang="ts" setup>
import { ref } from 'vue';
import { mdiWallet } from '@mdi/js';
import AccountConfigurationDialog from './AccountConfigDialog.vue';

const emit = defineEmits<{
  (event: 'created'): void;
}>();

const show = ref<boolean>(false);
</script>
