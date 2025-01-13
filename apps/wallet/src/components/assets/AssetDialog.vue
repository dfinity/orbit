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
          <VRadioGroup v-if="!input.assetId" inline v-model="assetType">
            <VRadio
              :label="$t('pages.assets.well_known.option_add_well_known_assets')"
              value="well-known"
            ></VRadio>
            <VRadio
              :label="$t('pages.assets.well_known.option_add_custom_asset')"
              value="custom"
            ></VRadio>
          </VRadioGroup>

          <template v-if="!input.assetId && assetType === 'well-known'">
            <VCombobox
              v-model="chosenWellKnownAssets"
              :loading="wellKnownAssets === undefined"
              :items="wellKnownAssets"
              item-title="name"
              item-value="symbol"
              multiple
              clear-on-select
              hide-selected
              :return-object="true"
              :custom-filter="filterWellKnownAssets"
              :placeholder="$t(`pages.assets.well_known.placeholder`)"
              :rules="[requiredRule]"
            >
              <template #selection="{ item, index }">
                <VChip
                  :text="item.title"
                  v-if="item === Object(item)"
                  color="secondary"
                  size="small"
                  variant="flat"
                  closable
                  label
                  @click:close="removeWellKnownAsset(index)"
                ></VChip>
              </template>

              <template #item="{ item, props: { onClick } }">
                <VListSubheader v-if="'header' in item.raw">
                  {{ $t(`pages.assets.well_known.groups.${item.raw.header}`) }}
                </VListSubheader>
                <!-- prettier-ignore -->

                <VListItem
                  @click="(onClick as any)"
                  :disabled="isExistingAsset(item.raw.data.blockchain, item.raw.data.symbol)"
                  v-else
                >
                  <VListItemTitle class="d-flex justify-space-between align-center"
                    >{{ item.title }}

                    <VIcon
                      v-if="isExistingAsset(item.raw.data.blockchain, item.raw.data.symbol)"
                      size="16"
                      class="ml-2"
                      :icon="mdiCheckCircle"
                    >
                    </VIcon>
                  </VListItemTitle>
                  <VListItemSubtitle>{{ item.raw.symbol }}</VListItemSubtitle>
                </VListItem>
              </template>
            </VCombobox>
          </template>

          <template v-else-if="input.assetId || assetType === 'custom'">
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
          </template>
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
            @click="onSave"
          >
            {{ props.assetId.value ? $t('terms.save') : $t('terms.create') }}
          </VBtn>
        </VCardActions>
      </VCard>
    </DataLoader>
  </VDialog>
</template>
<script lang="ts" setup>
import { mdiCheckCircle, mdiClose } from '@mdi/js';
import { computed, ref, toRefs, watch } from 'vue';
import {
  VBtn,
  VCard,
  VCardActions,
  VCardText,
  VDialog,
  VDivider,
  VRadio,
  VRadioGroup,
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
import { assertAndReturn, unreachable } from '~/utils/helper.utils';
import AssetForm from './AssetForm.vue';
import { requiredRule } from '~/utils/form.utils';
import { fetchWellKnownIcpAssets, getAllAssets, WellKnownAsset } from '~/utils/asset.utils';

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

const assetType = ref<'well-known' | 'custom'>('well-known');

const chosenWellKnownAssets = ref<ComboboxItem[]>([]);

type GroupedComboboxItem =
  | {
      header: string;
    }
  | {
      name: string;
      symbol: string;
      data: WellKnownAsset;
    };

const wellKnownAssets = ref<GroupedComboboxItem[] | undefined>(undefined);

function removeWellKnownAsset(index: number) {
  chosenWellKnownAssets.value.splice(index, 1);
}

function filterWellKnownAssets(_: string, queryText: string, item: any) {
  const { raw } = item as { raw: GroupedComboboxItem };
  if ('header' in raw) {
    return false;
  }

  const lowercaseQuery = queryText.toLowerCase();
  return (
    raw.name.toLowerCase().includes(lowercaseQuery) ||
    raw.symbol.toLowerCase().includes(lowercaseQuery)
  );
}

const emit = defineEmits<{
  (event: 'update:open', payload: boolean): void;
}>();

const props = toRefs(input);
const valid = ref(true);
const loading = ref(false);
const saving = ref(false);
const asset = ref<Partial<Asset>>({});
const existingAssets = ref<Asset[] | undefined>();
const openModel = computed({
  get: () => props.open.value,
  set: value => emit('update:open', value),
});

const station = useStationStore();

watch(openModel, value => {
  if (value) {
    assetType.value = 'well-known';
    chosenWellKnownAssets.value = [];
    wellKnownAssets.value = undefined;
    if (!input.assetId) {
      loadWellKnownAssets();
      getAllAssets().then(assets => {
        existingAssets.value = assets;
      });
    }
  }
});

function isExistingAsset(blockchain: string, symbol: string) {
  return existingAssets.value?.some(a => a.blockchain === blockchain && a.symbol === symbol);
}

type ComboboxItem = {
  name: string;
  symbol: string;
  data: WellKnownAsset;
};

function loadWellKnownAssets() {
  fetchWellKnownIcpAssets().then(groupedAssets => {
    wellKnownAssets.value = groupedAssets
      .map(
        group =>
          [
            {
              header: group.groupKey,
            },
            ...group.assets.map(a => ({
              name: a.name,
              symbol: a.symbol,
              data: a,
            })),
          ] satisfies GroupedComboboxItem[],
      )
      .flat();
  });
}

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
  if (input.assetId) {
    return valid.value && !loading.value;
  } else if (assetType.value === 'well-known') {
    return chosenWellKnownAssets.value.length > 0 && !loading.value;
  } else if (assetType.value === 'custom') {
    return valid.value && !loading.value;
  } else {
    unreachable(assetType.value);
  }
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

async function onSave() {
  if (!input.assetId) {
    if (assetType.value === 'custom') {
      triggerSubmit.value = true;
    } else if (assetType.value === 'well-known') {
      try {
        saving.value = true;
        const results = await Promise.all(
          chosenWellKnownAssets.value.map(asset => station.service.addAsset(asset.data)),
        );
        useOnSuccessfulOperation(results[0]);
        openModel.value = false;
      } catch (error) {
        logger.error(`Failed to save assets ${error}`);
        useOnFailedOperation();
      } finally {
        saving.value = false;
      }

      // useOnSuccessfulOperation(request);
    } else {
      unreachable(assetType.value);
    }
  } else {
    triggerSubmit.value = true;
  }
}
</script>
