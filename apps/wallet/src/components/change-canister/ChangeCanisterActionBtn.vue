<template>
  <ActionBtn
    v-model="upgradeModel"
    :text="$t('app.submit_upgrade')"
    :title="$t('app.submit_upgrade')"
    size="default"
    variant="outlined"
    density="comfortable"
    :submit="form => submitUpgrade(form.modelValue as ChangeCanisterFormProps['modelValue'])"
    data-test-id="submit-upgrade-btn"
    @opened="emit('editing', true)"
    @closed="emit('editing', false)"
    @failed="useOnFailedOperation"
    @submitted="useOnSuccessfulOperation"
  >
    <template #default="{ model: elem, submit }">
      <ChangeCanisterForm
        :model-value="elem.value.modelValue as ChangeCanisterFormProps['modelValue']"
        @update:model-value="elem.value.modelValue = $event"
        @valid="elem.value.valid = $event"
        @submit="submit"
      />
    </template>
    <template #actions="{ submit, loading: saving, model: elem }">
      <VSpacer />
      <VBtn
        :loading="saving"
        :disabled="!elem.value.valid"
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
import { ref } from 'vue';
import ActionBtn from '~/components/buttons/ActionBtn.vue';
import ChangeCanisterForm, {
  ChangeCanisterFormProps,
} from '~/components/change-canister/ChangeCanisterForm.vue';
import {
  useOnFailedOperation,
  useOnSuccessfulOperation,
} from '~/composables/notifications.composable';
import { Proposal } from '~/generated/station/station.did';
import { useStationStore } from '~/stores/station.store';
import { hexStringToArrayBuffer } from '~/utils/crypto.utils';
import { readFileAsArrayBuffer } from '~/utils/file.utils';
import { assertAndReturn } from '~/utils/helper.utils';

const station = useStationStore();

const upgradeModel = ref<ChangeCanisterFormProps>({
  modelValue: {
    target: null,
    wasmModule: undefined,
    arg: null,
  },
  valid: false,
});

const submitUpgrade = async (model: ChangeCanisterFormProps['modelValue']): Promise<Proposal> => {
  const wasmModule = assertAndReturn(model.wasmModule?.[0], 'model.wasmModule is required');
  const fileBuffer = await readFileAsArrayBuffer(wasmModule);

  return station.service.changeCanister({
    arg:
      model.arg && model.arg.length > 0 ? [new Uint8Array(hexStringToArrayBuffer(model.arg))] : [],
    module: new Uint8Array(fileBuffer),
    target: assertAndReturn(model.target),
  });
};

const emit = defineEmits<{
  (event: 'editing', payload: boolean): void;
}>();
</script>
