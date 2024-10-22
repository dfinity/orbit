<template>
  {{ assetNames.join(', ') }}
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { useStationStore } from '~/stores/station.store';

const props = defineProps<{
  assetIds: string[];
}>();

const station = useStationStore();

const assetNames = computed(() => {
  return props.assetIds.map(
    id =>
      station.configuration.details.supported_assets.find(token => token.id === id)?.symbol || id,
  );
});
</script>
