<template>
  <VDialog
    v-bind="$attrs"
    v-model="open"
    :persistent="!canClose"
    transition="dialog-bottom-transition"
    scrollable
    :max-width="props.dialogMaxWidth"
  >
    <VCard data-test-id="canister-call-card">
      <VToolbar color="background">
        <VToolbarTitle>
          {{ dialogTitle }}
        </VToolbarTitle>
        <VBtn :disabled="!canClose" :icon="mdiClose" @click="open = false" />
      </VToolbar>
      <VDivider />

      <CanisterCallForm
        v-model="canisterCallModel"
        :hide="{ canisterId: !!canisterCallModel.canisterId }"
        :allowed-methods="props.allowedMethods"
        :allow-any-method="props.allowAnyMethod"
        @submitting="canClose = !$event"
        @submitted="open = false"
      >
        <template #actions="{ valid, submitting, submit, edited }">
          <VCardActions class="pa-3">
            <VSpacer />
            <VBtn
              :disabled="!valid || !edited"
              :loading="submitting"
              color="primary"
              variant="elevated"
              data-test-id="submit-btn"
              @click="submit"
            >
              {{ $t('terms.execute') }}
            </VBtn>
          </VCardActions>
        </template>
      </CanisterCallForm>
    </VCard>
  </VDialog>
</template>
<script lang="ts" setup>
import { Principal } from '@dfinity/principal';
import { mdiClose } from '@mdi/js';
import { Ref, computed, ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import { VBtn, VCard, VDialog, VDivider, VToolbar, VToolbarTitle } from 'vuetify/components';
import { CanisterCallModel, CanisterAllowedMethod } from './external-canisters.types';
import CanisterCallForm from './CanisterCallForm.vue';

const props = withDefaults(
  defineProps<{
    open?: boolean;
    canisterId: Principal;
    allowedMethods?: CanisterAllowedMethod[];
    allowAnyMethod?: boolean;
    dialogMaxWidth?: number;
    title?: string;
  }>(),
  {
    open: false,
    allowedMethods: () => [],
    allowAnyMethod: false,
    dialogMaxWidth: 800,
    title: undefined,
  },
);

const emit = defineEmits<{
  (event: 'update:open', payload: boolean): void;
}>();

const i18n = useI18n();
const canClose = ref(true);
const dialogTitle = computed(() => props.title || i18n.t('external_canisters.perform_call.title'));

const initialModel = (): CanisterCallModel => {
  const model: CanisterCallModel = {
    canisterId: Principal.fromUint8Array(props.canisterId.toUint8Array()),
  };

  return model;
};

const canisterCallModel = ref(initialModel()) as Ref<CanisterCallModel>;

const open = computed({
  get: () => props.open,
  set: isOpen => emit('update:open', isOpen),
});

watch(
  open,
  isOpen => {
    if (isOpen) {
      canisterCallModel.value = initialModel();
    }
  },
  { immediate: true },
);
</script>
