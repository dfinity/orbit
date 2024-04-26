<template>
  <PageLayout>
    <template #sidebar="{ showWarningBanner }">
      <AppSidebar
        class="logo-markers-bg--contain"
        :class="{
          ['warning-banner--offset']: showWarningBanner,
        }"
        :language-selector="app.isMobile"
      >
        <template #nav>
          <SidebarHighlights />
        </template>
      </AppSidebar>
    </template>
    <template #main-body>
      <AddStationScreen :title="pageTitle" />
    </template>
  </PageLayout>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import PageLayout from '~/components/PageLayout.vue';
import AddStationScreen from '~/components/add-station/AddStationScreen.vue';
import AppSidebar from '~/components/layouts/AppSidebar.vue';
import SidebarHighlights from '~/components/ui/SidebarHighlights.vue';
import { useAppStore } from '~/stores/app.store';
import { PageProps } from '~/types/app.types';

const props = withDefaults(defineProps<PageProps>(), {
  breadcrumbs: () => [],
});

const i18n = useI18n();
const app = useAppStore();

const pageTitle = computed(() => props.title || i18n.t('pages.add_station.initialization_title'));
</script>
