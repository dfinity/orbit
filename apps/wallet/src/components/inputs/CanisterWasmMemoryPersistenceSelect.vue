<template>
  <VSelect
    v-model="selected"
    :label="label"
    :variant="props.variant"
    :density="props.density"
    :readonly="props.readonly"
    :items="items"
    :hint="props.hint"
    :persistent-hint="props.hint !== undefined"
    :prepend-icon="mdiMemory"
  />
</template>
<script setup lang="ts">
import { mdiMemory } from '@mdi/js';
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import { VSelect } from 'vuetify/components';
import { WasmMemoryPersistence } from '~/components/external-canisters/external-canisters.types';

type WasmMemoryPersistenceOption = 'default' | 'keep' | 'replace';

const props = withDefaults(
  defineProps<{
    modelValue?: WasmMemoryPersistence;
    readonly?: boolean;
    label?: string;
    hint?: string;
    density?: 'comfortable' | 'compact' | 'default';
    variant?: 'filled' | 'outlined' | 'plain' | 'solo' | 'underlined';
  }>(),
  {
    modelValue: undefined,
    readonly: false,
    label: undefined,
    hint: undefined,
    density: 'comfortable',
    variant: 'filled',
  },
);

const emit = defineEmits<{
  (event: 'update:modelValue', payload?: WasmMemoryPersistence): void;
}>();

const i18n = useI18n();
const label = computed(
  () => props.label ?? i18n.t('external_canisters.wasm_memory_persistence.label'),
);

const selected = computed<WasmMemoryPersistenceOption>({
  get: () => {
    if (!props.modelValue) {
      return 'default';
    }

    return 'keep' in props.modelValue ? 'keep' : 'replace';
  },
  set: value => {
    switch (value) {
      case 'keep':
        emit('update:modelValue', { keep: null });
        break;
      case 'replace':
        emit('update:modelValue', { replace: null });
        break;
      default:
        emit('update:modelValue', undefined);
    }
  },
});

const items = computed<{ title: string; value: WasmMemoryPersistenceOption }[]>(() => [
  { title: i18n.t('external_canisters.wasm_memory_persistence.default'), value: 'default' },
  { title: i18n.t('external_canisters.wasm_memory_persistence.keep'), value: 'keep' },
  { title: i18n.t('external_canisters.wasm_memory_persistence.replace'), value: 'replace' },
]);
</script>
