<template>
  <PageLayout hide-sidebar>
    <template #main-header>
      <div class="not-found pb-16">
        <header class="text-h3 not-found__title">
          {{ $t('not_found.title') }}
        </header>
        <p class="text-h6">
          {{ $t('not_found.description') }}
        </p>
        <VBtn color="primary-variant mt-8" :append-icon="mdiLink" to="/">
          {{ $t('not_found.btn_back') }}
        </VBtn>
      </div>
    </template>
    <template #toolbar-context>
      <div class="app-name">{{ $t('app.title', { app: settings.appName }) }}</div>
    </template>
  </PageLayout>
</template>

<script lang="ts" setup>
import PageLayout from '~/ui/components/PageLayout.vue';
import { mdiLink } from '@mdi/js';
import { useSettingsStore } from '~/ui/stores';

const settings = useSettingsStore();
</script>

<style scoped lang="scss">
.app-name {
  text-indent: var(--ds-bdu);
  font-size: var(--ds-font-size-xl);
  font-weight: 600;
}

.not-found {
  text-align: center;
  margin-top: calc(var(--ds-bdu) * 10);

  &__title {
    color: rgb(var(--ds-primary-variant));
  }
}
</style>
