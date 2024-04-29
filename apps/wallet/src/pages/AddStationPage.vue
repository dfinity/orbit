<template>
  <PageLayout>
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
import { PageProps } from '~/types/app.types';

const i18n = useI18n();

const props = withDefaults(defineProps<PageProps>(), {
  breadcrumbs: () => [],
});

const pageTitle = computed(() => props.title || i18n.t('pages.add_station.add_station_title'));
</script>
