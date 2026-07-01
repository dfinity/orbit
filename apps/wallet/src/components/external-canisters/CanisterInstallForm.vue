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
        <template v-if="isUpgrade">
          <VCol cols="12" class="pb-0">
            <WasmMemoryPersistenceSelect
              v-model="wasmMemoryPersistence"
              :readonly="props.readonly"
              name="wasm_memory_persistence"
            />
          </VCol>
          <VCol cols="12" class="pb-0">
            <VCheckbox
              v-model="skipPreUpgrade"
              name="skip_pre_upgrade"
              :label="$t('external_canisters.skip_pre_upgrade')"
              :readonly="props.readonly"
              density="comfortable"
              hide-details
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
import { computed, ref, watch } from 'vue';
import { VCheckbox, VCol, VContainer, VForm, VRow } from 'vuetify/components';
import CanisterArgumentField from '~/components/inputs/CanisterArgumentField.vue';
import CanisterInstallModeSelect from '~/components/inputs/CanisterInstallModeSelect.vue';
import CanisterWasmModuleField from '~/components/inputs/CanisterWasmModuleField.vue';
import WasmMemoryPersistenceSelect from '~/components/inputs/WasmMemoryPersistenceSelect.vue';
import { VFormValidation } from '~/types/helper.types';
import { WasmMemoryPersistence } from '~/types/station.types';
import { variantIs } from '~/utils/helper.utils';
import CanisterIdField from '../inputs/CanisterIdField.vue';
import { CanisterIcSettingsModel, CanisterInstallModel } from './external-canisters.types';

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

const isUpgrade = computed(
  () => model.value.mode !== undefined && variantIs(model.value.mode, 'upgrade'),
);

// The upgrade options (wasm memory persistence, skip pre-upgrade) live inside
// the `upgrade` variant of `model.mode`.
const upgradeOptions = computed(() => {
  const mode = model.value.mode;
  return mode && variantIs(mode, 'upgrade') ? mode.upgrade[0] : undefined;
});

const setUpgradeOptions = (
  wasmMemoryPersistence: [] | [WasmMemoryPersistence],
  skipPreUpgrade: [] | [boolean],
): void => {
  // Collapse to `{ upgrade: [] }` when no option is set so the request matches
  // one created without any upgrade options.
  const hasOptions = wasmMemoryPersistence.length > 0 || skipPreUpgrade.length > 0;
  model.value = {
    ...model.value,
    mode: {
      upgrade: hasOptions
        ? [{ wasm_memory_persistence: wasmMemoryPersistence, skip_pre_upgrade: skipPreUpgrade }]
        : [],
    },
  };
};

const wasmMemoryPersistence = computed<WasmMemoryPersistence | undefined>({
  get: () => upgradeOptions.value?.wasm_memory_persistence[0],
  set: value =>
    setUpgradeOptions(value ? [value] : [], upgradeOptions.value?.skip_pre_upgrade ?? []),
});

const skipPreUpgrade = computed<boolean>({
  get: () => upgradeOptions.value?.skip_pre_upgrade[0] ?? false,
  set: value =>
    setUpgradeOptions(upgradeOptions.value?.wasm_memory_persistence ?? [], value ? [true] : []),
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
