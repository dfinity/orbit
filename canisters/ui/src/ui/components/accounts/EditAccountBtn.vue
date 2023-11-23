<template>
  <AccountConfigurationDialog v-model="show" :account="account" mode="edit" />

  <VBtn size="small" variant="text" :icon="mdiPencil" @click="show = true" />
</template>
<script lang="ts" setup>
import { computed, ref } from 'vue';
import { mdiPencil } from '@mdi/js';
import AccountConfigurationDialog from './AccountConfigDialog.vue';
import { Account } from '~/generated/wallet/wallet.did';

const props = defineProps<{
  modelValue: Account;
}>();

const emit = defineEmits<{
  (event: 'update:modelValue', value: Account): void;
}>();

const show = ref<boolean>(false);
const account = computed({
  get: () => props.modelValue,
  set: (value: Account) => {
    if (!value) {
      emit('update:modelValue', value);
    }
  },
});
</script>
