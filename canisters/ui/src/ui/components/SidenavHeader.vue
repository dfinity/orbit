<template>
  <VContainer>
    <VRow>
      <VCol cols="12">
        <BrandLogo variation="dark" />
      </VCol>
      <VCol v-if="auth.isAuthenticated" cols="12">
        <WalletSelector />
      </VCol>
    </VRow>
  </VContainer>
</template>

<script lang="ts" setup>
import BrandLogo from '~/ui/components/BrandLogo.vue';
import WalletSelector from '~/ui/components/WalletSelector.vue';
import { useAuthStore } from '~/ui/stores';

const auth = useAuthStore();
</script>
