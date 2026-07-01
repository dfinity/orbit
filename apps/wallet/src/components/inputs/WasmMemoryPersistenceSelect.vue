<template>
  <VSelect
    v-model="model"
    :label="label"
    :variant="props.variant"
    :density="props.density"
    :readonly="props.readonly"
    :items="items"
    :prepend-icon="mdiMemory"
    :hint="hint"
    persistent-hint
    clearable
  />
</template>
<script setup lang="ts">
import { mdiMemory } from '@mdi/js';
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import { VSelect } from 'vuetify/components';
import { WasmMemoryPersistence } from '~/types/station.types';

const props = withDefaults(
  defineProps<{
    modelValue?: WasmMemoryPersistence;
    readonly?: boolean;
    label?: string;
    density?: 'comfortable' | 'compact' | 'default';
    variant?: 'filled' | 'outlined' | 'plain' | 'solo' | 'underlined';
  }>(),
  {
    modelValue: undefined,
    readonly: false,
    label: undefined,
    density: 'comfortable',
    variant: 'filled',
  },
);

const emit = defineEmits<{
  (event: 'update:modelValue', payload?: WasmMemoryPersistence): void;
}>();

const model = computed({
  get: () => props.modelValue,
  // Vuetify's `clearable` emits `null`; normalize it back to `undefined`.
  set: value => emit('update:modelValue', value ?? undefined),
});

const i18n = useI18n();
const label = computed(
  () => props.label ?? i18n.t('external_canisters.wasm_memory_persistence.label'),
);
const hint = computed(() => i18n.t('external_canisters.wasm_memory_persistence.hint'));

const items = computed<
  {
    title: string;
    value: WasmMemoryPersistence;
  }[]
>(() => [
  { title: i18n.t('external_canisters.wasm_memory_persistence.keep'), value: { keep: null } },
  { title: i18n.t('external_canisters.wasm_memory_persistence.replace'), value: { replace: null } },
]);
</script>
