<template>
  <PageLayout>
    <template #custom>
      <VMain class="d-flex flex-column bg-landing logo-markers-bg">
        <AppToolbar logo :sidebar="false" :theme-selector="false" bg-color="bg-transparent" />
        <VContainer
          fluid
          class="flex-grow-1"
          :class="{
            'mt-16': !app.isMobile,
          }"
        >
          <VRow>
            <VCol cols="12" md="6">
              <VCard variant="text" max-width="500" class="mx-auto">
                <VCardText>
                  <h1 class="text-h1 font-weight-bold">{{ $t('landing.title') }}</h1>
                  <p class="text-h4 mt-4 font-weight-light">{{ $t('landing.subtitle') }}</p>
                  <p v-if="!app.isMobile" class="text-h6 mt-4">{{ $t('landing.description') }}</p>
                </VCardText>
              </VCard>
            </VCol>
            <VCol cols="12" md="6">
              <VCard rounded="xl" max-width="500" class="mx-auto" color="landing-surface">
                <VCardText class="d-flex align-center ga-4 justify-center flex-column py-12 px-8">
                  <p class="text-h4 font-weight-bold text-center">
                    {{ $t('landing.connect_title') }}
                  </p>
                  <VBtn
                    color="primary"
                    rounded
                    width="300"
                    size="large"
                    class="mt-4 text-caption"
                    :loading="isAuthenticating"
                    @click.prevent="performLogin"
                  >
                    <span class="text-body-1">{{ $t('landing.connect_btn') }}</span>
                  </VBtn>
                </VCardText>
              </VCard>
            </VCol>
            <VCol v-if="app.isMobile" cols="12">
              <p class="text-h6 mt-4">{{ $t('landing.description') }}</p>
            </VCol>
          </VRow>
        </VContainer>
        <AppFooter />
      </VMain>
    </template>
  </PageLayout>
</template>

<script lang="ts" setup>
import { ref } from 'vue';
import { useI18n } from 'vue-i18n';
import { VBtn, VCard, VCardText, VCol, VContainer, VMain, VRow } from 'vuetify/components';
import AppFooter from '~/components/layouts/AppFooter.vue';
import AppToolbar from '~/components/layouts/AppToolbar.vue';
import PageLayout from '~/components/PageLayout.vue';
import { logger } from '~/core/logger.core';
import { useAppStore } from '~/stores/app.store';
import { useSessionStore } from '~/stores/session.store';
import { afterLoginRedirect } from '~/utils/app.utils';

const app = useAppStore();
const session = useSessionStore();
const i18n = useI18n();

const isAuthenticating = ref(false);

const performLogin = (): void => {
  isAuthenticating.value = true;

  session
    .signIn({ redirectOnSignIn: false })
    .then(() => afterLoginRedirect())
    .catch((e: Error) => {
      logger.error(`Authentication failed`, e);

      app.sendNotification({
        message: i18n.t('landing.connect_error'),
        type: 'error',
      });
    })
    .finally(() => {
      isAuthenticating.value = false;
    });
};
</script>
