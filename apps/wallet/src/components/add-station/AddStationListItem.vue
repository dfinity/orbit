<template>
  <VListItem data-test-id="add-station-item" :to="`/${$route.params.locale}/add-station`">
    <VIcon left class="mr-1">
      {{ mdiPlus }}
    </VIcon>
    <span>{{ $t('stations.add_station_list_item') }}</span>
  </VListItem>
</template>

<script lang="ts" setup>
import { mdiPlus } from '@mdi/js';
import { VListItem } from 'vuetify/components';
</script>
