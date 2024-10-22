<template>
  <TextOverflow
    v-if="format === AddressFormat.ICPNative"
    :max-length="32"
    :text="address"
  ></TextOverflow>

  <TextOverflow
    v-else-if="format === AddressFormat.ICRC1"
    :text="address"
    :overflow-position="shortenIcrc1Address"
  ></TextOverflow>

  <!-- unknown format -->
  <TextOverflow v-else :text="address" :max-length="32"></TextOverflow>
</template>

<script setup lang="ts">
import { AddressFormat } from '~/types/chain.types';
import TextOverflow from './TextOverflow.vue';
import { shortenIcrc1Address } from '~/utils/asset.utils';

defineProps<{
  address: string;
  format: AddressFormat | string | undefined;
}>();
</script>
