<template>
  <VForm ref="form" @submit.prevent="submit">
    <DiffView :before-value="currentName" :after-value="name">
      <template #default="{ value, diffMode }">
        <VTextField
          :model-value="value"
          :name="diffMode === 'before' ? 'name-before' : 'name'"
          :label="$t('terms.name')"
          density="comfortable"
          :rules="[maxLengthRule(48, $t('terms.name'))]"
          :variant="isViewMode ? 'plain' : 'filled'"
          :readonly="isViewMode || diffMode === 'before'"
          @update:model-value="val => diffMode === 'after' && (name = val)"
        />
      </template>
    </DiffView>

    <DiffView :before-value="currentObtainCyclesModel" :after-value="obtainCyclesModel">
      <template #default="{ value, diffMode }">
        <ObtainCyclesForm
          v-if="value"
          :model-value="value"
          :valid="valid"
          :trigger-submit="triggerSubmit"
          :current-system-info="currentSystemInfo"
          :is-view-mode="isViewMode || diffMode === 'before'"
          :is-before="diffMode === 'before'"
          @update:model-value="val => diffMode === 'after' && (obtainCyclesModel = val)"
        />
      </template>
    </DiffView>
  </VForm>
</template>

<script lang="ts" setup>
import { computed, ref, watch } from 'vue';
import { VForm, VTextField } from 'vuetify/components';
import DiffView from '~/components/requests/DiffView.vue';
import ObtainCyclesForm from '~/components/settings/obtain-cycles/ObtainCyclesForm.vue';
import {
  CycleObtainStrategyInput,
  ManageSystemInfoOperationInput,
  SystemInfo,
} from '~/generated/station/station.did';
import { VFormValidation } from '~/types/helper.types';
import { maxLengthRule } from '~/utils/form.utils';
import { unreachable, variantIs } from '~/utils/helper.utils';

const props = withDefaults(
  defineProps<{
    modelValue: Partial<ManageSystemInfoOperationInput>;
    valid?: boolean;
    triggerSubmit?: boolean;
    mode?: 'view' | 'edit';
    currentSystemInfo?: SystemInfo;
  }>(),
  {
    valid: true,
    triggerSubmit: false,
    mode: 'edit',
    currentSystemInfo: undefined,
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

const obtainCyclesModel = ref<CycleObtainStrategyInput | undefined>(
  props.modelValue.cycle_obtain_strategy?.[0],
);

const model = computed(() => props.modelValue);
watch(model.value, newValue => emit('update:modelValue', newValue), { deep: true });

watch(obtainCyclesModel, value => {
  if (value) {
    model.value.cycle_obtain_strategy = [value];
  } else {
    model.value.cycle_obtain_strategy = [];
  }
});

watch(
  () => isFormValid.value,
  isValid => emit('valid', isValid ?? false),
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

const currentName = computed(() => props.currentSystemInfo?.name);

const currentObtainCyclesModel = computed((): CycleObtainStrategyInput | undefined => {
  if (!props.currentSystemInfo?.cycle_obtain_strategy) {
    return;
  }

  if (variantIs(props.currentSystemInfo.cycle_obtain_strategy, 'Disabled')) {
    return { Disabled: null };
  } else if (variantIs(props.currentSystemInfo.cycle_obtain_strategy, 'MintFromNativeToken')) {
    return {
      MintFromNativeToken: {
        account_id: props.currentSystemInfo.cycle_obtain_strategy.MintFromNativeToken.account_id,
      },
    };
  } else if (variantIs(props.currentSystemInfo.cycle_obtain_strategy, 'WithdrawFromCyclesLedger')) {
    return {
      WithdrawFromCyclesLedger: {
        account_id:
          props.currentSystemInfo.cycle_obtain_strategy.WithdrawFromCyclesLedger.account_id,
      },
    };
  } else {
    return unreachable(props.currentSystemInfo.cycle_obtain_strategy);
  }
});

const submit = async () => {
  const { valid } = form.value ? await form.value.validate() : { valid: false };

  if (valid) {
    emit('submit', model.value);
  }
};
</script>
