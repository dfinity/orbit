<template>
  <ActionBtn
    v-model="upgradeModel"
    :text="btnText"
    :title="$t('app.software_update')"
    size="default"
    :variant="!isMobileHighlight ? 'outlined' : 'text'"
    density="comfortable"
    :icon="isMobileHighlight ? mdiCloudDownload : false"
    :append-icon="isHighlightedAction && !isMobileHighlight ? mdiCloudDownload : undefined"
    :color="isHighlightedAction ? 'warning' : undefined"
    :rounded="isHighlightedAction ? true : undefined"
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
        @loading="formLoading = $event"
        @submit="goToConfirmation(elem.value.modelValue)"
      />

      <ChangeCanisterConfirmationScreen
        v-if="screen === ChangeCanisterScreen.Confirm"
        :wasm-module-checksum="wasmChecksum"
        :comment="elem.value.modelValue.comment"
        @update:comment="
          elem.value.modelValue = {
            ...elem.value.modelValue,
            comment: $event,
          }
        "
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
      <div class="d-flex align-md-center justify-end flex-column-reverse flex-md-row ga-2">
        <VBtn
          v-if="
            screen === ChangeCanisterScreen.Form && formMode === ChangeCanisterFormMode.Registry
          "
          :disabled="station.versionManagement.loading || formLoading"
          color="primary"
          variant="text"
          :append-icon="mdiRefresh"
          size="small"
          @click="station.checkVersionUpdates"
        >
          {{ $t('app.check_updates_btn') }}
        </VBtn>
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
          v-else-if="screen === ChangeCanisterScreen.Confirm"
          :loading="saving"
          :disabled="saving"
          color="primary"
          variant="flat"
          @click="submit"
        >
          {{ $t('terms.submit') }}
        </VBtn>
      </div>
    </template>
  </ActionBtn>
</template>

<script lang="ts" setup>
import { mdiCloudDownload, mdiRefresh, mdiWrenchCog } from '@mdi/js';
import { computed, ref } from 'vue';
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
import { useAppStore } from '~/stores/app.store';
import { useI18n } from 'vue-i18n';

const props = withDefaults(
  defineProps<{
    mode?: 'default' | 'highlight';
  }>(),
  {
    mode: 'default',
  },
);

const i18n = useI18n();
const app = useAppStore();
const isHighlightedAction = computed(() => props.mode === 'highlight');
const isMobileHighlight = computed(() => isHighlightedAction.value && app.isMobile);
const btnText = computed(() => {
  if (isMobileHighlight.value) {
    return undefined;
  }

  return isHighlightedAction.value ? i18n.t('terms.update') : i18n.t('app.software_update');
});

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
const formLoading = ref(false);
const goToConfirmation = async (model: ChangeCanisterFormProps['modelValue']): Promise<void> => {
  const wasmModule = assertAndReturn(model.wasmModule, 'model.wasmModule is required');
  wasmChecksum.value = await arrayBufferToHashHex(wasmModule);

  screen.value = ChangeCanisterScreen.Confirm;
};

const submitUpgrade = async (model: ChangeCanisterFormProps['modelValue']): Promise<Request> => {
  const fileBuffer = assertAndReturn(model.wasmModule, 'model.wasmModule is required');
  const res = await station.service.changeCanister(
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

  // Refresh the version update status after the upgrade
  station.checkVersionUpdates();

  return res;
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
