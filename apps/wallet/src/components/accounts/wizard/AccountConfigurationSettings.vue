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
        disabled
      />
    </VCol>
    <VCol cols="12" class="pt-4 pb-0">
      <TokenAutocomplete
        v-if="props.display.asset"
        v-model="assetIds"
        class="mb-2"
        :label="$t('terms.asset')"
        :prepend-icon="mdiBank"
        :rules="[requiredRule]"
        variant="filled"
        density="comfortable"
        :disabled="isViewMode || !!model.id"
        :multiple="true"
      />
    </VCol>
    <VCol cols="12" class="pt-0 pb-4">
      <DiffView :before-value="props.currentConfiguration?.name" :after-value="model.name">
        <template #default="{ value, mode }">
          <VTextField
            :name="mode === 'before' ? 'name' : 'name-after'"
            :model-value="value"
            @update:model-value="val => mode === 'after' && (model.name = val)"
            :label="$t('terms.name')"
            density="comfortable"
            :prepend-icon="mdiWallet"
            :rules="mode === 'before' ? [] : [requiredRule, maxLengthRule(64, $t('terms.name'))]"
            :variant="isViewMode ? 'plain' : 'filled'"
            :disabled="isViewMode || mode === 'before'"
            class="mb-2"
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
</script>
