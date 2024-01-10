<template>
  <PageLayout :background-color="pageBackgroundColor" hide-toolbar-context hide-sidebar>
    <template #main-body> connect to wallet </template>
  </PageLayout>
</template>

<script lang="ts" setup>
import { computed } from 'vue';
import PageLayout from '~/ui/components/PageLayout.vue';
import { useAppStore } from '~/ui/stores/app';

const app = useAppStore();

const pageBackgroundColor = computed(() => {
  return app.isDarkTheme ? undefined : 'surface';
});
</script>
