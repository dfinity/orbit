<template>
  <VDialog v-model="modelValue" persistent transition="dialog-bottom-transition" scrollable>
    <VCard :loading="loading">
      <VToolbar dark color="primary">
        <VBtn icon dark @click="closeDialog()"><VIcon :icon="mdiClose" /></VBtn>
        <VToolbarTitle>
          {{ props.transferId ? $t('terms.transfer') : $t('terms.new_transfer') }}
        </VToolbarTitle>
      </VToolbar>
      <VCardText>
        <TransferForm
          v-if="!props.transferId"
          v-model="submitted"
          :wallet-id="props.walletId"
          @loading="show => (loading = show)"
          @updated="isChanged => (hasChanges = isChanged)"
          @saved="onSave"
        />
      </VCardText>
      <VCardActions>
        <VSpacer />
        <VBtn variant="text" @click="closeDialog()">{{ $t('terms.close') }}</VBtn>
        <VBtn
          v-if="!props.transferId"
          :disabled="!hasChanges"
          :loading="loading"
          color="primary"
          variant="flat"
          @click="submit"
        >
          {{ $t('terms.send') }}
        </VBtn>
      </VCardActions>
    </VCard>
  </VDialog>
</template>

<script lang="ts" setup>
import { ref, computed } from 'vue';
import { TransferId, WalletId } from '~/generated/bank/bank.did';
import { mdiClose } from '@mdi/js';
import TransferForm from '~/ui/components/TransferForm.vue';

const props = defineProps<{
  modelValue: boolean;
  walletId?: WalletId;
  transferId?: TransferId;
}>();

const emit = defineEmits<{
  (event: 'update:modelValue', payload: boolean): void;
}>();

const modelValue = computed({
  get: () => props.modelValue,
  set: value => emit('update:modelValue', value),
});

const closeDialog = (): void => {
  modelValue.value = false;
};

const onSave = (): void => {
  closeDialog();
};

const submitted = ref(false);
const loading = ref(false);
const hasChanges = ref(false);

const submit = (): void => {
  submitted.value = true;
};
</script>
