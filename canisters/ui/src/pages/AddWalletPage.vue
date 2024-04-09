<template>
  <PageLayout>
    <template #main-body>
      <AddWalletScreen :title="props.title" />
    </template>
  </PageLayout>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import PageLayout from '~/components/PageLayout.vue';
import AddWalletScreen from '~/components/add-wallet/AddWalletScreen.vue';
import { PageProps } from '~/types/app.types';

const i18n = useI18n();

const props = withDefaults(defineProps<PageProps>(), {
  title: i18n.t('pages.add_wallet.add_wallet_title'),
  breadcrumbs: () => [],
});
</script>
