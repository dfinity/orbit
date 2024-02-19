<template>
  <PageLayout :hide-sidebar="hideSidebar">
    <template #main-header>
      <VCard class="ma-4" color="background" variant="flat">
        <div class="d-flex flex-no-wrap justify-space-between">
          <VAvatar class="ma-3" size="180" rounded="0">
            <VIcon size="100%" color="primary" :icon="icon" />
          </VAvatar>
          <div class="flex-grow-1 my-8">
            <VCardTitle class="text-h4">{{ pageTitle }}</VCardTitle>
            <VCardSubtitle>{{ pageSubtitle }}</VCardSubtitle>
            <VCardActions v-if="showBackToHome">
              <VBtn
                color="primary-variant mt-8 mx-2"
                variant="tonal"
                size="small"
                :prepend-icon="mdiHome"
                :to="{
                  name: defaultHomeRoute,
                }"
              >
                {{ $t('app.btn_home_back') }}
              </VBtn>
            </VCardActions>
          </div>
        </div>
      </VCard>
    </template>
  </PageLayout>
</template>

<script lang="ts" setup>
import {
  mdiAccountOff,
  mdiAlertCircle,
  mdiHome,
  mdiLockOutline,
  mdiMagnifyRemoveOutline,
} from '@mdi/js';
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import PageLayout from '~/components/PageLayout.vue';
import { RouteStatusCode, defaultHomeRoute } from '~/configs/routes.config';
import { useAppStore } from '~/stores/app.store';
import { useSessionStore } from '~/stores/session.store';

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
const i18n = useI18n();
const pageTitle = computed(() => {
  switch (props.status) {
    case RouteStatusCode.Unauthorized:
      return i18n.t('pages.unauthorized.title');
    case RouteStatusCode.Disconnected:
      return i18n.t('pages.disconnected.title');
    case RouteStatusCode.NotFound:
      return i18n.t('pages.not_found.title');
    default:
      return i18n.t('pages.error.title');
  }
});

const pageSubtitle = computed(() => {
  switch (props.status) {
    case RouteStatusCode.Unauthorized:
      return i18n.t('pages.unauthorized.subtitle');
    case RouteStatusCode.Disconnected:
      return i18n.t('pages.disconnected.subtitle');
    case RouteStatusCode.NotFound:
      return i18n.t('pages.not_found.subtitle');
    default:
      return i18n.t('pages.error.subtitle');
  }
});

const icon = computed(() => {
  switch (props.status) {
    case RouteStatusCode.Unauthorized:
      return mdiLockOutline;
    case RouteStatusCode.Disconnected:
      return mdiAccountOff;
    case RouteStatusCode.NotFound:
      return mdiMagnifyRemoveOutline;
    default:
      return mdiAlertCircle;
  }
});

const showBackToHome = computed(() => {
  switch (props.status) {
    case RouteStatusCode.Unauthorized:
    case RouteStatusCode.Error:
    case RouteStatusCode.NotFound:
      return true;
    default:
      return false;
  }
});

const hideSidebar = computed(() => {
  if (app.isMobile) {
    return false;
  }

  return !session.isAuthenticated;
});
</script>
