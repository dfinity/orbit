<template>
  <PageLayout>
    <template #main-header>
      <VContainer class="pt-16 pb-16 pl-8 pr-8" fluid>
        <VRow>
          <VCol md="6" sm="12">
            <h1 class="text-h4">{{ $t('terms.wallets') }}</h1>
          </VCol>
          <VCol md="6" sm="12" class="header-actions">
            <VBtn rounded color="primary-variant" :prepend-icon="mdiSend">
              {{ $t('terms.new_transfer') }}
            </VBtn>
            <VBtn rounded color="primary-variant" :prepend-icon="mdiCogs">
              {{ $t('terms.policies') }}
            </VBtn>
          </VCol>
        </VRow>
      </VContainer>
    </template>
  </PageLayout>
</template>

<script lang="ts" setup>
import { mdiSend, mdiCogs } from '@mdi/js';
import PageLayout from '~/ui/components/PageLayout.vue';
</script>

<style scoped lang="scss">

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
