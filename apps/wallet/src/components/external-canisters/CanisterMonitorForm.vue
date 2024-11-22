<template>
  <VForm v-bind="$attrs" ref="form" @submit.prevent="submit">
    <slot name="errors" :errors="additionalFieldErrors">
      <FormErrorsContainer v-bind="{ errors: additionalFieldErrors }" />
    </slot>
    <VContainer>
      <VRow>
        <VCol cols="12" class="pb-0">
          <CanisterIdField
            v-if="props.display.canisterId || !model.canisterId"
            v-model="model.canisterId"
            name="canister_id"
            :readonly="props.readonly"
            required
          />
        </VCol>
      </VRow>
      <VRow>
        <VCol>
          <VContainer class="px-0 py-0">
            <VRow>
              <VCol cols="12" class="pb-0">
                <VSelect
                  v-model="monitoringStrategySelected"
                  :label="$t('external_canisters.monitor.strategy.label')"
                  :readonly="props.readonly"
                  :items="strategies"
                  :prepend-icon="mdiCog"
                  :rules="[requiredRule]"
                  name="strategy"
                />
              </VCol>
            </VRow>
            <template v-if="model.strategy">
              <template
                v-if="variantIs(model.strategy, MonitoringStrategyEnum.BelowEstimatedRuntime)"
              >
                <VRow>
                  <VCol cols="12" class="pb-0">
                    <SecondsInput
                      v-model="model.strategy.BelowEstimatedRuntime.fund_runtime_secs"
                      :label="$t('external_canisters.monitor.strategy.fund_runtime_secs')"
                      name="fund_runtime_secs"
                      :unit="TimeUnit.Days"
                      :readonly="props.readonly"
                      required
                      :hint="$t('external_canisters.monitor.strategy.fund_runtime_secs_hint')"
                      @keydown.enter.stop.prevent="submit"
                    />
                  </VCol>
                  <VCol cols="12" class="pb-0">
                    <SecondsInput
                      v-model="model.strategy.BelowEstimatedRuntime.min_runtime_secs"
                      :label="$t('external_canisters.monitor.strategy.min_runtime_secs')"
                      name="min_runtime_secs"
                      :unit="TimeUnit.Days"
                      :readonly="props.readonly"
                      required
                      :hint="$t('external_canisters.monitor.strategy.min_runtime_secs_hint')"
                      @keydown.enter.stop.prevent="submit"
                    />
                  </VCol>
                  <VCol cols="12" class="pb-0">
                    <CyclesInput
                      v-model="model.strategy.BelowEstimatedRuntime.max_runtime_cycles_fund"
                      :label="$t('external_canisters.monitor.strategy.max_runtime_cycles_fund')"
                      name="max_runtime_cycles_fund"
                      :unit="CyclesUnit.Billion"
                      :readonly="props.readonly"
                      required
                      :hint="$t('external_canisters.monitor.strategy.max_runtime_cycles_fund_hint')"
                      @keydown.enter.stop.prevent="submit"
                    />
                  </VCol>
                  <VCol cols="12" class="pb-0">
                    <CyclesInput
                      v-model="model.strategy.BelowEstimatedRuntime.fallback_fund_cycles"
                      :label="$t('external_canisters.monitor.strategy.fallback_fund_cycles')"
                      name="fallback_fund_cycles"
                      :unit="CyclesUnit.Billion"
                      :readonly="props.readonly"
                      required
                      :hint="$t('external_canisters.monitor.strategy.fallback_fund_cycles_hint')"
                      @keydown.enter.stop.prevent="submit"
                    />
                  </VCol>
                  <VCol cols="12" class="pb-0">
                    <CyclesInput
                      v-model="model.strategy.BelowEstimatedRuntime.fallback_min_cycles"
                      :label="$t('external_canisters.monitor.strategy.fallback_min_cycles')"
                      name="fallback_min_cycles"
                      :unit="CyclesUnit.Billion"
                      :readonly="props.readonly"
                      required
                      :hint="$t('external_canisters.monitor.strategy.fallback_min_cycles_hint')"
                      @keydown.enter.stop.prevent="submit"
                    />
                  </VCol>
                </VRow>
              </template>
              <template
                v-else-if="variantIs(model.strategy, MonitoringStrategyEnum.BelowThreshold)"
              >
                <VRow>
                  <VCol cols="12" class="pb-0">
                    <CyclesInput
                      v-model="model.strategy.BelowThreshold.fund_cycles"
                      :label="$t('external_canisters.monitor.strategy.fund_cycles')"
                      name="fund_cycles"
                      :unit="CyclesUnit.Billion"
                      :readonly="props.readonly"
                      required
                      :hint="$t('external_canisters.monitor.strategy.fund_cycles_hint')"
                      @keydown.enter.stop.prevent="submit"
                    />
                  </VCol>
                  <VCol cols="12" class="pb-0">
                    <CyclesInput
                      v-model="model.strategy.BelowThreshold.min_cycles"
                      :label="$t('external_canisters.monitor.strategy.min_cycles')"
                      name="min_cycles"
                      :unit="CyclesUnit.Billion"
                      :readonly="props.readonly"
                      required
                      :hint="$t('external_canisters.monitor.strategy.min_cycles_hint')"
                      @keydown.enter.stop.prevent="submit"
                    />
                  </VCol>
                </VRow>
              </template>
              <template v-else-if="variantIs(model.strategy, MonitoringStrategyEnum.Always)">
                <VRow>
                  <VCol cols="12" class="pb-0">
                    <CyclesInput
                      v-model="model.strategy.Always"
                      :label="$t('external_canisters.monitor.strategy.fund_cycles')"
                      name="always_fund_cycles"
                      :unit="CyclesUnit.Billion"
                      :readonly="props.readonly"
                      required
                      :hint="$t('external_canisters.monitor.strategy.fund_cycles_hint')"
                      @keydown.enter.stop.prevent="submit"
                    />
                  </VCol>
                </VRow>
              </template>
            </template>
          </VContainer>
        </VCol>
      </VRow>
    </VContainer>
  </VForm>
  <slot
    v-if="!readonly"
    name="actions"
    :valid="valid"
    :submitting="submitting"
    :edited="edited"
    :submit="submit"
  >
    <FormActions v-bind="{ valid, submitting, edited, submit }" />
  </slot>
</template>
<script lang="ts" setup>
import { computed, ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import { VCol, VContainer, VForm, VRow, VSelect } from 'vuetify/components';
import CanisterIdField from '../inputs/CanisterIdField.vue';
import { MonitoringStrategyEnum } from '~/types/station.types.ts';
import { CyclesUnit, TimeUnit } from '~/types/app.types.ts';
import CyclesInput from '~/components/inputs/CyclesInput.vue';
import { mdiCog } from '@mdi/js';
import { assertAndReturn, variantIs } from '~/utils/helper.utils.ts';
import SecondsInput from '~/components/inputs/SecondsInput.vue';
import { requiredRule } from '~/utils/form.utils.ts';
import FormActions from '~/components/ui/FormActions.vue';
import {
  useOnFailedOperation,
  useOnSuccessfulOperation,
} from '~/composables/notifications.composable.ts';
import logger from '~/core/logger.core.ts';
import { useStationStore } from '~/stores/station.store.ts';
import FormErrorsContainer from '~/components/ui/FormErrorsContainer.vue';
import { useForm } from '~/composables/forms.composable.ts';
import { CanisterMonitorModel } from '~/components/external-canisters/external-canisters.types.ts';

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
const emit = defineEmits<{
  (event: 'update:modelValue', payload: CanisterMonitorModel): void;
  (event: 'valid', payload: boolean): void;
  (event: 'edited', payload: boolean): void;
  (event: 'submitting', payload: boolean): void;
  (event: 'submitted'): void;
}>();

const model = computed({
  get: () => props.modelValue,
  set: value => emit('update:modelValue', value),
});

const i18n = useI18n();
const station = useStationStore();

const { submit, edited, additionalFieldErrors, submitting, valid, submitted } = useForm({
  model,
  submit: async (updatedModel: CanisterMonitorModel): Promise<void> => {
    try {
      const strategy = assertAndReturn(updatedModel.strategy, 'Strategy');
      const canisterId = assertAndReturn(updatedModel.canisterId, 'Canister ID');

      const request = await station.service.monitorExternalCanister({
        canister_id: canisterId,
        kind: {
          Start: {
            funding_strategy: strategy,
            cycle_obtain_strategy: [],
          },
        },
      });

      useOnSuccessfulOperation(request);
    } catch (error) {
      logger.error('Failed to submit monitoring request', error);
      useOnFailedOperation();
    }
  },
});

const monitoringStrategySelected = ref(
  model.value.strategy ? model.value.strategy.toString() : null,
);

watch(
  () => monitoringStrategySelected.value,
  newValue => {
    if (newValue) {
      switch (newValue) {
        case MonitoringStrategyEnum.BelowEstimatedRuntime:
          model.value.strategy = {
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
          model.value.strategy = {
            BelowThreshold: {
              fund_cycles: BigInt(250_000_000_000),
              min_cycles: BigInt(125_000_000_000),
            },
          };
          break;
        case MonitoringStrategyEnum.Always:
          model.value.strategy = { Always: BigInt(250_000_000_000) };
          break;
      }
    }
  },
);

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

watch(valid, value => emit('valid', value));
watch(edited, value => emit('edited', value));
watch(submitting, value => emit('submitting', value));
watch(
  submitted,
  (value, _) => {
    if (value) {
      emit('submitted');
    }
  },
  { immediate: true },
);
</script>
