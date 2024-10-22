<template>
  <div v-if="isListMode" class="d-flex flex-column ga-0 text-caption">
    <RequestOperationListRow>
      <template #name>{{ $t('terms.symbol') }}</template>
      <template #content> {{ formValue.symbol }} ({{ formValue.name }}) </template>
    </RequestOperationListRow>
    <RequestOperationListRow>
      <template #name>{{ $t('terms.standards') }}</template>
      <template #content>
        {{
          formValue.standards
            ?.map(standard => $t(`blockchains.${formValue.blockchain!}.standards.${standard}`))
            .join(', ')
        }}
      </template>
    </RequestOperationListRow>
  </div>
  <AssetForm v-else :model-value="formValue" mode="view" />
</template>

<script setup lang="ts">
import { Ref, computed, onBeforeMount, ref } from 'vue';
import { AddAssetOperation, Asset, Request } from '~/generated/station/station.did';
import RequestOperationListRow from '../RequestOperationListRow.vue';
import AssetForm from '~/components/assets/AssetForm.vue';

const props = withDefaults(
  defineProps<{
    request: Request;
    operation: AddAssetOperation;
    mode?: 'list' | 'detail';
  }>(),
  {
    mode: 'list',
  },
);

const isListMode = computed(() => props.mode === 'list');
const formValue: Ref<Partial<Asset>> = ref({});

onBeforeMount(() => {
  const entry: Partial<Asset> = {};
  entry.blockchain = props.operation.input.blockchain;
  entry.metadata = props.operation.input.metadata;
  entry.symbol = props.operation.input.symbol;
  entry.decimals = props.operation.input.decimals;
  entry.standards = props.operation.input.standards;
  entry.name = props.operation.input.name;

  formValue.value = entry;
});
</script>
