<template>
  <PageLayout>
    <template #main-header>
      <PageHeader :title="pageTitle" :subtitle="pageSubtitle" :breadcrumbs="props.breadcrumbs" />
    </template>
    <template #main-body>
      <VRow>
        <VCol cols="12">
          <VCard variant="flat">
            <VCardText>
              <VCol cols="12" class="pb-0">
                <VTextField
                  v-model="session.principal"
                  :label="$t('app.user_id')"
                  variant="plain"
                  readonly
                >
                  <template #append>
                    <VBtn
                      size="x-small"
                      variant="text"
                      :icon="mdiContentCopy"
                      @click="
                        copyToClipboard({
                          textToCopy: session.principal,
                          sendNotification: true,
                        })
                      "
                    />
                  </template>
                </VTextField>
              </VCol>
            </VCardText>
          </VCard>
        </VCol>
      </VRow>
    </template>
  </PageLayout>
</template>

<script lang="ts" setup>
import { mdiContentCopy } from '@mdi/js';
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import PageLayout from '~/components/PageLayout.vue';
import PageHeader from '~/components/layouts/PageHeader.vue';
import { useSessionStore } from '~/stores/session.store';
import type { PageProps } from '~/types/app.types';
import { copyToClipboard } from '~/utils/app.utils';

const props = withDefaults(defineProps<PageProps>(), {
  title: undefined,
  breadcrumbs: () => [],
});
const i18n = useI18n();
const pageTitle = computed(() => props.title || i18n.t('pages.user_settings.title'));
const pageSubtitle = computed(() => i18n.t('pages.user_settings.subtitle'));
const session = useSessionStore();
</script>
