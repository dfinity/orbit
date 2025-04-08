<template>
  <PageLayout>
    <template #custom>
      <VMain
        class="d-flex flex-column logo-markers"
        :class="{ 'logo-markers-dark': app.isDarkTheme }"
      >
        <VToolbar density="compact" class="bg-transparent py-4">
          <VContainer fluid :class="{ 'px-16': !app.isMobile }">
            <VRow>
              <VCol cols="6" class="d-flex justify-start align-center">
                <BrandLogo height="20px" />
              </VCol>
              <VCol cols="6" class="d-flex ga-0 ga-md-1 align-center justify-end">
                <VBtn
                  :icon="app.isDarkTheme ? mdiWeatherNight : mdiWeatherSunny"
                  @click.prevent="app.toogleTheme"
                />
                <LanguageSelector />
              </VCol>
            </VRow>
          </VContainer>
        </VToolbar>
        <VContainer class="flex-grow-1" :class="{ 'mt-16': !app.isMobile }">
          <VRow>
            <VCol cols="12">
              <VCard max-width="500" class="mx-auto" :class="{ 'py-10': !app.isMobile }">
                <VCardText class="d-flex ga-4 flex-column pa-6">
                  <p class="text-h4 font-weight-bold">
                    {{ $t('landing.connect_title') }}
                  </p>

                  <VList density="compact">
                    <VListItem class="px-0" min-height="auto">
                      <VListItemTitle>{{ $t('landing.highlights.multiapproval') }}</VListItemTitle>
                    </VListItem>
                    <VListItem class="px-0" min-height="auto">
                      <VListItemTitle>{{ $t('landing.highlights.treasury') }}</VListItemTitle>
                    </VListItem>
                    <VListItem class="px-0" min-height="auto">
                      <VListItemTitle>{{ $t('landing.highlights.smartcontract') }}</VListItemTitle>
                    </VListItem>
                  </VList>

                  <VBtn
                    color="white"
                    size="large"
                    class="text-caption"
                    :loading="isAuthenticating"
                    @click.prevent="performLogin"
                    data-test-id="internet-identity-button"
                  >
                    <VImg :src="infinityMark" width="20" />
                    <span class="text-body-1 ml-1">{{ $t('landing.connect_btn') }}</span>
                  </VBtn>

                  <p class="text-caption">
                    <i18n-t keypath="landing.btn_accept_license.phrase" scope="global">
                      <a
                        :href="
                          appInitConfig.marketingSiteUrl
                            ? appInitConfig.marketingSiteUrl + '/license'
                            : '#'
                        "
                        target="_blank"
                        >{{ $t('landing.btn_accept_license.license') }}</a
                      >
                    </i18n-t>
                  </p>
                </VCardText>
              </VCard>
            </VCol>
          </VRow>
        </VContainer>
        <AppFooter class="flex-0-0" />
      </VMain>
    </template>
  </PageLayout>
</template>

<script lang="ts" setup>
import { mdiWeatherNight, mdiWeatherSunny } from '@mdi/js';
import { ref } from 'vue';
import { useI18n } from 'vue-i18n';
import { VBtn, VCard, VCardText, VCol, VContainer, VMain, VRow } from 'vuetify/components';
import BrandLogo from '~/components/BrandLogo.vue';
import LanguageSelector from '~/components/LanguageSelector.vue';
import AppFooter from '~/components/layouts/AppFooter.vue';
import PageLayout from '~/components/PageLayout.vue';
import { appInitConfig } from '~/configs/init.config';
import { logger } from '~/core/logger.core';
import { useAppStore } from '~/stores/app.store';
import { useSessionStore } from '~/stores/session.store';
import { afterLoginRedirect } from '~/utils/app.utils';
import infinityMark from '~assets/images/infinity-mark.png';

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
