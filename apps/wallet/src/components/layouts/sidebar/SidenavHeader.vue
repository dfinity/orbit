<template>
  <VContainer class="py-4">
    <VRow>
      <VCol cols="12" class="d-flex">
        <BrandLogo variation="dark" class="flex-grow-1" height="22" />
        <LanguageSelector v-if="props.languageSelector" />
      </VCol>
      <VCol v-if="showStationSelector" cols="12" class="d-flex ga-2 align-center">
        <StationSelector bg-color="background" class="flex-grow-1" />
        <RouterLink :to="{ name: Routes.SystemSettings }">
          <VIcon :icon="mdiCog" size="small" />
        </RouterLink>
      </VCol>
    </VRow>
  </VContainer>
</template>

<script lang="ts" setup>
import { mdiCog } from '@mdi/js';
import { computed } from 'vue';
import { VCol, VContainer, VIcon, VRow } from 'vuetify/components';
import BrandLogo from '~/components/BrandLogo.vue';
import LanguageSelector from '~/components/LanguageSelector.vue';
import StationSelector from '~/components/StationSelector.vue';
import { Routes } from '~/configs/routes.config';
import { useSessionStore } from '~/stores/session.store';

const props = withDefaults(
  defineProps<{
    languageSelector?: boolean;
  }>(),
  {
    languageSelector: false,
  },
);

const session = useSessionStore();
const showStationSelector = computed(() => session.isAuthenticated && session.hasStations);
</script>
