<template>
  <VAutocomplete
    v-model="cycleObtainStrategySelected"
    class="mt-2"
    :items="cycleObtainStrategies"
    density="comfortable"
    :label="$t('terms.cycle_obtain_strategy')"
    hide-details
    :clearable="!isViewMode"
    :rules="[requiredRule]"
    :variant="isViewMode ? 'plain' : 'filled'"
    :readonly="isViewMode"
  />

  <template v-if="cycleObtainStrategySelected !== null">
    <MintFromNativeToken
      v-if="cycleObtainStrategySelected == 'MintFromNativeToken'"
      v-model="mintFromNativeTokenAccountId"
      :variant="isViewMode ? 'plain' : 'filled'"
      :element-name="isBefore ? 'account_id_before' : 'account_id'"
      :readonly="isViewMode"
    ></MintFromNativeToken>
    <WithdrawFromCyclesLedger
      v-else-if="cycleObtainStrategySelected == 'WithdrawFromCyclesLedger'"
      v-model="withdrawFromCyclesLedgerAccountId"
      :variant="isViewMode ? 'plain' : 'filled'"
      :readonly="isViewMode"
      :element-name="isBefore ? 'account_id_before' : 'account_id'"
    ></WithdrawFromCyclesLedger>
    <template v-else-if="cycleObtainStrategySelected == 'Disabled'"></template>
  </template>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue';
import { VAutocomplete } from 'vuetify/components';
import { UUID } from '~/generated/control-panel/control_panel.did';
import { CycleObtainStrategyInput, SystemInfo } from '~/generated/station/station.did';
import { cycleObtainStrategyInputToKey } from '~/mappers/obtain-cycles.mapper';
import { CycleObtainStrategyEnum } from '~/types/obtain-cycles.types';
import { requiredRule } from '~/utils/form.utils';
import { variantIs } from '~/utils/helper.utils';
import { useI18n } from 'vue-i18n';
import MintFromNativeToken from './MintFromNativeToken.vue';
import WithdrawFromCyclesLedger from './WithdrawFromCyclesLedger.vue';

const i18n = useI18n();

const props = defineProps<{
  modelValue: CycleObtainStrategyInput | undefined;
  valid?: boolean;
  triggerSubmit?: boolean;
  mode?: 'view' | 'edit';
  currentSystemInfo?: SystemInfo;
  isViewMode?: boolean;
  isBefore?: boolean;
}>();

const emit = defineEmits<{
  (event: 'update:modelValue', payload: CycleObtainStrategyInput | undefined): void;
}>();

const model = computed({
  get: () => props.modelValue,
  set: value => emit('update:modelValue', value),
});

const cycleObtainStrategySelected = ref(
  model.value ? cycleObtainStrategyInputToKey(model.value) : null,
);

const mintFromNativeTokenAccountId = ref<UUID | null>(
  model.value && variantIs(model.value, 'MintFromNativeToken')
    ? model.value.MintFromNativeToken.account_id
    : null,
);

const withdrawFromCyclesLedgerAccountId = ref<UUID | null>(
  model.value && variantIs(model.value, 'WithdrawFromCyclesLedger')
    ? model.value.WithdrawFromCyclesLedger.account_id
    : null,
);

const cycleObtainStrategyKeys = ref<CycleObtainStrategyEnum[]>([
  CycleObtainStrategyEnum.Disabled,
  CycleObtainStrategyEnum.MintFromNativeToken,
  CycleObtainStrategyEnum.WithdrawFromCyclesLedger,
]);

const cycleObtainStrategies = computed(() => {
  return cycleObtainStrategyKeys.value.map(key => ({
    value: key,
    title: i18n.t(`cycle_obtain_strategies.${key.toLowerCase()}`),
  }));
});

watch(
  () => cycleObtainStrategySelected.value,
  newValue => {
    if (newValue) {
      switch (newValue) {
        case CycleObtainStrategyEnum.Disabled:
          model.value = { Disabled: null };
          break;
      }
    }
  },
);

watch(
  () => mintFromNativeTokenAccountId.value,
  newValue => {
    if (newValue) {
      model.value = {
        MintFromNativeToken: {
          account_id: newValue,
        },
      };
    }
  },
);

watch(
  () => withdrawFromCyclesLedgerAccountId.value,
  newValue => {
    if (newValue) {
      model.value = {
        WithdrawFromCyclesLedger: {
          account_id: newValue,
        },
      };
    }
  },
);
</script>
