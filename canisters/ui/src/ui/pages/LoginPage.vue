<template>
  <PageLayout :background-color="pageBackgroundColor" hide-toolbar-context>
    <template v-if="!app.isMobile" #sidebar-header>
      <h1 class="signin__header__title">{{ $t('app.title', { app: app.appName }) }}</h1>
    </template>
    <template v-if="!app.isMobile" #sidebar-nav>
      <div class="signin__action">
        <section class="signin__action__slogan">
          {{ $t('login.signin_slogan') }}
        </section>
        <VBtn
          color="primary-variant"
          rounded
          width="300"
          :loading="isAuthenticating"
          @click.prevent="performLogin"
        >
          {{ $t('terms.signin') }}
        </VBtn>
      </div>
    </template>
    <template v-else #topnav>
      <h1 class="signin__header__title">{{ $t('app.title', { app: settings.appName }) }}</h1>
      <div class="signin__action">
        <section class="signin__action__slogan">
          {{ $t('login.signin_slogan') }}
        </section>
        <VBtn
          color="primary-variant"
          rounded
          width="300"
          :loading="isAuthenticating"
          @click.prevent="performLogin"
        >
          {{ $t('terms.signin') }}
        </VBtn>
      </div>
    </template>

    <template #main-body>
      <div class="main__body__content">
        <VSheet :elevation="!app.isMobile ? 1 : 0" class="main__body__content__card mb-4">
          <VContainer fluid>
            <VRow>
              <VCol cols="2" class="main__body__content__card__icon">
                <VIcon :icon="mdiShieldLockOutline"></VIcon>
              </VCol>
              <VCol cols="10">
                <i18n-t keypath="slogans.elevate_to_orbit.main" scope="global">
                  <template #term1>
                    <strong>{{ $t('slogans.elevate_to_orbit.term1') }}</strong>
                  </template>
                  <template #term2>
                    <br />
                    {{ $t('slogans.elevate_to_orbit.term2') }}
                  </template>
                </i18n-t>
              </VCol>
            </VRow>
          </VContainer>
        </VSheet>
        <VSheet :elevation="!app.isMobile ? 1 : 0" class="main__body__content__card">
          <VContainer fluid>
            <VRow>
              <VCol cols="2" class="main__body__content__card__icon">
                <VIcon :icon="mdiAccountGroupOutline"></VIcon>
              </VCol>
              <VCol cols="10">
                <i18n-t keypath="slogans.institutions_multi_custody.main" scope="global">
                  <template #term1>
                    <strong>{{ $t('slogans.institutions_multi_custody.term1') }}</strong>
                  </template>
                  <template #term2>
                    <br />
                    <strong>{{ $t('slogans.institutions_multi_custody.term2') }}</strong>
                  </template>
                </i18n-t>
              </VCol>
            </VRow>
          </VContainer>
        </VSheet>
        <div v-if="app.isMobile" class="main__body__content__symbol">
          <VImg :src="appLogoImg"></VImg>
        </div>
      </div>
    </template>
  </PageLayout>
</template>

<script lang="ts" setup>
import { mdiAccountGroupOutline, mdiShieldLockOutline } from '@mdi/js';
import { computed, ref } from 'vue';
import { logger } from '~/core';
import PageLayout from '~/ui/components/PageLayout.vue';
import { i18n } from '~/ui/modules';
import { useAuthStore, useAppStore } from '~/ui/stores';

const app = useAppStore();
const auth = useAuthStore();

const isAuthenticating = ref(false);

const performLogin = async (): Promise<void> => {
  isAuthenticating.value = true;
  await auth
    .signIn()
    .then(() => auth.afterLoginRedirect())
    .catch((e: Error) => {
      logger.error(`Authentication failed`, e);

      app.sendNotification({
        message: i18n.global.t('login.auth_failed'),
        type: 'error',
      });
    })
    .finally(() => {
      isAuthenticating.value = false;
    });
};

const appLogoImg = computed(() => {
  return app.isDarkTheme ? '/images/app-logo-dark.png' : '/images/app-logo-light.png';
});

const pageBackgroundColor = computed(() => {
  return app.isDarkTheme ? undefined : 'surface';
});
</script>

<style scoped lang="scss">
@use '~/ui/styles/Variables' as variables;

.signin {
  &__header {
    &__title {
      text-align: center;
      margin-top: calc(var(--ds-bdu) * 6);
      margin-bottom: calc(var(--ds-bdu) * 6);

      @media only screen and (max-width: variables.$device-breakpoint) {
        margin: calc(var(--ds-bdu) * 2) 0;
      }
    }
  }

  &__action {
    height: 100%;
    padding: calc(var(--ds-bdu) * 4) calc(var(--ds-bdu) * 4);
    display: flex;
    align-items: center;
    justify-content: center;
    flex-direction: column;

    @media only screen and (max-width: variables.$device-breakpoint) {
      background-color: rgb(var(--ds-primary));
      color: rgb(var(--ds-on-primary));
    }

    &__slogan {
      margin-bottom: calc(var(--ds-bdu) * 2);
    }
  }
}

.theme--dark {
  .main {
    &__body {
      &__content {
        @media (prefers-color-scheme: dark) {
          &:after {
            background-image: url('/images/app-logo-dark.png');
          }
        }
      }
    }
  }
}

.main {
  &__body {
    &__content {
      display: flex;
      flex-direction: column;
      height: var(--ds-device-sm);
      align-self: center;
      margin: 0;
      min-height: var(--ds-device-sm);
      min-width: var(--ds-device-sm);
      max-width: var(--ds-device-lg);
      padding: calc(var(--ds-bdu) * 10);
      position: relative;

      @media only screen and (max-width: variables.$device-breakpoint) {
        width: 100%;
        min-width: 100%;
        max-width: 100%;
        min-height: 0;
        height: auto;
        margin-top: calc(var(--ds-bdu) * 1);
        padding: calc(var(--ds-bdu) * 2) calc(var(--ds-bdu) * 2);
        min-height: 0;

        &__symbol {
          width: 100%;
          display: block;
          align-items: center;
          margin-top: calc(var(--ds-bdu) * 4);
          margin-bottom: calc(var(--ds-bdu) * 2);

          .v-img {
            width: 40%;
            min-width: calc(var(--ds-device-sm) / 4);
            margin: 0 auto;
          }
        }
      }

      @media only screen and (min-width: variables.$device-breakpoint) {
        &:after {
          content: ' ';
          display: block;
          position: absolute;
          top: 140px;
          left: 160px;
          width: 100%;
          height: 100%;
          min-height: 200px;
          max-height: 500px;
          background-image: url('/images/app-logo-light.png');
          background-repeat: no-repeat;
          background-position: 40% bottom;
          background-size: contain;
          z-index: 0;
        }
      }

      &__card {
        z-index: 1;
        font-size: var(--ds-font-size-lg);
        width: calc(var(--ds-bdu) * 48);
        background-color: rgb(var(--ds-primary-variant-darker));
        color: rgb(var(--ds-on-primary-variant-darker));

        @media only screen and (max-width: variables.$device-breakpoint) {
          width: 100%;
          background-color: rgb(var(--ds-background));
          color: rgb(var(--ds-on-background));
        }

        &__icon {
          font-size: var(--ds-font-size-xxl);
          line-height: calc(var(--ds-bdu) * 6);
          text-align: center;
        }
      }
    }
  }
}
</style>
