<template>
  <VForm ref="form" @submit.prevent="submit">
    <VContainer class="px-0 py-2">
      <VRow>
        <VCol cols="12" class="pb-0">
          <CanisterIdField
            v-if="props.display.canisterId || !model.canisterId"
            v-model="model.canisterId"
            name="canister_id"
            density="comfortable"
            :readonly="props.readonly"
            required
          />
        </VCol>
        <VCol cols="12" class="pb-0">
          <CanisterInstallModeSelect v-model="model.mode" :readonly="props.readonly" required />
        </VCol>
        <template v-if="isUpgradeMode">
          <VCol cols="12" class="pb-0">
            <CanisterWasmMemoryPersistenceSelect
              v-model="wasmMemoryPersistence"
              :readonly="props.readonly"
              :hint="$t('external_canisters.wasm_memory_persistence.hint')"
            />
          </VCol>
          <VCol cols="12" class="pb-0">
            <VCheckbox
              v-model="skipPreUpgrade"
              :readonly="props.readonly"
              :label="$t('external_canisters.skip_pre_upgrade.label')"
              :hint="$t('external_canisters.skip_pre_upgrade.hint')"
              persistent-hint
              density="comfortable"
              :prepend-icon="mdiDebugStepOver"
            />
          </VCol>
        </template>
        <VCol cols="12" class="pb-0">
          <CanisterWasmModuleField
            v-model="model.wasmModule"
            :readonly="props.readonly"
            required
            name="wasm_module"
          />
        </VCol>
        <VCol cols="12" class="pb-0">
          <CanisterArgumentField
            v-model="model.wasmInstallArg"
            :readonly="props.readonly"
            :candid="props.candidIdl ? { idl: props.candidIdl } : undefined"
            name="argument"
          />
        </VCol>
      </VRow>
    </VContainer>

    <slot name="actions"> </slot>
  </VForm>
</template>
<script lang="ts" setup>
import { mdiDebugStepOver } from '@mdi/js';
import { computed, ref, watch } from 'vue';
import { VCheckbox, VCol, VContainer, VForm, VRow } from 'vuetify/components';
import CanisterArgumentField from '~/components/inputs/CanisterArgumentField.vue';
import CanisterInstallModeSelect from '~/components/inputs/CanisterInstallModeSelect.vue';
import CanisterWasmMemoryPersistenceSelect from '~/components/inputs/CanisterWasmMemoryPersistenceSelect.vue';
import CanisterWasmModuleField from '~/components/inputs/CanisterWasmModuleField.vue';
import { VFormValidation } from '~/types/helper.types';
import CanisterIdField from '../inputs/CanisterIdField.vue';
import {
  CanisterIcSettingsModel,
  CanisterInstallModel,
  CanisterUpgradeOptions,
  WasmMemoryPersistence,
} from './external-canisters.types';

const props = withDefaults(
  defineProps<{
    modelValue: CanisterInstallModel;
    triggerSubmit?: boolean;
    readonly?: boolean;
    candidIdl?: string;
    display?: {
      canisterId: boolean;
    };
  }>(),
  {
    readonly: false,
    triggerSubmit: false,
    candidIdl: undefined,
    display: () => ({
      canisterId: true,
    }),
  },
);

const emit = defineEmits<{
  (event: 'update:modelValue', payload: CanisterInstallModel): void;
  (event: 'update:triggerSubmit', payload: boolean): void;
  (event: 'valid', payload: boolean): void;
  (event: 'submit', payload: CanisterIcSettingsModel): void;
}>();

const form = ref<VFormValidation>();
const valid = ref(true);
const fieldsWithErrors = ref<string[]>([]);

const model = computed({
  get: () => props.modelValue,
  set: value => emit('update:modelValue', value),
});

const isUpgradeMode = computed(() => !!model.value.mode && 'upgrade' in model.value.mode);

const upgradeOptions = computed<CanisterUpgradeOptions | undefined>(() => {
  const mode = model.value.mode;
  if (mode && 'upgrade' in mode && mode.upgrade.length > 0) {
    return mode.upgrade[0];
  }

  return undefined;
});

// Merges a partial patch into the upgrade-options record, collapsing back to an
// empty record (`{ upgrade: [] }`) when neither option is set so a plain
// upgrade request is emitted.
const setUpgradeOptions = (patch: Partial<CanisterUpgradeOptions>): void => {
  const mode = model.value.mode;
  if (!mode || !('upgrade' in mode)) {
    return;
  }

  const current: CanisterUpgradeOptions = upgradeOptions.value ?? {
    wasm_memory_persistence: [],
    skip_pre_upgrade: [],
  };
  const next: CanisterUpgradeOptions = { ...current, ...patch };
  const isEmpty = next.wasm_memory_persistence.length === 0 && next.skip_pre_upgrade.length === 0;

  model.value = { ...model.value, mode: { upgrade: isEmpty ? [] : [next] } };
};

const wasmMemoryPersistence = computed<WasmMemoryPersistence | undefined>({
  get: () => upgradeOptions.value?.wasm_memory_persistence?.[0],
  set: value => setUpgradeOptions({ wasm_memory_persistence: value !== undefined ? [value] : [] }),
});

const skipPreUpgrade = computed<boolean>({
  get: () => upgradeOptions.value?.skip_pre_upgrade?.[0] ?? false,
  set: value => setUpgradeOptions({ skip_pre_upgrade: value ? [true] : [] }),
});

const triggerSubmit = computed({
  get: () => props.triggerSubmit,
  set: value => emit('update:triggerSubmit', value),
});

watch(valid, newValid => emit('valid', newValid), { immediate: true });

watch(
  () => form.value?.errors,
  _ => {
    valid.value = form.value?.isValid ?? false;
    fieldsWithErrors.value = form.value?.errors.map(error => error.id) ?? [];
  },
  { deep: true },
);

watch(triggerSubmit, shouldTrigger => {
  if (shouldTrigger) {
    emit('update:triggerSubmit', false);

    submit();
  }
});

const revalidate = async (): Promise<boolean> => {
  const { valid: isValid, errors } = form.value
    ? await form.value.validate()
    : { valid: false, errors: [] };

  valid.value = isValid;
  fieldsWithErrors.value = errors.map(error => error.id);

  return isValid;
};

const submit = async (): Promise<void> => {
  const isValid = await revalidate();

  if (isValid) {
    emit('submit', model.value);
  }
};
</script>
