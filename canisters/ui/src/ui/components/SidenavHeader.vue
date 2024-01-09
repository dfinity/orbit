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
import BrandLogo from '~/ui/components/BrandLogo.vue';
import WalletSelector from '~/ui/components/WalletSelector.vue';
import { useSessionStore } from '~/ui/stores';

const session = useSessionStore();
</script>
