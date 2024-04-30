<template>
  <PageLayout>
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
            station.connectionError === ConnectionError.NOT_FOUND_USER_IDENTITY
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
              :label="$t('terms.identity')"
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
            station.connectionError === ConnectionError.OTHER_ERROR
          "
          :icon="mdiMagnifyRemoveOutline"
          :title="
            station.connectionErrorMessage || $t('pages.disconnected.title_other_station_error')
          "
          :subtitle="
            station.connectionErrorMessage || $t('pages.disconnected.subtitle_other_station_error')
          "
          :show-back-to-home="false"
        />

        <ErrorScreen
          v-else-if="
            props.status == RouteStatusCode.Disconnected &&
            station.connectionError === ConnectionError.CANISTER_ERROR
          "
          :icon="mdiMagnifyRemoveOutline"
          :title="$t('pages.disconnected.title_canister_error')"
          :subtitle="$t('pages.disconnected.subtitle_canister_error')"
          :show-back-to-home="false"
        >
          <VTextarea
            v-model="station.connectionErrorMessage"
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
        />
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
import { RouteStatusCode } from '~/configs/routes.config';
import { useSessionStore } from '~/stores/session.store';
import { useStationStore, ConnectionError } from '~/stores/station.store';
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
const station = useStationStore();
</script>
