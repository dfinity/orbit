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
      <!-- @selected-asset="onSelectedAsset" -->
    </VCol>
    <VCol cols="12" class="pt-0 pb-4">
      <VTextField
        v-model="model.name"
        name="name"
        :label="$t('terms.name')"
        :rules="[requiredRule]"
        variant="filled"
        class="mb-2"
        density="comfortable"
        :prepend-icon="mdiWallet"
        :disabled="isViewMode"
      />
    </VCol>
  </VRow>
</template>

<script lang="ts" setup>
import { mdiBank, mdiIdentifier, mdiWallet } from '@mdi/js';
import { computed } from 'vue';
import { VCol, VRow, VTextField } from 'vuetify/components';
import TokenAutocomplete from '~/components/inputs/TokenAutocomplete.vue';
import { TimestampRFC3339, UUID } from '~/generated/station/station.did';
import { requiredRule } from '~/utils/form.utils';

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
  }>(),
  {
    display: () => ({
      id: true,
      asset: true,
    }),
    mode: 'edit',
    triggerSubmit: false,
  },
);

const emit = defineEmits<{
  (event: 'update:modelValue', payload: Partial<AccountConfigurationModel>): void;
}>();

const isViewMode = computed(() => props.mode === 'view');

const model = computed({
  get: () => props.modelValue,
  set: value => {
    console.log(value);

    emit('update:modelValue', value);
  },
});

const assetIds = computed({
  get: () => props.modelValue.assets,
  set: value => {
    props.modelValue.assets = value;
    console.log(value);

    // emit('update:modelValue', props.modelValue);
  },
});

// const onSelectedAsset = (asset?: Asset): void => {
//   model.value.assets = asset ? [asset] : [];
// };
</script>
