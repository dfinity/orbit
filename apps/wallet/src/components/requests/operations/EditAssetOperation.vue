<template>
  <div v-if="isListMode" class="d-flex flex-column ga-0 text-caption">
    <RequestOperationListRow v-if="symbolName">
      <template #name>{{ $t('terms.symbol') }}</template>
      <template #content>
        {{ symbolName }}
      </template>
    </RequestOperationListRow>
    <RequestOperationListRow v-if="blockchainStandards">
      <template #name>{{ $t('terms.standards') }}</template>
      <template #content> {{ blockchainStandards }} </template>
    </RequestOperationListRow>
  </div>
  <VProgressCircular v-else-if="loading" indeterminate />
  <AssetForm v-else :model-value="formValue" mode="view" />
</template>

<script setup lang="ts">
import { Ref, computed, onBeforeMount, ref } from 'vue';
import { Asset, EditAssetOperation, Request } from '~/generated/station/station.did';
import RequestOperationListRow from '../RequestOperationListRow.vue';
import AssetForm from '~/components/assets/AssetForm.vue';
import { useStationStore } from '~/stores/station.store';
import { unreachable, variantIs } from '~/utils/helper.utils';
import { VProgressCircular } from 'vuetify/components';
import { useI18n } from 'vue-i18n';
const i18n = useI18n();

const props = withDefaults(
  defineProps<{
    request: Request;
    operation: EditAssetOperation;
    mode?: 'list' | 'detail';
  }>(),
  {
    mode: 'list',
  },
);

const isListMode = computed(() => props.mode === 'list');
const formValue: Ref<Partial<Asset>> = ref({});
const station = useStationStore();
const loading = ref(false);

const symbolName = ref('');
const blockchainStandards = ref('');

const fetchDetails = async () => {
  loading.value = true;

  const entry: Partial<Asset> = await station.service
    .getAsset(
      {
        asset_id: props.operation.input.asset_id,
      },
      true,
    )
    .then(({ asset }) => asset as Partial<Asset>)
    .catch(
      () =>
        ({
          id: props.operation.input.asset_id,
          metadata: [],
        }) satisfies Partial<Asset>,
    );

  if (props.operation.input.blockchain && props.operation.input.blockchain.length > 0) {
    entry.blockchain = props.operation.input.blockchain[0];
  }

  if (props.operation.input.change_metadata?.[0]) {
    const changeMetadata = props.operation.input.change_metadata[0];
    if (variantIs(changeMetadata, 'ReplaceAllBy')) {
      entry.metadata = changeMetadata.ReplaceAllBy;
    } else if (variantIs(changeMetadata, 'OverrideSpecifiedBy')) {
      changeMetadata.OverrideSpecifiedBy.forEach(metadata => {
        const existingValue = entry.metadata!.find(m => m.key === metadata.key);
        if (existingValue) {
          existingValue.value = metadata.value;
        }
      });
    } else if (variantIs(changeMetadata, 'RemoveKeys')) {
      changeMetadata.RemoveKeys.forEach(metadata => {
        const existingValueIndex = entry.metadata!.findIndex(m => m.key === metadata);
        if (existingValueIndex !== -1) {
          entry.metadata!.splice(existingValueIndex, 1);
        }
      });
    } else {
      return unreachable(changeMetadata);
    }
  }

  if (props.operation.input.symbol && props.operation.input.symbol.length > 0) {
    entry.symbol = props.operation.input.symbol[0];
  }

  if (props.operation.input.standards && props.operation.input.standards.length > 0) {
    entry.standards = props.operation.input.standards[0];
  }

  if (props.operation.input.name && props.operation.input.name.length > 0) {
    entry.name = props.operation.input.name[0];
  }

  formValue.value = entry;
  loading.value = false;
};

onBeforeMount(() => {
  const symbol = props.operation.input.symbol?.length > 0 ? props.operation.input.symbol[0] : '';
  const name = props.operation.input.name?.length > 0 ? props.operation.input.name[0] : '';
  const blockchain =
    props.operation.input.blockchain?.length > 0 ? props.operation.input.blockchain[0] : '';
  const standards: string[] =
    props.operation.input.standards?.length > 0 ? props.operation.input.standards[0]! : [];

  if (symbol && name) {
    symbolName.value = `${symbol} (${name})`;
  } else if (symbol) {
    symbolName.value = symbol;
  } else if (name) {
    symbolName.value = name;
  }

  if (blockchain && standards.length > 0) {
    blockchainStandards.value = standards
      .map(standard => i18n.t(`blockchains.${blockchain}.standards.${standard}`))
      .join(', ');
  } else if (blockchain) {
    blockchainStandards.value = i18n.t(`blockchains.${blockchain}.name`);
  } else if (standards) {
    blockchainStandards.value = standards.join(', ');
  }

  if (!isListMode.value) {
    fetchDetails();
  }
});
</script>
