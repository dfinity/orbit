<template>
  <VFileInput
    v-model="wasmModuleFile"
    :label="label"
    :rules="props.required ? [requiredRule] : undefined"
    :prepend-icon="mdiDatabase"
    :variant="props.variant"
    :hint="wasmModuleChecksumHint"
    persistent-hint
    :density="props.density"
  />
</template>
<script setup lang="ts">
import { mdiDatabase } from '@mdi/js';
import { computed, ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import { VFileInput } from 'vuetify/components';
import logger from '~/core/logger.core';
import { arrayBufferToHashHex } from '~/utils/crypto.utils';
import { readFileAsArrayBuffer } from '~/utils/file.utils';
import { requiredRule } from '~/utils/form.utils';

const props = withDefaults(
  defineProps<{
    modelValue?: Uint8Array;
    readonly?: boolean;
    required?: boolean;
    label?: string;
    density?: 'comfortable' | 'compact' | 'default';
    variant?: 'filled' | 'outlined' | 'plain' | 'solo' | 'underlined';
  }>(),
  {
    modelValue: undefined,
    readonly: false,
    required: false,
    label: undefined,
    density: 'comfortable',
    variant: 'filled',
  },
);

const emit = defineEmits<{
  (event: 'update:modelValue', payload?: Uint8Array): void;
}>();

const model = computed({
  get: () => props.modelValue,
  set: value => emit('update:modelValue', value),
});

const i18n = useI18n();
const label = computed(() => props.label ?? i18n.t('external_canisters.wasm_module'));
const wasmModuleFile = ref<File[]>([]);
const wasmModuleChecksum = ref<string>();
const wasmModuleChecksumHint = computed(() => {
  if (!wasmModuleChecksum.value) {
    return undefined;
  }

  return `${i18n.t('terms.checksum')}: ${wasmModuleChecksum.value}`;
});

const updateComputedCanisterModule = async () => {
  try {
    if (!wasmModuleFile.value || wasmModuleFile.value.length === 0) {
      model.value = undefined;
      wasmModuleChecksum.value = undefined;

      return;
    }

    const module = await readFileAsArrayBuffer(wasmModuleFile.value[0]);

    wasmModuleChecksum.value = await arrayBufferToHashHex(module);
    model.value = new Uint8Array(module);
  } catch (error) {
    model.value = undefined;
    wasmModuleChecksum.value = undefined;
    logger.error('Failed to read wasm module file', error);
  }
};

watch(wasmModuleFile, () => updateComputedCanisterModule(), { deep: true });
</script>
