<template>
  <PageLayout>
    <template #main-header>
      <VContainer class="pt-16 pb-16 pl-8 pr-8" fluid>
        <VRow>
          <VCol md="6" sm="12">
            <h1 class="text-h4">{{ $t('home.welcome_back') }}</h1>
            <p class="info-box">
              <VIcon :icon="mdiBellRing" size="18" class="mr-1" />
              <span>{{ $t('home.notifications.none') }}</span>
            </p>
          </VCol>
          <VCol md="6" sm="12" class="header-actions">
            <VBtn rounded color="primary-variant" :prepend-icon="mdiSend">
              {{ $t('terms.new_transfer') }}
            </VBtn>
            <VBtn rounded color="primary-variant" :prepend-icon="mdiCogs">
              {{ $t('terms.settings') }}
            </VBtn>
          </VCol>
        </VRow>
      </VContainer>
    </template>
    <template #main-body>
      <VContainer class="pl-8 pr-8" fluid>
        <VRow>
          <VCol cols="12" md="4">
            <VCard color="surface" height="100%">
              <VCardTitle>{{ $t('terms.wallets') }}</VCardTitle>
              <VCardText class="text-center text-h3 pt-8 pb-16">0</VCardText>
            </VCard>
          </VCol>
          <VCol cols="12" md="4">
            <VCard color="surface" height="100%">
              <VCardTitle>{{ $t('terms.transactions') }}</VCardTitle>
              <VCardText class="text-center text-h3 pt-8 pb-16">
                <VRow>
                  <VCol cols="6" class="transactions__card">
                    <span>0</span>
                    <span class="text-subtitle-1">{{ $t('terms.completed') }}</span>
                  </VCol>
                  <VCol cols="6" class="transactions__card">
                    <span>0</span>
                    <span class="text-subtitle-2">{{ $t('terms.pending') }}</span>
                  </VCol>
                </VRow>
              </VCardText>
            </VCard>
          </VCol>
          <VCol cols="12" md="4">
            <VCard color="surface" height="100%">
              <VCardTitle>{{ $t('terms.address_book') }}</VCardTitle>
              <VCardText class="text-center text-h3 pt-8 pb-16">0</VCardText>
            </VCard>
          </VCol>
        </VRow>
      </VContainer>
    </template>
  </PageLayout>
</template>

<script lang="ts" setup>
import { mdiBellRing, mdiSend, mdiCogs } from '@mdi/js';
import PageLayout from '~/ui/components/PageLayout.vue';
</script>

<style scoped lang="scss">
.info-box {
  display: flex;
  justify-content: start;
  align-items: center;
}

.transactions {
  &__card {
    display: flex;
    justify-content: center;
    flex-direction: column;
    gap: var(--ds-bdu);

    &:first-child {
      border-right: var(--ds-border-width) var(--ds-border-style) rgb(var(--ds-background-border));
    }
  }
}

.header-actions {
  display: flex;
  justify-content: end;
  align-items: center;
  gap: calc(var(--ds-bdu) * 2);
}

.page-layout--mobile {
  .header-actions {
    justify-content: center;
  }
}
</style>
