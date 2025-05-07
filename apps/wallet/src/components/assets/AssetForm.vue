<template>
  <VForm ref="form" @submit.prevent="submit">
    <VTextField
      v-if="model.id && displayId"
      v-model="model.id"
      name="id"
      :label="$t('terms.id')"
      variant="plain"
      density="compact"
      readonly
    />

    <DiffView :before-value="currentAsset?.blockchain" :after-value="model.blockchain">
      <template #default="{ value, diffMode }">
        <BlockchainAutocomplete
          v-if="!isViewMode || value"
          :model-value="value"
          class="mb-2"
          :name="diffMode === 'before' ? 'blockchain-before' : 'blockchain'"
          :label="$t('terms.blockchain')"
          :prepend-icon="mdiTransitConnectionVariant"
          :rules="diffMode === 'before' ? [] : [requiredRule]"
          :variant="isViewMode || !!model.id ? 'plain' : 'filled'"
          density="comfortable"
          :readonly="isViewMode || diffMode === 'before' || !!model.id"
          @update:model-value="val => diffMode === 'after' && (model.blockchain = val)"
        />
      </template>
    </DiffView>

    <DiffView :before-value="currentAsset?.standards" :after-value="model.standards">
      <template #default="{ value, diffMode }">
        <StandardsAutocomplete
          v-if="model.blockchain"
          :model-value="value"
          class="mb-2"
          :name="diffMode === 'before' ? 'standards-before' : 'standards'"
          :blockchain="model.blockchain"
          :label="$t('terms.standards')"
          :prepend-icon="mdiKeyChain"
          :rules="diffMode === 'before' ? [] : [requiredRule]"
          :variant="isViewMode ? 'plain' : 'filled'"
          density="comfortable"
          :readonly="isViewMode || diffMode === 'before' || !!model.id"
          :multiple="true"
          @update:model-value="val => diffMode === 'after' && (model.standards = val)"
        />
      </template>
    </DiffView>
    <InternetComputerNativeStandardForm
      v-if="shouldUseIcpForm"
      v-model="model.metadata!"
      :readonly="isViewMode"
      :current-metadata="currentAsset?.metadata"
    ></InternetComputerNativeStandardForm>
    <template v-if="model.blockchain && model.standards && model.standards.length > 0">
      <DiffView :before-value="currentAsset?.name" :after-value="model.name">
        <template #default="{ value, diffMode }">
          <VTextField
            :model-value="value"
            :name="diffMode === 'before' ? 'name-before' : 'name'"
            :label="$t('terms.name')"
            :variant="isViewMode ? 'plain' : 'filled'"
            density="comfortable"
            :readonly="isViewMode || diffMode === 'before'"
            :prepend-icon="mdiTextBox"
            :rules="
              diffMode === 'before' ? [] : [requiredRule, maxLengthRule(64, $t('terms.name'))]
            "
            @update:model-value="val => diffMode === 'after' && (model.name = val)"
          />
        </template>
      </DiffView>

      <DiffView :before-value="currentAsset?.symbol" :after-value="model.symbol">
        <template #default="{ value, diffMode }">
          <VTextField
            :model-value="value"
            :name="diffMode === 'before' ? 'symbol-before' : 'symbol'"
            :label="$t('terms.symbol')"
            :variant="isViewMode ? 'plain' : 'filled'"
            density="comfortable"
            :readonly="isViewMode || diffMode === 'before'"
            :prepend-icon="mdiTag"
            :rules="diffMode === 'before' ? [] : [requiredRule, validSymbolRule]"
            @update:model-value="val => diffMode === 'after' && (model.symbol = val)"
          />
        </template>
      </DiffView>

      <DiffView :before-value="currentAssetDecimals" :after-value="decimals">
        <template #default="{ value, diffMode }">
          <VTextField
            :model-value="value"
            :name="diffMode === 'before' ? 'decimals-before' : 'decimals'"
            type="number"
            :label="$t('pages.assets.forms.decimals')"
            :variant="isViewMode ? 'plain' : 'filled'"
            density="comfortable"
            :readonly="isViewMode || diffMode === 'before' || !!model.id"
            :prepend-icon="mdiDecimal"
            :rules="
              diffMode === 'before' ? [] : [requiredRule, numberRangeRule({ min: 0, max: 18 })]
            "
            @update:model-value="val => diffMode === 'after' && (decimals = val)"
          />
        </template>
      </DiffView>

      <DiffView
        :before-value="currentAsset?.metadata"
        :after-value="model.metadata"
        :compare-values="compareAssetMetadata"
      >
        <template #default="{ value, diffMode }">
          <MetadataField
            :model-value="value"
            :label="$t('terms.metadata')"
            :rules="diffMode === 'before' ? [] : [requiredRule]"
            :readonly="isViewMode || diffMode === 'before'"
            :hide-keys="hiddenMetadataKeys"
            @update:model-value="val => diffMode === 'after' && (model.metadata = val)"
          />
        </template>
      </DiffView>
    </template>
  </VForm>
</template>

<script lang="ts" setup>
import { mdiDecimal, mdiKeyChain, mdiTag, mdiTextBox, mdiTransitConnectionVariant } from '@mdi/js';
import { computed, onMounted, ref, toRefs, watch } from 'vue';
import { VForm, VTextField } from 'vuetify/components';
import BlockchainAutocomplete from '~/components/inputs/BlockchainAutocomplete.vue';
import MetadataField from '~/components/inputs/MetadataField.vue';
import { Asset, AssetMetadata } from '~/generated/station/station.did';
import { VFormValidation } from '~/types/helper.types';
import {
  compareMetadata,
  maxLengthRule,
  numberRangeRule,
  requiredRule,
  validSymbolRule,
} from '~/utils/form.utils';
import StandardsAutocomplete from '../inputs/StandardsAutocomplete.vue';
import InternetComputerNativeStandardForm from './standards/InternetComputerNativeStandardForm.vue';
import { BlockchainStandard } from '~/types/chain.types';
import DiffView from '~/components/requests/DiffView.vue';

export type AssetFormProps = {
  modelValue: Partial<Asset>;
  currentAsset?: Asset;
  triggerSubmit?: boolean;
  valid?: boolean;
  mode?: 'view' | 'edit';
  display?: {
    id?: boolean;
  };
};

const form = ref<VFormValidation | null>(null);

const input = withDefaults(defineProps<AssetFormProps>(), {
  valid: true,
  display: () => ({
    id: true,
  }),
  mode: 'edit',
  triggerSubmit: false,
  currentAsset: undefined,
});
const props = toRefs(input);

const isViewMode = computed(() => props.mode.value === 'view');
const displayId = computed(() => props.display.value.id);

const emit = defineEmits<{
  (event: 'update:modelValue', payload: AssetFormProps['modelValue']): void;
  (event: 'update:triggerSubmit', payload: boolean): void;
  (event: 'valid', payload: boolean): void;
  (event: 'submit', payload: AssetFormProps['modelValue']): void;
}>();

const model = computed(() => props.modelValue.value);

watch(model.value, newValue => emit('update:modelValue', newValue), { deep: true });

const currentAssetDecimals = computed(() =>
  props.currentAsset.value?.decimals !== undefined
    ? props.currentAsset.value.decimals.toString()
    : undefined,
);

const decimals = computed({
  get: () => (model.value.decimals === undefined ? '' : model.value.decimals.toString()),
  set: value => {
    model.value.decimals = value !== '' ? Number.parseInt(value) : undefined;
  },
});

onMounted(() => {
  if (!model.value.metadata) {
    model.value.metadata = [];
  }
});

const isFormValid = computed(() => (form.value ? form.value.isValid : false));
watch(
  () => isFormValid.value,
  isValid => emit('valid', isValid ?? false),
);

watch(
  () => props.triggerSubmit.value,
  () => {
    if (props.triggerSubmit.value) {
      emit('update:triggerSubmit', false);
      submit();
    }
  },
);

const submit = async () => {
  const { valid } = form.value ? await form.value.validate() : { valid: false };

  if (valid) {
    emit('submit', model.value);
  }
};

const shouldUseIcpForm = computed(
  () =>
    model.value.blockchain === 'icp' &&
    ((model.value.standards && model.value.standards.includes(BlockchainStandard.Native)) ||
      (model.value.standards && model.value.standards.includes(BlockchainStandard.ICRC1))),
);

const icpKeys = ['ledger_canister_id', 'index_canister_id'];

const hiddenMetadataKeys = computed(() => (shouldUseIcpForm.value ? icpKeys : []));

function compareAssetMetadata(
  before: AssetMetadata[] | undefined,
  after: AssetMetadata[] | undefined,
) {
  if (!before && !after) {
    return true;
  }

  if (!before || !after) {
    return false;
  }

  before = before.filter(item => !icpKeys.includes(item.key));
  after = after.filter(item => !icpKeys.includes(item.key));

  return compareMetadata(before, after);
}
</script>
