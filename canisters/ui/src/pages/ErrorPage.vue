<template>
  <PageLayout>
    <template #sidebar>
      <AppSidebar width="320" class="logo-markers-bg--contain" :language-selector="app.isMobile">
        <template #nav>
          <SidebarHighlights />
        </template>
      </AppSidebar>
    </template>
    <template #main-body>
      <VCard class="ma-4">
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
          :icon="mdiAccountOffOutline"
          :title="$t('pages.disconnected.title_not_found_user_identity')"
          :subtitle="$t('pages.disconnected.subtitle_not_found_user_identity')"
          :show-back-to-home="false"
        >
          <div class="mt-4 w-md-75">
            <VTextField
              :model-value="session.principal"
              variant="solo-filled"
              :label="$t('terms.principal')"
              readonly
              hide-details
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
        />

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
          :icon="mdiAlertCircleOutline"
          :title="$t('pages.error.title')"
          :subtitle="$t('pages.error.subtitle')"
          show-back-to-home
        ></ErrorScreen>
      </VCard>
    </template>
  </PageLayout>
</template>

<script lang="ts" setup>
import {
  mdiAccountOffOutline,
  mdiAlertCircleOutline,
  mdiContentCopy,
  mdiLockOutline,
  mdiMagnifyRemoveOutline,
} from '@mdi/js';
import { VCard, VTextarea, VTextField } from 'vuetify/components';
import PageLayout from '~/components/PageLayout.vue';
import ErrorScreen from '~/components/error/ErrorScreen.vue';
import AppSidebar from '~/components/layouts/AppSidebar.vue';
import SidebarHighlights from '~/components/ui/SidebarHighlights.vue';
import { RouteStatusCode } from '~/configs/routes.config';
import { useAppStore } from '~/stores/app.store';
import { useSessionStore } from '~/stores/session.store';
import { useWalletStore, WalletConnectionError } from '~/stores/wallet.store';
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
const app = useAppStore();
const wallet = useWalletStore();
</script>
