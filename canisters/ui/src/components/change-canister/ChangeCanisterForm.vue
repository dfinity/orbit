<template>
  <VForm ref="form" @submit.prevent="submit">
    <VSelect
      v-model="upgradeTarget"
      name="target"
      :items="upgradeTargetItems"
      :label="$t('app.canister_upgrade_target')"
      :prepend-icon="mdiTarget"
      variant="underlined"
    />

    <VFileInput
      v-model="modelValue.wasmModule"
      name="wasm"
      :label="$t('app.canister_wasm_module')"
      :rules="[requiredRule]"
      :prepend-icon="mdiCube"
      variant="underlined"
    >
      <template v-if="moduleChecksum" #counter>
        {{ $t('terms.checksum') }}: {{ moduleChecksum }}
      </template>
    </VFileInput>

    <VTextarea
      v-model="modelValue.arg"
      name="arg"
      :label="$t(`app.canister_upgrade_args_input`)"
      :prepend-icon="mdiMessageText"
      :hint="$t(`app.canister_upgrade_args_input_hint`)"
      variant="underlined"
    />
  </VForm>
</template>

<script lang="ts" setup>
import { mdiCube, mdiMessageText, mdiTarget } from '@mdi/js';
import { computed, ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import { ChangeCanisterTarget } from '~/generated/wallet/wallet.did';
import { VFormValidation } from '~/types/helper.types';
import { ChangeCanisterTargetType } from '~/types/wallet.types';
import { arrayBufferToHashHex } from '~/utils/crypto.utils';
import { readFileAsArrayBuffer } from '~/utils/file.utils';
import { requiredRule } from '~/utils/form.utils';

export type ChangeCanisterFormProps = {
  modelValue: {
    target: ChangeCanisterTarget | null;
    wasmModule: File[] | undefined;
    arg: string | null;
  };
  valid?: boolean;
};

const i18n = useI18n();

const upgradeTarget = ref<ChangeCanisterTargetType>(ChangeCanisterTargetType.UpgradeWallet);
const upgradeTargetItems = computed(() => [
  {
    value: ChangeCanisterTargetType.UpgradeWallet,
    title: i18n.t('change_canister.targets.upgradewallet'),
  },
  {
    value: ChangeCanisterTargetType.UpgradeUpgrader,
    title: i18n.t('change_canister.targets.upgradeupgrader'),
  },
]);

const form = ref<VFormValidation | null>(null);
const isFormValid = computed(() => (form.value ? form.value.isValid : false));
const moduleChecksum = ref<string | null>(null);

const props = withDefaults(defineProps<ChangeCanisterFormProps>(), {
  valid: true,
});

const emit = defineEmits<{
  (event: 'update:modelValue', payload: ChangeCanisterFormProps['modelValue']): void;
  (event: 'valid', payload: boolean): void;
  (event: 'submit', payload: ChangeCanisterFormProps['modelValue']): void;
}>();

watch(
  () => isFormValid.value,
  isValid => emit('valid', isValid ?? false),
);

const modelValue = computed({
  get: () => props.modelValue,
  set: value => emit('update:modelValue', value),
});

watch(
  () => modelValue.value,
  () => updateComputedCanisterModule(),
  { deep: true },
);

watch(
  () => upgradeTarget.value,
  () => {
    switch (upgradeTarget.value) {
      case ChangeCanisterTargetType.UpgradeWallet:
        modelValue.value.target = {
          UpgradeWallet: null,
        };
        break;
      case ChangeCanisterTargetType.UpgradeUpgrader:
        modelValue.value.target = {
          UpgradeUpgrader: null,
        };
        break;
      default:
        modelValue.value.target = null;
        break;
    }
  },
  { immediate: true },
);

const updateComputedCanisterModule = async (): Promise<void> => {
  if (modelValue.value.wasmModule && modelValue.value.wasmModule.length > 0) {
    const file = modelValue.value.wasmModule[0];
    const fileBuffer = await readFileAsArrayBuffer(file);
    const hashHex = await arrayBufferToHashHex(fileBuffer);

    moduleChecksum.value = hashHex;
    return;
  }

  moduleChecksum.value = null;
};

const submit = async () => {
  const { valid } = form.value ? await form.value.validate() : { valid: false };

  if (valid) {
    emit('submit', modelValue.value);
  }
};
</script>
