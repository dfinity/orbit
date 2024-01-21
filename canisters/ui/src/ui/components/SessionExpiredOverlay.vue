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
import { onMounted, onUnmounted } from 'vue';
import { VCardText, VDialog } from 'vuetify/components';
import { useSessionStore } from '../stores/session';
import { throttle } from '~/core/utils';

const session = useSessionStore();

const registerActivity = throttle(() => {
  session.registerActivity();
}, 1000);

onMounted(() => {
  window.addEventListener('mousemove', registerActivity);
  window.addEventListener('mousedown', registerActivity);
  window.addEventListener('keypress', registerActivity);
  window.addEventListener('DOMMouseScroll', registerActivity);
  window.addEventListener('mousewheel', registerActivity);
  window.addEventListener('touchmove', registerActivity);
  window.addEventListener('MSPointerMove', registerActivity);
});

onUnmounted(() => {
  window.removeEventListener('mousemove', registerActivity);
  window.removeEventListener('mousedown', registerActivity);
  window.removeEventListener('keypress', registerActivity);
  window.removeEventListener('DOMMouseScroll', registerActivity);
  window.removeEventListener('mousewheel', registerActivity);
  window.removeEventListener('touchmove', registerActivity);
  window.removeEventListener('MSPointerMove', registerActivity);
});


</script>
