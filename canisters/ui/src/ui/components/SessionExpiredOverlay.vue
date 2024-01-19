
<template>
  <VDialog persistent v-model="session.reauthenticationNeeded" width="600">
    <template v-slot:default>
      <VCard :title="$t('session.expired_dialog_title')">
        <VCardText>
          {{ $t('session.expired_dialog_content') }}
        </VCardText>
        <VCardActions>
          <VSpacer />
          <VBtn @click="session.reauthenticate">
            {{ $t('session.expired_dialog_btn') }}
          </VBtn>
        </VCardActions>
      </VCard>
    </template>
  </VDialog>
</template>

<script lang="ts" setup>
import { VCardText, VDialog } from 'vuetify/components';
import { useSessionStore } from '../stores/session';

const session = useSessionStore();

</script>
