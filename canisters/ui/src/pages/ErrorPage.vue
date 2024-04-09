<template>
  <PageLayout :hide-sidebar="hideSidebar">
    <template #main-header>
      <VCard class="ma-4" color="background" variant="flat">
        <ErrorScreen
          v-if="props.status == RouteStatusCode.Unauthorized"
          :icon="mdiLockOutline"
          :title="$t('pages.unauthorized.title')"
          :subtitle="$t('pages.unauthorized.subtitle')"
          show-back-to-home
        />

        <ErrorScreen
          v-else-if="props.status == RouteStatusCode.NotFound"
          :icon="mdiMagnifyRemoveOutline"
          :title="$t('pages.not_found.title')"
          :subtitle="$t('pages.not_found.subtitle')"
          show-back-to-home
        />

        <ErrorScreen
          v-else-if="
            props.status == RouteStatusCode.Disconnected &&
            wallet.connectionError === WalletConnectionError.NOT_FOUND_USER_IDENTITY
          "
          :icon="mdiMagnifyRemoveOutline"
          :title="$t('pages.disconnected.title_not_found_user_identity')"
          :subtitle="$t('pages.disconnected.subtitle_not_found_user_identity')"
          :show-back-to-home="false"
        >
          <div class="mt-10 w-md-75">
            <VTextField
              :model-value="session.principal"
              variant="outlined"
              :label="$t('terms.principal')"
              readonly
              :append-inner-icon="mdiContentCopy"
              @click:append-inner="
                copyToClipboard({ textToCopy: session.principal, sendNotification: true })
              "
            />
          </div>
        </ErrorScreen>

        <ErrorScreen
          v-else-if="
            props.status == RouteStatusCode.Disconnected &&
            wallet.connectionError === WalletConnectionError.OTHER_WALLET_ERROR
          "
          :icon="mdiMagnifyRemoveOutline"
          :title="
            wallet.connectionErrorMessage || $t('pages.disconnected.title_other_wallet_error')
          "
          :subtitle="
            wallet.connectionErrorMessage || $t('pages.disconnected.subtitle_other_wallet_error')
          "
          :show-back-to-home="false"
        ></ErrorScreen>

        <ErrorScreen
          v-else-if="
            props.status == RouteStatusCode.Disconnected &&
            wallet.connectionError === WalletConnectionError.CANISTER_ERROR
          "
          :icon="mdiMagnifyRemoveOutline"
          :title="$t('pages.disconnected.title_canister_error')"
          :subtitle="$t('pages.disconnected.subtitle_canister_error')"
          :show-back-to-home="false"
        >
          <VTextarea
            v-model="wallet.connectionErrorMessage"
            :variant="'outlined'"
            readonly
            auto-grow
          />
        </ErrorScreen>

        <ErrorScreen
          v-else
          :icon="mdiAlertCircle"
          :title="$t('pages.error.title')"
          :subtitle="$t('pages.error.subtitle')"
          show-back-to-home
        ></ErrorScreen>
      </VCard>
    </template>
  </PageLayout>
</template>

<script lang="ts" setup>
import { mdiAlertCircle, mdiContentCopy, mdiLockOutline, mdiMagnifyRemoveOutline } from '@mdi/js';
import { computed } from 'vue';
import { VTextarea } from 'vuetify/components';
import PageLayout from '~/components/PageLayout.vue';
import ErrorScreen from '~/components/error/ErrorScreen.vue';
import { RouteStatusCode } from '~/configs/routes.config';
import { useAppStore } from '~/stores/app.store';
import { useSessionStore } from '~/stores/session.store';
import { WalletConnectionError, useWalletStore } from '~/stores/wallet.store';
import { copyToClipboard } from '~/utils/app.utils';

const props = withDefaults(
  defineProps<{
    status?: RouteStatusCode;
  }>(),
  {
    status: RouteStatusCode.NotFound,
  },
);
const session = useSessionStore();
const wallet = useWalletStore();
const app = useAppStore();

const hideSidebar = computed(() => {
  if (app.isMobile) {
    return false;
  }

  return !session.isAuthenticated;
});
</script>
