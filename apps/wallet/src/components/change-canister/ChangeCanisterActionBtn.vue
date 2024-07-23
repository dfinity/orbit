<template>
  <ActionBtn
    v-model="upgradeModel"
    :text="$t('app.software_update')"
    :title="$t('app.software_update')"
    size="default"
    variant="outlined"
    density="comfortable"
    :submit="form => submitUpgrade(form.modelValue as ChangeCanisterFormProps['modelValue'])"
    data-test-id="submit-upgrade-btn"
    @opened="emit('editing', true)"
    @closed="onClosed"
    @failed="useOnFailedOperation"
    @submitted="useOnSuccessfulOperation"
  >
    <template #default="{ model: elem }">
      <ChangeCanisterForm
        v-show="screen === ChangeCanisterScreen.Form"
        :mode="formMode"
        :model-value="elem.value.modelValue as ChangeCanisterFormProps['modelValue']"
        @update:model-value="elem.value.modelValue = $event"
        @valid="elem.value.valid = $event"
        @submit="goToConfirmation(elem.value.modelValue)"
      />

      <ChangeCanisterConfirmationScreen
        v-if="screen === ChangeCanisterScreen.Confirm"
        :wasm-module-checksum="wasmChecksum"
        :comment="elem.value.modelValue.comment"
        @update:comment="elem.value.modelValue = {
          ...elem.value.modelValue,
          comment: $event,
        }"
      />
    </template>
    <template #actions="{ submit, loading: saving, model: elem }">
      <VBtn
        v-if="screen === ChangeCanisterScreen.Form"
        :disabled="saving"
        :append-icon="
          formMode === ChangeCanisterFormMode.Advanced ? mdiCloudDownload : mdiWrenchCog
        "
        variant="text"
        @click="toggleFormMode"
      >
        {{
          formMode === ChangeCanisterFormMode.Advanced
            ? $t('terms.automated')
            : $t('terms.advanced')
        }}
      </VBtn>
      <VSpacer />
      <VBtn
        v-if="screen === ChangeCanisterScreen.Form"
        :loading="saving"
        :disabled="!elem.value.valid"
        color="primary"
        variant="flat"
        @click="goToConfirmation(elem.value.modelValue)"
      >
        {{ $t('terms.continue') }}
      </VBtn>
      <VBtn
        v-if="screen === ChangeCanisterScreen.Confirm"
        :loading="saving"
        :disabled="saving"
        color="primary"
        variant="flat"
        @click="submit"
      >
        {{ $t('terms.submit') }}
      </VBtn>
    </template>
  </ActionBtn>
</template>

<script lang="ts" setup>
import { mdiCloudDownload, mdiWrenchCog } from '@mdi/js';
import { ref } from 'vue';
import { VBtn } from 'vuetify/components';
import ActionBtn from '~/components/buttons/ActionBtn.vue';
import ChangeCanisterForm, {
  ChangeCanisterFormProps,
} from '~/components/change-canister/ChangeCanisterForm.vue';
import { useDefaultUpgradeModel } from '~/composables/change-canister.composable';
import {
  useOnFailedOperation,
  useOnSuccessfulOperation,
} from '~/composables/notifications.composable';
import { Request } from '~/generated/station/station.did';
import { useStationStore } from '~/stores/station.store';
import { arrayBufferToHashHex, hexStringToArrayBuffer } from '~/utils/crypto.utils';
import { assertAndReturn } from '~/utils/helper.utils';
import { ChangeCanisterFormMode, ChangeCanisterScreen } from './change-canister.types';
import ChangeCanisterConfirmationScreen from './ChangeCanisterConfirmationScreen.vue';

const station = useStationStore();
const upgradeModel = ref<ChangeCanisterFormProps>(useDefaultUpgradeModel());
const screen = ref<ChangeCanisterScreen>(ChangeCanisterScreen.Form);
const formMode = ref<ChangeCanisterFormMode>(ChangeCanisterFormMode.Registry);
const toggleFormMode = () => {
  upgradeModel.value = useDefaultUpgradeModel();
  formMode.value =
    formMode.value === ChangeCanisterFormMode.Advanced
      ? ChangeCanisterFormMode.Registry
      : ChangeCanisterFormMode.Advanced;
};
const wasmChecksum = ref<string>('');

const goToConfirmation = async (model: ChangeCanisterFormProps['modelValue']): Promise<void> => {
  const wasmModule = assertAndReturn(model.wasmModule, 'model.wasmModule is required');
  wasmChecksum.value = await arrayBufferToHashHex(wasmModule);

  screen.value = ChangeCanisterScreen.Confirm;
};

const submitUpgrade = async (model: ChangeCanisterFormProps['modelValue']): Promise<Request> => {
  const fileBuffer = assertAndReturn(model.wasmModule, 'model.wasmModule is required');

  return station.service.changeCanister(
    {
      arg:
        model.wasmInitArg && model.wasmInitArg.length > 0
          ? [new Uint8Array(hexStringToArrayBuffer(model.wasmInitArg))]
          : [],
      module: new Uint8Array(fileBuffer),
      target: assertAndReturn(model.target, 'model.target is required'),
    },
    {
      comment: model.comment,
    },
  );
};

const emit = defineEmits<{
  (event: 'editing', payload: boolean): void;
}>();

const onClosed = () => {
  formMode.value = ChangeCanisterFormMode.Registry;
  screen.value = ChangeCanisterScreen.Form;
  upgradeModel.value = useDefaultUpgradeModel();

  emit('editing', false);
};
</script>
