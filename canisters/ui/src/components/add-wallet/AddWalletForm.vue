<template>
  <VForm ref="form" @submit.prevent="addNewWallet">
    <VCard>
      <VCardItem>
        <VCardTitle>{{ $t('wallets.add_wallet_dialog_title') }}</VCardTitle>
      </VCardItem>
      <VCardText>
        <VTextField
          v-model.trim="name"
          :label="$t('terms.name')"
          data-test-id="add-wallet-form-name"
        />
        <VTextField
          v-model="canisterId"
          :rules="[
            requiredRule,
            uniqueRule(existingWallets, $t('wallets.add_wallet_dialog_already_added')),
            validCanisterId,
          ]"
          :label="$t('terms.canister_id')"
          data-test-id="add-wallet-form-canister-id"
        />
      </VCardText>

      <VCardActions>
        <VSpacer />
        <VBtn data-test-id="cancel-button" @click="emit('cancelled')">{{
          $t('terms.cancel')
        }}</VBtn>
        <VBtn
          type="submit"
          data-test-id="submit-button"
          :disabled="!isFormValid"
          :loading="working"
        >
          {{ $t('terms.submit') }}</VBtn
        >
      </VCardActions>
    </VCard>
  </VForm>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue';
import {
  VBtn,
  VCard,
  VCardActions,
  VCardItem,
  VCardText,
  VCardTitle,
  VForm,
  VSpacer,
  VTextField,
} from 'vuetify/components';
import { useAppStore } from '~/stores/app.store';
import { useSessionStore } from '~/stores/session.store';
import { VFormValidation } from '~/types/helper.types';
import { requiredRule, uniqueRule, validCanisterId } from '~/utils/form.utils';

const form = ref<VFormValidation | null>(null);

const session = useSessionStore();
const app = useAppStore();

const working = ref(false);
const canisterId = ref('');
const name = ref('');

const isFormValid = computed(() => (form.value ? form.value.isValid : false));

const existingWallets = computed(() => session.data.wallets.map(wallet => wallet.canisterId));

const emit = defineEmits<{
  (event: 'submitted'): void;
  (event: 'cancelled'): void;
  (event: 'working', value: boolean): void;
}>();

function reset() {
  canisterId.value = '';
  name.value = '';
}

defineExpose({
  reset,
});

async function addNewWallet() {
  if (working.value) {
    return;
  }

  const { valid } = form.value ? await form.value.validate() : { valid: false };

  if (valid) {
    emit('working', true);
    working.value = true;

    try {
      await session.addWallet(canisterId.value, name.value);

      emit('submitted');
    } catch (e: unknown) {
      app.sendErrorNotification(e);
    }
    emit('working', false);
    working.value = false;
  }
}
</script>
