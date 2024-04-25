<template>
  <PageLayout>
    <template #main-header>
      <PageHeader :title="pageTitle" :subtitle="pageSubtitle" :breadcrumbs="props.breadcrumbs" />
    </template>
    <template #main-body>
      <PageBody>
        <VCard>
          <VCardText>
            <VTextField
              v-model="session.principal"
              :label="$t('app.user_id')"
              variant="plain"
              readonly
              hide-details
            >
              <template #append-inner>
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
          </VCardText>
        </VCard>
      </PageBody>
    </template>
  </PageLayout>
</template>

<script lang="ts" setup>
import { mdiContentCopy } from '@mdi/js';
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import { VBtn, VCard, VCardText, VTextField } from 'vuetify/components';
import PageLayout from '~/components/PageLayout.vue';
import PageBody from '~/components/layouts/PageBody.vue';
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
