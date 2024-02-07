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
import { Principal } from '@dfinity/principal';
import { computed, ref } from 'vue';
import { sessionUserWalletToUserWallet } from '~/mappers/wallets.mapper';
import { i18n } from '~/modules/i18n.module';
import { services } from '~/modules/services.module';
import { useAppStore } from '~/stores/app.store';
import { useSessionStore } from '~/stores/session.store';
import { VFormValidation } from '~/types/utils.types';
import { isApiError } from '~/utils/app.utils';
import { requiredRule, uniqueRule, validCanisterId } from '~/utils/form.utils';

const form = ref<VFormValidation | null>(null);

const session = useSessionStore();
const app = useAppStore();

const working = ref(false);
const canisterId = ref('');
const name = ref('');

const isFormValid = computed(() => (form.value ? form.value.isValid : false));

const existingWallets = computed(() => session.data.wallets.map(wallet => wallet.canisterId));

const controlPanelService = services().controlPanel;

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
  const { valid } = form.value ? await form.value.validate() : { valid: false };

  if (valid) {
    emit('working', true);
    working.value = true;

    try {
      const user = await controlPanelService.editUser({
        main_wallet: session.mainWallet ? [session.mainWallet] : [],
        wallets: [
          [
            ...session.data.wallets.map(wallet => sessionUserWalletToUserWallet(wallet)),
            sessionUserWalletToUserWallet({
              canisterId: canisterId.value,
              name: name.value,
            }),
          ],
        ],
      });

      session.populateUser(user);

      await session.connectWallet(Principal.fromText(canisterId.value));

      emit('submitted');
    } catch (e: unknown) {
      let message = i18n.global.t('app.request_failed_message');

      if (isApiError(e) && e.message.length > 0) {
        message = `${message}: ${e.message[0]}`;
      } else if (e instanceof Error) {
        message = `${message}: ${e.message}`;
      }
      app.sendNotification({
        type: 'error',
        message,
      });
    }
    emit('working', false);
    working.value = false;
  }
}
</script>
