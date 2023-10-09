<template>
  <VForm ref="form" class="transfer-form" @submit.prevent="submit">
    <VContainer fluid class="px-0">
      <VRow>
        <VCol cols="12" class="py-0">
          <VAutocomplete
            v-model="transferStore.form.walletId"
            :label="$t('terms.from')"
            variant="solo"
            :readonly="transferStore.fixedWallet"
            density="compact"
            :disabled="transferStore.loading"
            :prepend-icon="mdiWallet"
            :rules="transferStore.validationRules.walletId"
            :items="transferStore.wallets"
          >
            <template #item="{ props: itemProps, item }">
              <VListItem v-bind="itemProps" :title="item.title" :subtitle="item.raw.balance" />
            </template>
            <template #selection="{ item }">
              <VListItem
                density="compact"
                class="px-0"
                :title="item.title"
                :subtitle="item.raw.balance"
              />
            </template>
          </VAutocomplete>
        </VCol>
        <VCol cols="12" class="py-0">
          <VTextField
            v-model="transferStore.form.to"
            :label="$t('terms.destination_address')"
            variant="solo"
            density="compact"
            :disabled="transferStore.loading"
            type="text"
            :prepend-icon="mdiSend"
            :rules="transferStore.validationRules.to"
          />
        </VCol>
        <VCol cols="12" class="py-0">
          <VTextField
            v-model="transferStore.form.amount"
            :label="$t('terms.amount')"
            variant="solo"
            density="compact"
            :disabled="transferStore.loading"
            type="number"
            :prepend-icon="mdiNumeric"
            :rules="transferStore.validationRules.amount"
          />
        </VCol>
      </VRow>
    </VContainer>
  </VForm>
</template>

<script lang="ts" setup>
import { mdiWallet, mdiSend, mdiNumeric } from '@mdi/js';
import { ref, computed, watch, onMounted } from 'vue';
import { Transfer, WalletId } from '~/generated/bank/bank.did';
import { useTransferFormStore } from '~/ui/stores/TransferForm';

const transferStore = useTransferFormStore();

const emit = defineEmits<{
  (event: 'update:modelValue', payload: boolean): void;
  (event: 'saved', payload: Transfer): void;
  (event: 'loading', payload: boolean): void;
  (event: 'updated', payload: boolean): void;
}>();

const props = defineProps<{
  modelValue: boolean;
  walletId?: WalletId;
}>();

onMounted(() => {
  transferStore.load(props.walletId);
  transferStore.$subscribe(() => {
    if (transferStore.hasChanges) {
      emit('updated', true);
    }
  });
});

const submitted = computed({
  get: () => props.modelValue,
  set: value => emit('update:modelValue', value),
});

watch(submitted, () => {
  if (submitted.value) {
    submit();
  }

  submitted.value = false;
});

const form = ref<{ validate: () => Promise<{ valid: boolean }> } | null>(null);

const submit = async () => {
  const { valid } = form.value ? await form.value.validate() : { valid: false };

  transferStore.isValid = valid;

  if (valid) {
    transferStore.loading = true;
    emit('loading', transferStore.loading);
    const transfer = await transferStore.save();
    if (transfer) {
      emit('saved', transfer);
    }
    emit('loading', transferStore.loading);
  }
};
</script>

<style scoped lang="scss">
.transfer-form {
  height: 100%;
}
</style>
