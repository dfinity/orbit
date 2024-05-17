<template>
  <VForm ref="form" @submit.prevent="submit">
    <slot name="prepend"></slot>

    <VTextField
      v-model="modelValue.name"
      name="name"
      :label="$t('terms.station_name')"
      variant="filled"
      density="comfortable"
      :rules="rules.name"
    />
    <VSwitch
      v-if="canSetMain"
      v-model="modelValue.main"
      :label="$t('terms.main')"
      name="main"
      inset
      color="success"
      hide-details
    />
  </VForm>
</template>

<script lang="ts" setup>
import { computed, ref, watch } from 'vue';
import { VForm, VSwitch, VTextField } from 'vuetify/components';
import { i18n } from '~/plugins/i18n.plugin';
import { useSessionStore } from '~/stores/session.store';
import { useStationStore } from '~/stores/station.store';
import { FormValidationRules, VFormValidation } from '~/types/helper.types';
import { maxLengthRule } from '~/utils/form.utils';

const form = ref<VFormValidation | null>(null);
const isFormValid = computed(() => (form.value ? form.value.isValid : false));
const session = useSessionStore();
const station = useStationStore();

export interface StationInfoModel {
  name: string;
  main: boolean;
}

const props = withDefaults(
  defineProps<{
    modelValue: StationInfoModel;
    valid?: boolean;
  }>(),
  {
    valid: true,
  },
);

const rules: {
  name: FormValidationRules;
} = {
  name: [maxLengthRule(100, i18n.global.t('terms.station_name'))],
};

const emit = defineEmits<{
  (event: 'update:modelValue', payload: StationInfoModel): void;
  (event: 'valid', payload: boolean): void;
  (event: 'submit', payload: StationInfoModel): void;
}>();

watch(
  () => isFormValid.value,
  isValid => emit('valid', isValid ?? false),
);

const modelValue = computed({
  get: () => props.modelValue,
  set: value => emit('update:modelValue', value),
});

const canSetMain = computed(
  () => session.data.stations.length > 1 && session.mainStation?.toText() !== station.canisterId,
);

const submit = async () => {
  const { valid } = form.value ? await form.value.validate() : { valid: false };

  if (valid) {
    emit('submit', modelValue.value);
  }
};
</script>
