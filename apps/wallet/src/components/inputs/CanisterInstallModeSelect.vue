<template>
  <VSelect
    v-model="model"
    :label="label"
    :variant="props.variant"
    :density="props.density"
    :readonly="props.readonly"
    :items="modes"
    auto-select-first
    :rules="props.required ? [requiredRule] : undefined"
    :prepend-icon="mdiCog"
  />
</template>
<script setup lang="ts">
import { mdiCog } from '@mdi/js';
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import { VSelect } from 'vuetify/components';
import { CanisterInstallMode } from '~/generated/station/station.did';
import { requiredRule } from '~/utils/form.utils';

const props = withDefaults(
  defineProps<{
    modelValue?: CanisterInstallMode;
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
  (event: 'update:modelValue', payload?: CanisterInstallMode): void;
}>();

const model = computed({
  get: () => props.modelValue,
  set: value => emit('update:modelValue', value),
});

const i18n = useI18n();
const label = computed(() => props.label ?? i18n.t('terms.mode'));

const modes = computed<
  {
    title: string;
    value: CanisterInstallMode;
  }[]
>(() => [
  { title: i18n.t('external_canisters.install_mode.install'), value: { install: null } },
  { title: i18n.t('external_canisters.install_mode.reinstall'), value: { reinstall: null } },
  { title: i18n.t('external_canisters.install_mode.upgrade'), value: { upgrade: null } },
]);
</script>
