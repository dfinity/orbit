<template>
  <RouterView />
</template>

<script lang="ts" setup>
import { watch } from 'vue';
import { useTheme } from 'vuetify';
import { useSettingsStore } from '~/ui/stores';

const settings = useSettingsStore();
const vuetifyTheme = useTheme();

watch(
  () => settings.theme,
  theme => {
    vuetifyTheme.global.name.value = theme;
  },
  {
    immediate: true,
  },
);
</script>
