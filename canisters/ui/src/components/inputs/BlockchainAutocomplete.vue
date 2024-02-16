<template>
  <VAutocomplete
    v-model="model"
    :multiple="props.multiple.value"
    :label="props.label.value"
    item-value="value"
    item-title="text"
    :items="items"
    :variant="props.variant.value"
    :density="props.density.value"
    :readonly="props.readonly.value"
    :disabled="props.disabled.value"
    :rules="props.rules.value"
  />
</template>

<script setup lang="ts">
import { computed, toRefs } from 'vue';
import { useI18n } from 'vue-i18n';
import { useWalletStore } from '~/stores/wallet.store';
import { FormValidationRuleFn } from '~/types/helper.types';

const input = withDefaults(
  defineProps<{
    modelValue?: string;
    label?: string;
    variant?: 'underlined' | 'outlined';
    density?: 'comfortable' | 'compact';
    multiple?: boolean;
    readonly?: boolean;
    disabled?: boolean;
    rules?: FormValidationRuleFn[];
  }>(),
  {
    modelValue: undefined,
    label: undefined,
    variant: 'underlined',
    density: 'comfortable',
    multiple: false,
    readonly: false,
    disabled: false,
    rules: undefined,
  },
);

const props = toRefs(input);
const i18n = useI18n();
const wallet = useWalletStore();
const blockchains = computed(() =>
  Array.from(new Set(wallet.configuration.details.supported_assets.map(token => token.blockchain))),
);

const model = computed({
  get: () => props.modelValue.value,
  set: value => emit('update:modelValue', value),
});

const emit = defineEmits<{
  (event: 'update:modelValue', payload?: string): void;
}>();

const items = computed(() =>
  blockchains.value.map(chain => ({
    value: chain,
    text: i18n.t(`blockchains.${chain}.name`),
  })),
);
</script>
