<template>
  <VForm ref="form" @submit.prevent="submit">
    <VTextField
      v-if="model.id && props.display.value.id"
      v-model="model.id"
      name="id"
      :label="$t('terms.id')"
      variant="plain"
      density="compact"
      :disabled="isViewMode"
    />
    <BlockchainAutocomplete
      v-if="!isViewMode || model.blockchain"
      v-model="model.blockchain"
      class="mb-2"
      :label="$t('terms.blockchain')"
      :prepend-icon="mdiTransitConnectionVariant"
      :rules="[requiredRule]"
      variant="filled"
      density="comfortable"
      :disabled="isViewMode || !!model.id"
    />

    <StandardsAutocomplete
      v-if="model.blockchain"
      v-model="model.standards"
      class="mb-2"
      :blockchain="model.blockchain"
      :label="$t('terms.standards')"
      :prepend-icon="mdiKeyChain"
      :rules="[requiredRule]"
      variant="filled"
      density="comfortable"
      :disabled="isViewMode || !!model.id"
      :multiple="true"
    />

    <InternetComputerNativeStandardForm
      v-if="
        model.blockchain === 'icp' &&
        ((model.standards && model.standards.includes(BlockchainStandard.Native)) ||
          (model.standards && model.standards.includes(BlockchainStandard.ICRC1)))
      "
      v-model="model.metadata!"
      :readonly="isViewMode"
    ></InternetComputerNativeStandardForm>

    <template v-if="model.blockchain && model.standards && model.standards.length > 0">
      <VTextField
        v-model="model.name"
        name="name"
        :label="$t('terms.name')"
        variant="filled"
        density="comfortable"
        :disabled="isViewMode"
        :prepend-icon="mdiTextBox"
        :rules="[requiredRule]"
      />
      <VTextField
        v-model="model.symbol"
        name="symbol"
        :label="$t('terms.symbol')"
        variant="filled"
        density="comfortable"
        :disabled="isViewMode"
        :prepend-icon="mdiTag"
        :rules="[requiredRule]"
      />
      <VTextField
        v-model="decimals"
        name="decimals"
        type="number"
        :label="$t('pages.assets.forms.decimals')"
        variant="filled"
        density="comfortable"
        :disabled="isViewMode"
        :prepend-icon="mdiDecimal"
        :rules="[requiredRule]"
      />

      <MetadataField
        v-model="model.metadata"
        :label="$t('terms.metadata')"
        :rules="[requiredRule]"
        :disabled="isViewMode"
      />
    </template>
  </VForm>
</template>

<script lang="ts" setup>
import { mdiDecimal, mdiKeyChain, mdiTag, mdiTextBox, mdiTransitConnectionVariant } from '@mdi/js';
import { computed, onMounted, ref, toRefs, watch } from 'vue';
import { VForm, VTextField } from 'vuetify/components';
import BlockchainAutocomplete from '~/components/inputs/BlockchainAutocomplete.vue';
import MetadataField from '~/components/inputs/MetadataField.vue';
import { Asset } from '~/generated/station/station.did';
import { VFormValidation } from '~/types/helper.types';
import { requiredRule } from '~/utils/form.utils';
import StandardsAutocomplete from '../inputs/StandardsAutocomplete.vue';
import InternetComputerNativeStandardForm from './standards/InternetComputerNativeStandardForm.vue';
import { BlockchainStandard } from '~/types/chain.types';

export type AssetFormProps = {
  modelValue: Partial<Asset>;
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
});
const props = toRefs(input);

const isViewMode = computed(() => props.mode.value === 'view');

const emit = defineEmits<{
  (event: 'update:modelValue', payload: AssetFormProps['modelValue']): void;
  (event: 'update:triggerSubmit', payload: boolean): void;
  (event: 'valid', payload: boolean): void;
  (event: 'submit', payload: AssetFormProps['modelValue']): void;
}>();

const model = computed(() => props.modelValue.value);

watch(model.value, newValue => emit('update:modelValue', newValue), { deep: true });

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
</script>
