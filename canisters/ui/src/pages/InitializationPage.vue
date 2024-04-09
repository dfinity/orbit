<template>
  <PageLayout>
    <template #sidebar-nav><div /></template>
    <template #main-body>
      <AddWalletScreen :title="pageTitle" />
    </template>
  </PageLayout>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import PageLayout from '~/components/PageLayout.vue';
import AddWalletScreen from '~/components/add-wallet/AddWalletScreen.vue';
import { PageProps } from '~/types/app.types';

const i18n = useI18n();

const props = withDefaults(defineProps<PageProps>(), {
  breadcrumbs: () => [],
});

const pageTitle = computed(() => props.title || i18n.t('pages.add_wallet.initialization_title'));
</script>
