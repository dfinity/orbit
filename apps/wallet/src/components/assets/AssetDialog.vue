<template>
  <VDialog
    v-model="openModel"
    :persistent="loading || saving"
    transition="dialog-bottom-transition"
    scrollable
    :max-width="props.dialogMaxWidth.value"
  >
    <DataLoader
      v-slot="{ data }"
      :load="loadAsset"
      @loading="loading = $event"
      @loaded="asset = $event.asset"
    >
      <VCard>
        <VToolbar color="background">
          <VToolbarTitle>{{ $t('app.asset') }}</VToolbarTitle>
          <VBtn :disabled="loading || saving" :icon="mdiClose" @click="openModel = false" />
        </VToolbar>
        <VCardText v-if="loading" class="py-8">
          <LoadingMessage />
        </VCardText>
        <VCardText v-else>
          <AssetForm
            v-if="data"
            v-model="asset"
            v-model:trigger-submit="triggerSubmit"
            :display="{
              id: true,
            }"
            :disabled="props.readonly.value"
            @submit="save"
            @valid="valid = $event"
          />
        </VCardText>
        <VDivider />
        <VCardActions class="pa-3">
          <VSpacer />
          <VBtn
            v-if="!props.readonly.value"
            :disabled="!canSave"
            :loading="saving"
            color="primary"
            variant="elevated"
            data-test-id="save-asset"
            @click="triggerSubmit = true"
          >
            {{ props.assetId.value ? $t('terms.save') : $t('terms.create') }}
          </VBtn>
        </VCardActions>
      </VCard>
    </DataLoader>
  </VDialog>
</template>
<script lang="ts" setup>
import { mdiClose } from '@mdi/js';
import { computed, ref, toRefs } from 'vue';
import {
  VBtn,
  VCard,
  VCardActions,
  VCardText,
  VDialog,
  VDivider,
  VSpacer,
  VToolbar,
  VToolbarTitle,
} from 'vuetify/components';
import DataLoader from '~/components/DataLoader.vue';
import LoadingMessage from '~/components/LoadingMessage.vue';
import {
  useOnFailedOperation,
  useOnSuccessfulOperation,
} from '~/composables/notifications.composable';
import logger from '~/core/logger.core';
import { Asset, UUID } from '~/generated/station/station.did';
import { useStationStore } from '~/stores/station.store';
import { assertAndReturn } from '~/utils/helper.utils';
import AssetForm from './AssetForm.vue';

const input = withDefaults(
  defineProps<{
    assetId?: UUID;
    open?: boolean;
    dialogMaxWidth?: number;
    readonly?: boolean;
  }>(),
  {
    assetId: undefined,
    open: false,
    dialogMaxWidth: 800,
    readonly: false,
  },
);

const emit = defineEmits<{
  (event: 'update:open', payload: boolean): void;
}>();

const props = toRefs(input);
const valid = ref(true);
const loading = ref(false);
const saving = ref(false);
const asset = ref<Partial<Asset>>({});
const openModel = computed({
  get: () => props.open.value,
  set: value => emit('update:open', value),
});

const station = useStationStore();

const loadAsset = async (): Promise<{
  asset: Partial<Asset>;
}> => {
  if (props.assetId.value === undefined) {
    const createModel: Partial<Asset> = {};

    return { asset: createModel };
  }

  const result = await station.service.getAsset(
    {
      asset_id: props.assetId.value,
    },
    true,
  );

  return { asset: result.asset };
};

const canSave = computed(() => {
  return valid.value && !loading.value;
});

const triggerSubmit = ref(false);

const save = async (): Promise<void> => {
  if (!canSave.value) {
    return;
  }
  try {
    saving.value = true;
    if (asset.value.id) {
      const request = await station.service.editAsset({
        asset_id: asset.value.id,
        change_metadata: [
          {
            ReplaceAllBy: asset.value.metadata ?? [],
          },
        ],
        blockchain: [assertAndReturn(asset.value.blockchain, 'blockchain')],
        name: [assertAndReturn(asset.value.name, 'name')],
        symbol: [assertAndReturn(asset.value.symbol, 'symbol')],
        standards: [assertAndReturn(asset.value.standards, 'standards')],
      });
      useOnSuccessfulOperation(request);
      openModel.value = false;
      return;
    }

    const request = await station.service.addAsset({
      blockchain: assertAndReturn(asset.value.blockchain, 'blockchain'),
      metadata: asset.value.metadata ?? [],
      decimals: assertAndReturn(asset.value.decimals, 'decimals'),
      name: assertAndReturn(asset.value.name, 'name'),
      symbol: assertAndReturn(asset.value.symbol, 'symbol'),
      standards: assertAndReturn(asset.value.standards, 'standards'),
    });
    useOnSuccessfulOperation(request);
    openModel.value = false;
  } catch (error) {
    logger.error(`Failed to save asset ${error}`);
    useOnFailedOperation();
  } finally {
    saving.value = false;
  }
};
</script>
