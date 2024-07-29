<template>
  <VAlert type="warning" density="compact" variant="tonal" class="mb-4">
    {{ $t('app.advanced_software_update_warning') }}
  </VAlert>

  <VSelect
    v-model="upgradeTarget"
    name="target"
    :items="upgradeTargetItems"
    :label="$t('app.canister_upgrade_target')"
    :prepend-icon="mdiTarget"
    variant="filled"
    density="comfortable"
  />

  <VFileInput
    v-model="wasmModuleFile"
    name="wasm"
    :label="$t('app.canister_wasm_module')"
    :rules="[requiredRule]"
    :prepend-icon="mdiCube"
    variant="filled"
    density="comfortable"
  />

  <VTextarea
    v-model="wasmInitArg"
    name="arg"
    :label="$t(`app.canister_upgrade_args_input`)"
    :prepend-icon="mdiCodeArray"
    :hint="$t(`app.canister_upgrade_args_input_hint`)"
    variant="filled"
    density="comfortable"
  />
</template>

<script lang="ts" setup>
import { mdiCodeArray, mdiCube, mdiTarget } from '@mdi/js';
import { computed, ref, watch } from 'vue';
import { VAlert, VFileInput, VSelect, VTextarea } from 'vuetify/components';
import { ChangeCanisterFormValue } from '~/components/change-canister/change-canister.types';
import {
  useDefaultUpgradeFormValue,
  useUpgradeTargets,
} from '~/composables/change-canister.composable';
import logger from '~/core/logger.core';
import { ChangeCanisterTargetType } from '~/types/station.types';
import { readFileAsArrayBuffer } from '~/utils/file.utils';
import { requiredRule } from '~/utils/form.utils';

const props = defineProps<{
  modelValue: ChangeCanisterFormValue;
}>();

const emit = defineEmits<{
  (event: 'update:modelValue', payload: ChangeCanisterFormValue): void;
}>();

const modelValue = computed({
  get: () => props.modelValue,
  set: value => emit('update:modelValue', value),
});

const wasmModuleFile = ref<File[]>([]);
const upgradeTargets = useUpgradeTargets();
const upgradeTarget = ref<ChangeCanisterTargetType>(ChangeCanisterTargetType.UpgradeStation);
const upgradeTargetItems = computed(() => Object.values(upgradeTargets.value));
const wasmInitArg = ref<string>(props.modelValue.wasmInitArg ?? '');

const updateComputedCanisterModule = async () => {
  if (!wasmModuleFile.value || wasmModuleFile.value.length === 0) {
    modelValue.value = {
      ...modelValue.value,
      wasmModule: undefined,
    };

    return;
  }

  try {
    const wasmModule = await readFileAsArrayBuffer(wasmModuleFile.value[0]);
    modelValue.value = {
      ...modelValue.value,
      wasmModule,
    };
  } catch (error) {
    logger.error('Failed to read wasm module file', error);
  }
};

watch(
  () => wasmModuleFile.value,
  () => updateComputedCanisterModule(),
  { deep: true },
);

watch(
  () => upgradeTarget.value,
  () => {
    switch (upgradeTarget.value) {
      case ChangeCanisterTargetType.UpgradeStation:
        modelValue.value = {
          ...modelValue.value,
          target: { UpgradeStation: null },
        };
        break;
      case ChangeCanisterTargetType.UpgradeUpgrader:
        modelValue.value = {
          ...modelValue.value,
          target: { UpgradeUpgrader: null },
        };
        break;
      default:
        wasmModuleFile.value = [];
        modelValue.value = useDefaultUpgradeFormValue();
        break;
    }
  },
  { immediate: true },
);

watch(
  () => wasmInitArg.value,
  () => {
    modelValue.value = {
      ...modelValue.value,
      wasmInitArg: wasmInitArg.value?.length ? wasmInitArg.value : undefined,
    };
  },
);
</script>
