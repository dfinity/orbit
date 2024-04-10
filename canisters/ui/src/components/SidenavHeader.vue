<template>
  <VContainer>
    <VRow>
      <VCol cols="12" class="d-flex">
        <BrandLogo variation="dark" class="flex-grow-1" />
        <LanguageSelector v-if="props.languageSelector" />
      </VCol>
      <VCol v-if="showWalletSelector" cols="12">
        <WalletSelector />
      </VCol>
    </VRow>
  </VContainer>
</template>

<script lang="ts" setup>
import { computed } from 'vue';
import { VCol, VContainer, VRow } from 'vuetify/components';
import BrandLogo from '~/components/BrandLogo.vue';
import LanguageSelector from '~/components/LanguageSelector.vue';
import WalletSelector from '~/components/WalletSelector.vue';
import { useSessionStore } from '~/stores/session.store';

const props = withDefaults(
  defineProps<{
    languageSelector?: boolean;
  }>(),
  {
    languageSelector: false,
  },
);

const session = useSessionStore();
const showWalletSelector = computed(() => session.isAuthenticated && session.hasWallets);
</script>
