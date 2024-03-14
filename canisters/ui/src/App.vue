<template>
  <ErrorPage v-if="statusCode !== RouteStatusCode.Success" :status="statusCode" />
  <RouterView v-else />
</template>

<script lang="ts" setup>
import { onMounted, watch } from 'vue';
import { useTheme } from 'vuetify';
import { useAppStore } from '~/stores/app.store';
import { initWorkers } from './workers';
import ErrorPage from '~/pages/ErrorPage.vue';
import { ref } from 'vue';
import { RouteStatusCode } from '~/configs/routes.config';

const app = useAppStore();
const vuetifyTheme = useTheme();
const statusCode = ref<RouteStatusCode>(RouteStatusCode.Success);

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
  }
);

onMounted(async () => await initWorkers());
</script>
