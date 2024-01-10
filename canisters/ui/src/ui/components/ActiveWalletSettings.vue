<template>
  <VContainer class="wallet-form">
    <VRow>
      <VCol cols="12 wallet-form__title">{{ $t('terms.general') }}</VCol>
      <VCol cols="12" class="py-0">
        <VTextField
          v-model="wallet.currentUser.me.id"
          :prepend-inner-icon="mdiIdentifier"
          variant="solo"
          density="compact"
          readonly
        />
      </VCol>
    </VRow>
  </VContainer>
</template>

<script lang="ts" setup>
import { mdiIdentifier } from '@mdi/js';
import { useWalletStore } from '~/ui/stores/wallet';

const wallet = useWalletStore();
</script>

<style scoped lang="scss">
.wallet-form {
  &__title {
    font-weight: bold;
    font-size: var(--ds-font-size-xs);
  }
}
</style>
