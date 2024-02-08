<template>
  <VContainer>
    <VRow>
      <VCol cols="12">
        <BrandLogo variation="dark" />
      </VCol>
      <VCol v-if="session.isAuthenticated" cols="12">
        <WalletSelector />
      </VCol>
    </VRow>
  </VContainer>
</template>

<script lang="ts" setup>
import BrandLogo from '~/components/BrandLogo.vue';
import WalletSelector from '~/components/WalletSelector.vue';
import { useSessionStore } from '~/stores/session.store';

const session = useSessionStore();
</script>
