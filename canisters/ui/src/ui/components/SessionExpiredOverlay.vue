<template>
  <VDialog v-model="session.reauthenticationNeeded" persistent width="600">
    <VCard :title="$t('session.expired_dialog_title')">
      <VCardText>
        {{ $t('session.expired_dialog_content') }}
      </VCardText>
      <VCardActions>
        <VSpacer />
        <VBtn @click="session.signIn">
          {{ $t('session.expired_dialog_btn') }}
        </VBtn>
      </VCardActions>
    </VCard>
  </VDialog>
</template>

<script lang="ts" setup>
import { VCardText, VDialog } from 'vuetify/components';
import { useSessionStore } from '../stores/session';

const session = useSessionStore();
</script>
