<template>
  <VRow>
    <VCol v-if="props.display.canisterId || !model.canisterId" cols="12">
      <CanisterIdField
        v-model="model.canisterId"
        name="canister_id"
        :readonly="props.readonly"
        required
      />
    </VCol>
    <VCol cols="12">
      <VSelect
        v-model="monitoringStrategySelected"
        :label="$t('external_canisters.monitor.strategy.label')"
        :readonly="props.readonly"
        :items="strategies"
        :prepend-icon="mdiCog"
        :rules="[requiredRule]"
        name="funding-strategy"
      />
    </VCol>
  </VRow>
  <template v-if="model.fundingStrategy">
    <template v-if="variantIs(model.fundingStrategy, MonitoringStrategyEnum.BelowEstimatedRuntime)">
      <VRow>
        <VCol cols="12">
          <SecondsInput
            v-model="model.fundingStrategy.BelowEstimatedRuntime.fund_runtime_secs"
            :label="$t('external_canisters.monitor.strategy.fund_runtime_secs')"
            name="fund_runtime_secs"
            :unit="TimeUnit.Days"
            :readonly="props.readonly"
            required
            :hint="$t('external_canisters.monitor.strategy.fund_runtime_secs_hint')"
          />
        </VCol>
        <VCol cols="12">
          <SecondsInput
            v-model="model.fundingStrategy.BelowEstimatedRuntime.min_runtime_secs"
            :label="$t('external_canisters.monitor.strategy.min_runtime_secs')"
            name="min_runtime_secs"
            :unit="TimeUnit.Days"
            :readonly="props.readonly"
            required
            :hint="$t('external_canisters.monitor.strategy.min_runtime_secs_hint')"
          />
        </VCol>
        <VCol cols="12">
          <CyclesInput
            v-model="model.fundingStrategy.BelowEstimatedRuntime.max_runtime_cycles_fund"
            :label="$t('external_canisters.monitor.strategy.max_runtime_cycles_fund')"
            name="max_runtime_cycles_fund"
            :unit="CyclesUnit.Trillion"
            :readonly="props.readonly"
            required
            :hint="$t('external_canisters.monitor.strategy.max_runtime_cycles_fund_hint')"
          />
        </VCol>
        <VCol cols="12">
          <CyclesInput
            v-model="model.fundingStrategy.BelowEstimatedRuntime.fallback_fund_cycles"
            :label="$t('external_canisters.monitor.strategy.fallback_fund_cycles')"
            name="fallback_fund_cycles"
            :unit="CyclesUnit.Trillion"
            :readonly="props.readonly"
            required
            :hint="$t('external_canisters.monitor.strategy.fallback_fund_cycles_hint')"
          />
        </VCol>
        <VCol cols="12">
          <CyclesInput
            v-model="model.fundingStrategy.BelowEstimatedRuntime.fallback_min_cycles"
            :label="$t('external_canisters.monitor.strategy.fallback_min_cycles')"
            name="fallback_min_cycles"
            :unit="CyclesUnit.Trillion"
            :readonly="props.readonly"
            required
            :hint="$t('external_canisters.monitor.strategy.fallback_min_cycles_hint')"
          />
        </VCol>
      </VRow>
    </template>
    <template v-else-if="variantIs(model.fundingStrategy, MonitoringStrategyEnum.BelowThreshold)">
      <VRow>
        <VCol cols="12">
          <CyclesInput
            v-model="model.fundingStrategy.BelowThreshold.fund_cycles"
            :label="$t('external_canisters.monitor.strategy.fund_cycles')"
            name="fund_cycles"
            :unit="CyclesUnit.Trillion"
            :readonly="props.readonly"
            required
            :hint="$t('external_canisters.monitor.strategy.fund_cycles_hint')"
          />
        </VCol>
        <VCol cols="12">
          <CyclesInput
            v-model="model.fundingStrategy.BelowThreshold.min_cycles"
            :label="$t('external_canisters.monitor.strategy.min_cycles')"
            name="min_cycles"
            :unit="CyclesUnit.Trillion"
            :readonly="props.readonly"
            required
            :hint="$t('external_canisters.monitor.strategy.min_cycles_hint')"
          />
        </VCol>
      </VRow>
    </template>
    <template v-else-if="variantIs(model.fundingStrategy, MonitoringStrategyEnum.Always)">
      <VRow>
        <VCol cols="12">
          <CyclesInput
            v-model="model.fundingStrategy.Always"
            :label="$t('external_canisters.monitor.strategy.fund_cycles')"
            name="always_fund_cycles"
            :unit="CyclesUnit.Trillion"
            :readonly="props.readonly"
            required
            :hint="$t('external_canisters.monitor.strategy.fund_cycles_hint')"
          />
        </VCol>
      </VRow>
    </template>
  </template>
</template>

<script lang="ts" setup>
import { computed, ref } from 'vue';
import { VCol, VRow } from 'vuetify/components';
import CanisterIdField from '~/components/inputs/CanisterIdField.vue';
import { useI18n } from 'vue-i18n';
import { CyclesUnit, TimeUnit } from '~/types/app.types.ts';
import { variantIs } from '~/utils/helper.utils.ts';
import { requiredRule } from '~/utils/form.utils.ts';
import { mdiCog } from '@mdi/js';
import CyclesInput from '~/components/inputs/CyclesInput.vue';
import SecondsInput from '~/components/inputs/SecondsInput.vue';
import { CanisterMonitorModel } from '~/components/external-canisters/external-canisters.types.ts';
import { MonitoringStrategyEnum } from '~/components/external-canisters/monitor/monitor.types.ts';

const props = withDefaults(
  defineProps<{
    modelValue: CanisterMonitorModel;
    readonly?: boolean;
    display?: {
      canisterId: boolean;
    };
  }>(),
  {
    readonly: false,
    display: () => ({
      canisterId: true,
    }),
  },
);

const i18n = useI18n();

const emit = defineEmits<{
  (event: 'update:modelValue', payload: CanisterMonitorModel): void;
}>();

const model = computed({
  get: () => props.modelValue,
  set: value => emit('update:modelValue', value),
});

const monitoringStrategySelected = computed({
  get: () => {
    if (model.value.fundingStrategy) {
      if (MonitoringStrategyEnum.BelowEstimatedRuntime in model.value.fundingStrategy) {
        return MonitoringStrategyEnum.BelowEstimatedRuntime;
      } else if (MonitoringStrategyEnum.BelowThreshold in model.value.fundingStrategy) {
        return MonitoringStrategyEnum.BelowThreshold;
      } else if (MonitoringStrategyEnum.Always in model.value.fundingStrategy) {
        return MonitoringStrategyEnum.Always;
      }
    }
    return null;
  },
  set: newValue => {
    if (newValue) {
      switch (newValue) {
        case MonitoringStrategyEnum.BelowEstimatedRuntime:
          model.value.fundingStrategy = {
            BelowEstimatedRuntime: {
              fund_runtime_secs: BigInt(28 * 24 * 60 * 60),
              min_runtime_secs: BigInt(14 * 24 * 60 * 60),
              max_runtime_cycles_fund: BigInt(1_000_000_000_000),
              fallback_fund_cycles: BigInt(250_000_000_000),
              fallback_min_cycles: BigInt(125_000_000_000),
            },
          };
          break;
        case MonitoringStrategyEnum.BelowThreshold:
          model.value.fundingStrategy = {
            BelowThreshold: {
              fund_cycles: BigInt(250_000_000_000),
              min_cycles: BigInt(125_000_000_000),
            },
          };
          break;
        case MonitoringStrategyEnum.Always:
          model.value.fundingStrategy = { Always: BigInt(250_000_000_000) };
          break;
      }
    }
  },
});

const strategyKeys = ref<MonitoringStrategyEnum[]>([
  MonitoringStrategyEnum.BelowEstimatedRuntime,
  MonitoringStrategyEnum.BelowThreshold,
  MonitoringStrategyEnum.Always,
]);

const strategies = computed<
  {
    title: string;
    value: string;
  }[]
>(() =>
  strategyKeys.value.map(key => ({
    title: i18n.t(`external_canisters.monitor.strategy.${key}`),
    value: key,
  })),
);
</script>
