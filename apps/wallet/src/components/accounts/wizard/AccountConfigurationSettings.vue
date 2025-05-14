<template>
  <VRow>
    <VCol v-if="model.id && props.display.id" cols="12" class="pt-0 pb-0">
      <VTextField
        v-model="model.id"
        name="id"
        :prepend-icon="mdiIdentifier"
        :label="$t('terms.id')"
        variant="plain"
        density="comfortable"
        hide-details
        readonly
      />
    </VCol>
    <VCol cols="12" class="pt-4 pb-0">
      <DiffView :before-value="currentAssetIds" :after-value="assetIds">
        <template #default="{ value, diffMode }">
          <TokenAutocomplete
            v-if="props.display.asset"
            :model-value="value"
            class="mb-2"
            :label="$t('terms.asset')"
            :prepend-icon="mdiBank"
            :rules="diffMode === 'before' ? [] : [requiredRule]"
            :variant="isViewMode ? 'plain' : 'filled'"
            density="comfortable"
            :readonly="isViewMode || !!model.id"
            :multiple="true"
            @update:model-value="val => diffMode === 'after' && (assetIds = val as UUID[])"
          />
        </template>
      </DiffView>
    </VCol>
    <VCol cols="12" class="pt-0 pb-4">
      <DiffView :before-value="props.currentConfiguration?.name" :after-value="model.name">
        <template #default="{ value, diffMode }">
          <VTextField
            :name="diffMode === 'before' ? 'name-before' : 'name'"
            :model-value="value"
            :label="$t('terms.name')"
            density="comfortable"
            :prepend-icon="mdiWallet"
            :rules="
              diffMode === 'before' ? [] : [requiredRule, maxLengthRule(64, $t('terms.name'))]
            "
            :variant="isViewMode ? 'plain' : 'filled'"
            :readonly="isViewMode || diffMode === 'before'"
            class="mb-2"
            @update:model-value="val => diffMode === 'after' && (model.name = val)"
          />
        </template>
      </DiffView>
    </VCol>
  </VRow>
</template>

<script lang="ts" setup>
import { mdiBank, mdiIdentifier, mdiWallet } from '@mdi/js';
import { computed } from 'vue';
import { VCol, VRow, VTextField } from 'vuetify/components';
import TokenAutocomplete from '~/components/inputs/TokenAutocomplete.vue';
import DiffView from '~/components/requests/DiffView.vue';
import { TimestampRFC3339, UUID } from '~/generated/station/station.did';
import { maxLengthRule, requiredRule } from '~/utils/form.utils';

export interface AccountConfigurationModel {
  id: UUID;
  name: string;
  assets: UUID[];
  lastModified: TimestampRFC3339;
}

const props = withDefaults(
  defineProps<{
    modelValue: Partial<AccountConfigurationModel>;
    mode: 'view' | 'edit';
    display?: {
      id?: boolean;
      asset?: boolean;
    };
    currentConfiguration?: AccountConfigurationModel;
  }>(),
  {
    display: () => ({
      id: true,
      asset: true,
    }),
    mode: 'edit',
    currentConfiguration: undefined,
  },
);

const emit = defineEmits<{
  (event: 'update:modelValue', payload: Partial<AccountConfigurationModel>): void;
}>();

const isViewMode = computed(() => props.mode === 'view');

const model = computed({
  get: () => props.modelValue,
  set: value => emit('update:modelValue', value),
});

const assetIds = computed({
  get: () => model.value.assets,
  set: value => (model.value.assets = value),
});

const currentAssetIds = computed(() => props.currentConfiguration?.assets);
</script>
