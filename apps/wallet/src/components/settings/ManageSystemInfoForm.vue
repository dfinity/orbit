<template>
  <VForm ref="form" @submit.prevent="submit">
    <VTextField
      v-model="name"
      name="name"
      :label="$t('terms.name')"
      density="comfortable"
      :rules="[maxLengthRule(48, $t('terms.name'))]"
      :variant="isViewMode ? 'plain' : 'filled'"
      :disabled="isViewMode"
    />

    <VAutocomplete
      v-model="cycleObtainStrategySelected"
      class="mt-2"
      :items="cycleObtainStrategies"
      density="comfortable"
      :label="$t('terms.cycle_obtain_strategy')"
      hide-details
      clearable
      :rules="[requiredRule]"
      :variant="isViewMode ? 'plain' : 'filled'"
      :disabled="isViewMode"
    />

    <template v-if="cycleObtainStrategySelected !== null">
      <MintFromNativeToken
        v-if="cycleObtainStrategySelected == 'MintFromNativeToken'"
        v-model="mintFromNativeTokenAccountId"
        :variant="isViewMode ? 'plain' : 'filled'"
        :disabled="isViewMode"
      ></MintFromNativeToken>
      <template v-else-if="cycleObtainStrategySelected == 'Disabled'"></template>
      <template v-else>{{ unreachable(cycleObtainStrategySelected) }}</template>
    </template>
  </VForm>
</template>

<script lang="ts" setup>
import { computed, ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import { VAutocomplete, VForm, VTextField } from 'vuetify/components';
import { UUID } from '~/generated/control-panel/control_panel.did';
import { ManageSystemInfoOperationInput } from '~/generated/station/station.did';
import { cycleObtainStrategyInputToKey } from '~/mappers/obtain-cycles.mapper';
import { VFormValidation } from '~/types/helper.types';
import { CycleObtainStrategyEnum } from '~/types/obtain-cycles.types';
import { maxLengthRule, requiredRule } from '~/utils/form.utils';
import { unreachable, variantIs } from '~/utils/helper.utils';
import MintFromNativeToken from './obtain-cycle/MintFromNativeToken.vue';

const i18n = useI18n();

const props = withDefaults(
  defineProps<{
    modelValue: Partial<ManageSystemInfoOperationInput>;
    valid?: boolean;
    triggerSubmit?: boolean;
    mode?: 'view' | 'edit';
  }>(),
  {
    valid: true,
    triggerSubmit: false,
    mode: 'edit',
  },
);

const emit = defineEmits<{
  (event: 'update:modelValue', payload: Partial<ManageSystemInfoOperationInput>): void;
  (event: 'update:triggerSubmit', payload: boolean): void;
  (event: 'valid', payload: boolean): void;
  (event: 'submit', payload: Partial<ManageSystemInfoOperationInput>): void;
}>();

const form = ref<VFormValidation | null>(null);
const isFormValid = computed(() => (form.value ? form.value.isValid : false));

const model = computed(() => props.modelValue);
watch(model.value, newValue => emit('update:modelValue', newValue), { deep: true });

const cycleObtainStrategySelected = ref(
  model.value.cycle_obtain_strategy?.[0]
    ? cycleObtainStrategyInputToKey(model.value.cycle_obtain_strategy[0])
    : null,
);

const mintFromNativeTokenAccountId = ref<UUID | null>(
  model.value.cycle_obtain_strategy?.[0] &&
    variantIs(model.value.cycle_obtain_strategy?.[0], 'MintFromNativeToken')
    ? model.value.cycle_obtain_strategy?.[0].MintFromNativeToken.account_id
    : null,
);

const cycleObtainStrategyKeys = ref<CycleObtainStrategyEnum[]>([
  CycleObtainStrategyEnum.Disabled,
  CycleObtainStrategyEnum.MintFromNativeToken,
]);

const cycleObtainStrategies = computed(() => {
  return cycleObtainStrategyKeys.value.map(key => ({
    value: key,
    title: i18n.t(`cycle_obtain_strategies.${key.toLowerCase()}`),
  }));
});

watch(
  () => isFormValid.value,
  isValid => {
    if (isValid) {
      const selectedCycleObtainStrategy = cycleObtainStrategySelected.value!;

      switch (selectedCycleObtainStrategy) {
        case CycleObtainStrategyEnum.Disabled:
          model.value.cycle_obtain_strategy = [{ Disabled: null }];
          break;
        case CycleObtainStrategyEnum.MintFromNativeToken:
          model.value.cycle_obtain_strategy = [
            {
              MintFromNativeToken: {
                account_id: mintFromNativeTokenAccountId.value!,
              },
            },
          ];
          break;
        default:
          unreachable(selectedCycleObtainStrategy);
      }
    }

    emit('valid', isValid ?? false);
  },
);

const name = computed({
  get: () => model.value.name?.[0],
  set: value => {
    model.value.name = !value ? [] : [value];
  },
});

watch(
  () => props.triggerSubmit,
  () => {
    if (props.triggerSubmit) {
      emit('update:triggerSubmit', false);
      submit();
    }
  },
);

const isViewMode = computed(() => props.mode === 'view');

const submit = async () => {
  const { valid } = form.value ? await form.value.validate() : { valid: false };

  if (valid) {
    emit('submit', model.value);
  }
};
</script>
