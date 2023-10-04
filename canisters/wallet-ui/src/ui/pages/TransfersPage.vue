<template>
  <PageLayout v-if="activeBank.hasAccount">
    <template #main-header>
      <VContainer class="pt-16 pb-16 pl-8 pr-8" fluid>
        <VRow>
          <VCol sm="12">
            <h1 class="text-h4">{{ $t('terms.transfers') }}</h1>
          </VCol>
        </VRow>
      </VContainer>
    </template>
  </PageLayout>
  <PageLayout v-else>
    <template #main-header>
      <VContainer class="pt-16 pb-16 pl-8 pr-8" fluid>
        <VRow>
          <VCol sm="12">
            <h1 class="text-h4">{{ $t('banks.no_bank_account') }}</h1>
            <p class="text-subtitle">{{ $t('banks.please_register_to_continue') }}</p>
          </VCol>
        </VRow>
      </VContainer>
    </template>
  </PageLayout>
</template>

<script lang="ts" setup>
import PageLayout from '~/ui/components/PageLayout.vue';
import { useActiveBankStore } from '~/ui/stores';

const activeBank = useActiveBankStore();
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
