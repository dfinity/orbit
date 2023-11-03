<template>
  <VContainer class="bank-form">
    <VRow>
      <VCol cols="12 bank-form__title">{{ $t('terms.general') }}</VCol>
      <VCol cols="12" class="py-0">
        <VTextField
          v-model="activeBank.user.id"
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
import { useActiveBankStore } from '~/ui/stores';

const activeBank = useActiveBankStore();
</script>

<style scoped lang="scss">
.bank-form {
  &__title {
    font-weight: bold;
    font-size: var(--ds-font-size-xs);
  }
}
</style>
