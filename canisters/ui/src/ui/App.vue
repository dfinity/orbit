<template>
  <RouterView />
</template>

<script lang="ts" setup>
import { onMounted, watch } from 'vue';
import { useTheme } from 'vuetify';
import { useAppStore } from '~/ui/stores/app';
import { initWorkers } from './modules';

const app = useAppStore();
const vuetifyTheme = useTheme();

watch(
  () => app.theme,
  theme => {
    vuetifyTheme.global.name.value = theme;
  },
  {
    immediate: true,
  },
);
onMounted(async () => await initWorkers());
</script>
