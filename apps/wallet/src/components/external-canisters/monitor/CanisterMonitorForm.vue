<template>
  <VForm v-bind="$attrs" ref="form" @submit.prevent="submit">
    <slot name="errors" :errors="additionalFieldErrors">
      <FormErrorsContainer v-bind="{ errors: additionalFieldErrors }" />
    </slot>
    <VContainer>
      <VRow>
        <VCol cols="12">
          <VStepper
            v-model="step"
            hide-actions
            :mobile="app.isMobile"
            flat
            class="canister-monitor-setup"
          >
            <VStepperHeader class="box-shadow-none">
              <VStepperItem
                :value="CanisterMonitorSetupStep.Fund"
                :title="$t('external_canisters.monitor.funding_strategy_label')"
              >
                <template #icon>{{ CanisterMonitorSetupStep.Fund }}</template>
              </VStepperItem>
              <VDivider />
              <VStepperItem
                :value="CanisterMonitorSetupStep.Obtain"
                :title="$t('external_canisters.monitor.cycle_obtain_strategy_label')"
              >
                <template #icon>{{ CanisterMonitorSetupStep.Obtain }}</template>
              </VStepperItem>
            </VStepperHeader>
            <VStepperWindow>
              <VAlert :icon="mdiInformationOutline" class="mb-8" :text="$t('external_canisters.monitor.funding_info_text')" />
              <VStepperWindowItem :value="CanisterMonitorSetupStep.Fund">
                <CanisterMonitorFundStep v-model="model" :display="display" />
              </VStepperWindowItem>
              <VStepperWindowItem :value="CanisterMonitorSetupStep.Obtain">
                <CanisterMonitorObtainStep v-model="model" />
              </VStepperWindowItem>
            </VStepperWindow>
          </VStepper>
        </VCol>
        <VDivider />
        <VCol cols="12" class="d-flex">
          <VBtn :disabled="!canReturn" variant="text" @click="step--">
            {{ $t('terms.previous') }}
          </VBtn>
          <VSpacer />
          <VBtn
            v-if="canAdvance"
            variant="tonal"
            color="primary-variant"
            data-test-id="monitor-dialog-stepper-next"
            @click="step++"
          >
            {{ $t('terms.next') }}
          </VBtn>
          <VBtn
            v-if="showSubmit"
            :disabled="!valid"
            :loading="submitting"
            variant="elevated"
            color="primary"
            data-test-id="monitor-dialog-submit"
            @click="submit"
          >
            {{ $t('terms.save') }}
          </VBtn>
        </VCol>
      </VRow>
    </VContainer>
  </VForm>
</template>
<script lang="ts" setup>
import { computed, ref, watch } from 'vue';
import {
  VBtn,
  VCol,
  VContainer,
  VDivider,
  VForm,
  VRow,
  VSpacer,
  VStepper,
  VStepperHeader,
  VStepperItem,
  VStepperWindow,
  VStepperWindowItem,
} from 'vuetify/components';
import { assertAndReturn } from '~/utils/helper.utils.ts';
import {
  useOnFailedOperation,
  useOnSuccessfulOperation,
} from '~/composables/notifications.composable.ts';
import logger from '~/core/logger.core.ts';
import { useStationStore } from '~/stores/station.store.ts';
import FormErrorsContainer from '~/components/ui/FormErrorsContainer.vue';
import { useForm } from '~/composables/forms.composable.ts';
import { CanisterMonitorModel } from '~/components/external-canisters/external-canisters.types.ts';
import { CanisterMonitorSetupStep } from '~/components/external-canisters/monitor/monitor.types.ts';
import { useAppStore } from '~/stores/app.store.ts';
import CanisterMonitorFundStep from '~/components/external-canisters/monitor/CanisterMonitorFundStep.vue';
import CanisterMonitorObtainStep from '~/components/external-canisters/monitor/CanisterMonitorObtainStep.vue';
import { mdiInformationOutline } from '@mdi/js';

const props = withDefaults(
  defineProps<{
    step?: 1 | 2;
    modelValue: CanisterMonitorModel;
    readonly?: boolean;
    display?: {
      canisterId: boolean;
    };
  }>(),
  {
    step: 1,
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

const station = useStationStore();
const app = useAppStore();
const step = ref(props.step);

const canAdvance = computed(() => step.value < CanisterMonitorSetupStep.Obtain);
const canReturn = computed(() => step.value > CanisterMonitorSetupStep.Fund);
const showSubmit = computed(() => step.value === CanisterMonitorSetupStep.Obtain);

const { submit, edited, additionalFieldErrors, submitting, valid, submitted } = useForm({
  model,
  submit: async (updatedModel: CanisterMonitorModel): Promise<void> => {
    try {
      const strategy = assertAndReturn(updatedModel.fundingStrategy, 'Funding strategy');
      const canisterId = assertAndReturn(updatedModel.canisterId, 'Canister ID');

      const request = await station.service.monitorExternalCanister({
        canister_id: canisterId,
        kind: {
          Start: {
            funding_strategy: strategy,
            cycle_obtain_strategy: updatedModel.cycleObtainStrategy
              ? [updatedModel.cycleObtainStrategy]
              : [],
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
<style lang="scss">
.canister-monitor-setup {
  & .v-stepper-window {
    margin-top: 0;
    margin-bottom: 0;
  }
}
</style>
