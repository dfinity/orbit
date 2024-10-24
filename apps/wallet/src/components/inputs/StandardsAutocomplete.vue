<template>
  <VSelect
    v-model="model"
    :multiple="props.multiple"
    :label="props.label"
    item-value="value"
    item-title="text"
    :items="items"
    :variant="props.variant"
    :density="props.density"
    :readonly="props.readonly"
    :disabled="props.disabled"
    :rules="props.rules"
  />
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import { VSelect } from 'vuetify/components';
import { useStationStore } from '~/stores/station.store';
import { FormValidationRuleFn } from '~/types/helper.types';

const props = withDefaults(
  defineProps<{
    modelValue?: string[];
    blockchain: string;
    label?: string;
    variant?: 'underlined' | 'outlined' | 'filled';
    density?: 'comfortable' | 'compact';
    multiple?: boolean;
    readonly?: boolean;
    disabled?: boolean;
    rules?: FormValidationRuleFn[];
  }>(),
  {
    modelValue: undefined,
    label: undefined,
    variant: 'filled',
    density: 'comfortable',
    multiple: false,
    readonly: false,
    disabled: false,
    rules: undefined,
  },
);

// const props = toRefs(input);
const i18n = useI18n();
const station = useStationStore();
const standardsData = computed(
  () =>
    station.configuration.details.supported_blockchains.find(b => b.blockchain === props.blockchain)
      ?.supported_standards || [],
);

const model = computed({
  get: () => props.modelValue,
  set: value => {
    emit('update:modelValue', value || []);
  },
});

const emit = defineEmits<{
  (event: 'update:modelValue', payload?: string[]): void;
}>();

const items = computed(() => {
  return standardsData.value.map(data => ({
    value: data.standard,
    text: i18n.t(`blockchains.${props.blockchain}.standards.${data.standard}`),
  }));
});
</script>
