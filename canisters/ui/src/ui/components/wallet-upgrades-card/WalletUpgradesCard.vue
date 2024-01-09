<template>
  <VCard color="background" variant="flat" :loading="store.loading">
    <VCardTitle class="font-weight-bold">
      {{ $t(`app.wallet_upgrades_card_title`) }}
    </VCardTitle>
    <VCardText> todo </VCardText>
  </VCard>
</template>

<script lang="ts" setup>
import { useStore } from './store';

const store = useStore();
</script>
