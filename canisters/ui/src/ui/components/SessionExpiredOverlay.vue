<template>
  <VDialog v-model="session.reauthenticationNeeded" persistent width="600">
    <VCard :title="$t('session.expired_dialog_title')">
      <VCardText>
        {{ $t('session.expired_dialog_content') }}
      </VCardText>
      <VCardActions>
        <VSpacer />
        <VBtn :loading="isReauthenticating" @click="reauthenticate">
          {{ $t('session.expired_dialog_btn') }}
        </VBtn>
      </VCardActions>
    </VCard>
  </VDialog>
</template>

<script lang="ts" setup>
import { VCardText, VDialog } from 'vuetify/components';
import { useSessionStore } from '../stores/session';
import { useUserActivity } from '../modules/user-activity';
import { ref } from 'vue';
import { services } from '../modules';

const session = useSessionStore();

const isReauthenticating = ref(false);

function reauthenticate() {
  isReauthenticating.value = true;
  session.signIn(false).finally(() => {
    isReauthenticating.value = false;
  });
}

const sessionExpirationService = services().sessionExpiration;

useUserActivity({
  onActive: () => {
    sessionExpirationService.registerActivity();
  },
  throttleMs: 1000,
});
</script>
