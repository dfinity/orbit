<template>
  <RouterView />
</template>

<script lang="ts" setup>
import { watch } from 'vue';
import { useTheme } from 'vuetify';
import { useAppStore } from '~/ui/stores/app';

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

</script>
