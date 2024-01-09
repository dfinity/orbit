<template>
  <PageLayout :hide-sidebar="hideSidebar">
    <template #main-header>
      <div class="not-found pb-16">
        <header class="text-h3 not-found__title">
          {{ $t('not_found.title') }}
        </header>
        <p class="text-h6">
          {{ $t('not_found.description') }}
        </p>
        <VBtn color="primary-variant mt-8" :append-icon="mdiLink" :to="{ name: defaultHomeRoute }">
          {{ $t('not_found.btn_back') }}
        </VBtn>
      </div>
    </template>
  </PageLayout>
</template>

<script lang="ts" setup>
import { mdiLink } from '@mdi/js';
import { computed } from 'vue';
import PageLayout from '~/ui/components/PageLayout.vue';
import { defaultHomeRoute } from '~/ui/modules';
import { useSessionStore, useAppStore } from '~/ui/stores';

const session = useSessionStore();
const app = useAppStore();

const hideSidebar = computed(() => {
  if (app.isMobile) {
    return false;
  }

  return !session.isAuthenticated;
});
</script>

<style scoped lang="scss">
.not-found {
  text-align: center;
  margin-top: calc(var(--ds-bdu) * 10);

  &__title {
    color: rgb(var(--ds-primary-variant));
  }
}
</style>
