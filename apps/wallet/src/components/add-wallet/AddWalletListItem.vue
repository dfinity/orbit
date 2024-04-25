<template>
  <VListItem data-test-id="add-wallet-item" :to="`/${$route.params.locale}/add-wallet`">
    <VIcon left class="mr-1">
      {{ mdiPlus }}
    </VIcon>
    <span>{{ $t('wallets.add_wallet_list_item') }}</span>
  </VListItem>
</template>

<script lang="ts" setup>
import { mdiPlus } from '@mdi/js';
import { VListItem } from 'vuetify/components';
</script>
