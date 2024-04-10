<template>
  <PageLayout>
    <template #sidebar>
      <AppSidebar width="320" class="logo-markers-bg--contain" :language-selector="app.isMobile">
        <template #nav>
          <div class="d-flex flex-grow-1 d-flex-column justify-start align-start">
            <p class="px-4 text-h4 mt-16 text-primary font-weight-bold">
              <i18n-t keypath="landing.sidebar_slogan.main" scope="global">
                <template #term1>
                  <br />
                  <span class="font-weight-regular text-white">
                    {{ $t('landing.sidebar_slogan.term1') }}
                  </span>
                </template>
                <template #term2>
                  <br />
                  <span class="font-weight-regular text-white">
                    {{ $t('landing.sidebar_slogan.term2') }}
                  </span>
                </template>
              </i18n-t>
            </p>
          </div>
        </template>
      </AppSidebar>
    </template>
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
import AppSidebar from '~/components/layouts/AppSidebar.vue';
import { useAppStore } from '~/stores/app.store';
import { PageProps } from '~/types/app.types';

const props = withDefaults(defineProps<PageProps>(), {
  breadcrumbs: () => [],
});

const i18n = useI18n();
const app = useAppStore();

const pageTitle = computed(() => props.title || i18n.t('pages.add_wallet.initialization_title'));
</script>
