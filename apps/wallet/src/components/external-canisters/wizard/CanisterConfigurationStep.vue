<template>
  <VRow class="py-2">
    <VCol v-if="model.id" cols="12" class="pt-0 pb-0">
      <VTextField
        v-model="model.canisterId"
        name="id"
        :prepend-icon="mdiIdentifier"
        class="pt-4 pb-2"
        :label="$t('terms.id')"
        variant="plain"
        density="comfortable"
        hide-details
        disabled
      />
    </VCol>
    <VCol cols="12" class="pt-2 pb-0">
      <VTextField
        v-model="model.name"
        name="name"
        :label="$t('terms.name')"
        variant="filled"
        density="comfortable"
        :rules="[requiredRule]"
        :disabled="isViewMode"
        :prepend-icon="mdiDatabase"
      />
    </VCol>
    <VCol cols="12" class="pt-2 pb-0">
      <VTextarea
        v-model="model.description"
        name="description"
        :label="$t('terms.description')"
        variant="filled"
        density="comfortable"
        :disabled="isViewMode"
        :prepend-icon="mdiText"
      />
    </VCol>
    <VCol cols="12" class="pt-2">
      <SuggestiveAutocomplete
        v-model="model.labels"
        :icon="mdiLabel"
        :readonly="isViewMode"
        :items="availableLabels"
        :label="$t('terms.labels')"
        :placeholder="$t('external_canisters.add_new_label')"
      />
    </VCol>
    <VCol v-if="!isCreationMode" cols="12" class="pt-2 pb-0 d-flex">
      <div><VIcon :icon="mdiCogs" class="text-medium-emphasis" /></div>
      <div>
        <VRadioGroup v-model="model.state" name="state" inline :label="$t('terms.status')">
          <VRadio
            v-for="(state, idx) in availableStates"
            :key="idx"
            :label="state.text"
            :value="state.key"
            :readonly="isViewMode"
            class="mr-4"
          />
        </VRadioGroup>
      </div>
    </VCol>
    <VCol v-if="isCreationMode" cols="12" class="pt-0 pb-0">
      <h3 class="text-h6 mt-4 mb-2">{{ $t('external_canisters.target_canister') }}</h3>
      <VDivider />
      <VRadioGroup v-model="creationModeKind" name="creation_kind" inline class="mt-2">
        <VRadio :label="$t('external_canisters.create_new')" value="new" class="mr-4" />
        <VRadio :label="$t('external_canisters.use_existing')" value="existing" />
      </VRadioGroup>
      <div v-if="creationModeKind === 'existing'">
        <VTextField
          v-model="canisterIdInput"
          name="canister_id"
          :label="$t('terms.canister_id')"
          variant="filled"
          density="comfortable"
          :rules="[requiredRule, validCanisterId]"
          :prepend-icon="mdiIdentifier"
        />
      </div>
      <CyclesInput
        v-else
        v-model="model.maybe_with_initial_cycles"
        name="initial_cycles"
        :unit="CyclesUnit.Trillion"
        :label="$t('external_canisters.initial_cycles')"
      />
    </VCol>
  </VRow>
</template>

<script lang="ts" setup>
import { Principal } from '@icp-sdk/core/principal';
import { mdiCogs, mdiDatabase, mdiIdentifier, mdiLabel, mdiText } from '@mdi/js';
import { computed, onBeforeMount, ref, watch } from 'vue';
import {
  VCol,
  VDivider,
  VIcon,
  VRadio,
  VRadioGroup,
  VRow,
  VTextField,
  VTextarea,
} from 'vuetify/components';
import CyclesInput from '~/components/inputs/CyclesInput.vue';
import SuggestiveAutocomplete from '~/components/inputs/SuggestiveAutocomplete.vue';
import { useExternalCanistersStates } from '~/composables/external-canisters.composable';
import logger from '~/core/logger.core';
import { useStationStore } from '~/stores/station.store';
import { CyclesUnit } from '~/types/app.types';
import { SelectItem } from '~/types/helper.types';
import { requiredRule, validCanisterId } from '~/utils/form.utils';
import { CanisterConfigurationModel } from './wizard.types';

const props = withDefaults(
  defineProps<{
    modelValue: Partial<CanisterConfigurationModel>;
    mode: 'view' | 'edit';
  }>(),
  {
    mode: 'edit',
    triggerSubmit: false,
  },
);

const emit = defineEmits<{
  (event: 'update:modelValue', payload: Partial<CanisterConfigurationModel>): void;
}>();

const model = computed({
  get: () => props.modelValue,
  set: value => emit('update:modelValue', value),
});

const station = useStationStore();
const isViewMode = computed(() => props.mode === 'view');
const isCreationMode = computed(() => !model.value.id);
const availableStates = useExternalCanistersStates();
const creationModeKind = ref<'existing' | 'new'>(model.value.canisterId ? 'existing' : 'new');
const availableLabels = ref<SelectItem<string>[]>([]);
const canisterIdInput = ref<string | undefined>(model.value.canisterId?.toText());

watch(canisterIdInput, newValue => {
  try {
    newValue = newValue?.trim();

    if (!newValue) {
      throw new Error('Empty canisterId');
    }

    model.value = {
      ...model.value,
      canisterId: Principal.fromText(newValue),
    };
  } catch (_) {
    // Unset the canisterId if the input is invalid or empty
    model.value = {
      ...model.value,
      canisterId: undefined,
    };
  }
});

watch(
  () => model.value.canisterId,
  updatedCanisterId => {
    if (updatedCanisterId) {
      canisterIdInput.value = updatedCanisterId.toText();
    }
  },
  { immediate: true },
);

watch(
  () => creationModeKind.value,
  () => {
    if (creationModeKind.value === 'new') {
      model.value.canisterId = undefined;
      model.value.maybe_with_initial_cycles = undefined;
    }
  },
  { immediate: true },
);

onBeforeMount(async () => {
  try {
    const result = await station.service.fetchExternalCanisterFilters({
      with_labels: true,
    });
    const labels = result.labels?.[0] ?? [];
    availableLabels.value = labels.map((label: string) => ({ value: label, text: label }));
  } catch (error) {
    logger.error('Failed to fetch external canister filters', error);
  }
});
</script>
