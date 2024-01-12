<template>
  <PageLayout>
    <template #main-header>
      <VContainer class="pt-8 pb-8 pl-8 pr-8" fluid>
        <VRow>
          <VCol cols="12">
            <h1 class="text-h4">{{ $t('pages.user_settings.title') }}</h1>
            <p>{{ $t('pages.user_settings.subtitle') }}</p>
          </VCol>
        </VRow>
      </VContainer>
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
                  variant="underlined"
                  readonly
                />
              </VCol>
            </VCardText>
          </VCard>
        </VCol>
      </VRow>
    </template>
  </PageLayout>
</template>

<script lang="ts" setup>
import PageLayout from '~/ui/components/PageLayout.vue';
import { useSessionStore } from '~/ui/stores/session';

const session = useSessionStore();
</script>
