<template>
  <ErrorPage v-if="statusCode !== RouteStatusCode.Success" :status="statusCode" />
  <RouterView v-else :key="walletId" />
</template>

<script lang="ts" setup>
import { onMounted, watch } from 'vue';
import { useTheme } from 'vuetify';
import { useAppStore } from '~/stores/app.store';
import { initWorkers } from './workers';
import ErrorPage from '~/pages/ErrorPage.vue';
import { ref } from 'vue';
import { RouteStatusCode } from '~/configs/routes.config';
import { useRouter } from 'vue-router';
import { WALLET_ID_QUERY_PARAM } from '~/core/constants.core';

const app = useAppStore();
const vuetifyTheme = useTheme();
const statusCode = ref<RouteStatusCode>(RouteStatusCode.Success);
const router = useRouter();
const walletId = ref<string | number>(0);

watch(
  () => router.currentRoute.value,
  route => {
    const queryParam = route.query?.[WALLET_ID_QUERY_PARAM];
    if (!queryParam) {
      return;
    }

    const newId = Array.isArray(queryParam) ? queryParam?.[0] ?? '' : queryParam;

    if (newId !== walletId.value) {
      walletId.value = newId;
    }
  },
  { deep: true, immediate: true },
);

watch(
  () => app.theme,
  theme => {
    vuetifyTheme.global.name.value = theme;
  },
  {
    immediate: true,
  },
);

watch(
  () => app.loading,
  loading => {
    if (!loading) {
      statusCode.value = app.routeStatusCode;
    }
  },
);

onMounted(async () => await initWorkers());
</script>
