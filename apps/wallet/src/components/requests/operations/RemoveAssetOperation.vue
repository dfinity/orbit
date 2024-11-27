<template>
  <div v-if="isListMode" class="d-flex flex-column ga-0 text-caption">
    <RequestOperationListRow v-if="formValue.id">
      <template #name>{{ $t('terms.id') }}</template>
      <template #content>
        {{ formValue.id }}
      </template>
    </RequestOperationListRow>
  </div>
  <VProgressCircular v-else-if="loading" indeterminate />
  <AssetForm v-else :model-value="formValue" mode="view" />
</template>

<script setup lang="ts">
import { Ref, computed, onBeforeMount, ref } from 'vue';
import logger from '~/core/logger.core';
import { Asset, RemoveAssetOperation, Request } from '~/generated/station/station.did';
import { useStationStore } from '~/stores/station.store';
import RequestOperationListRow from '../RequestOperationListRow.vue';
import AssetForm from '~/components/assets/AssetForm.vue';
import { VProgressCircular } from 'vuetify/components';

const props = withDefaults(
  defineProps<{
    request: Request;
    operation: RemoveAssetOperation;
    mode?: 'list' | 'detail';
  }>(),
  {
    mode: 'list',
  },
);

const isListMode = computed(() => props.mode === 'list');
const formValue: Ref<Partial<Asset>> = ref({});
const loading = ref(false);
const assetLoadingFailed = ref(false);
const station = useStationStore();

const fetchDetails = async () => {
  assetLoadingFailed.value = false;
  try {
    if (loading.value || isListMode.value) {
      return;
    }

    loading.value = true;
    const currentEntry: Partial<Asset> = await station.service
      .getAsset(
        {
          asset_id: props.operation.input.asset_id,
        },
        true,
      )
      .then(response => response.asset)
      .catch(() => ({
        id: props.operation.input.asset_id,
      }));

    formValue.value = currentEntry;
  } catch (e) {
    logger.error('Failed to fetch asset details', e);
    assetLoadingFailed.value = true;
  } finally {
    loading.value = false;
  }
};

onBeforeMount(() => {
  const entry: Partial<Asset> = {};
  entry.id = props.operation.input.asset_id;

  formValue.value = entry;

  fetchDetails();
});
</script>
