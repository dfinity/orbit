<template>
  <VDialog
    v-bind="$attrs"
    v-model="open"
    :persistent="!canClose"
    transition="dialog-bottom-transition"
    scrollable
    :max-width="props.dialogMaxWidth"
  >
    <VCard data-test-id="canister-call-condition-card">
      <VToolbar color="background">
        <VToolbarTitle>
          {{ dialogTitle }}
        </VToolbarTitle>
        <VBtn :disabled="!canClose" :icon="mdiClose" @click="open = false" />
      </VToolbar>
      <VDivider />

      <CanisterCallConditionForm
        v-model="canisterConfigureMethodCallModel"
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
              {{ $t('terms.save') }}
            </VBtn>
          </VCardActions>
        </template>
      </CanisterCallConditionForm>
    </VCard>
  </VDialog>
</template>
<script lang="ts" setup>
import { Principal } from '@dfinity/principal';
import { mdiClose } from '@mdi/js';
import { computed, Ref, ref, toRefs, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import {
  VBtn,
  VCard,
  VCardActions,
  VDialog,
  VDivider,
  VSpacer,
  VToolbar,
  VToolbarTitle,
} from 'vuetify/components';
import { variantIs } from '~/utils/helper.utils';
import CanisterCallConditionForm from './CanisterCallConditionForm.vue';
import {
  CanisterConfiguredMethodCall,
  CanisterMethodCallConfigurationModel,
} from './external-canisters.types';

const props = withDefaults(
  defineProps<{
    open?: boolean;
    canisterId: Principal;
    method: CanisterConfiguredMethodCall;
    alreadyConfiguredMethods?: CanisterConfiguredMethodCall[];
    dialogMaxWidth?: number;
    title?: string;
  }>(),
  {
    open: false,
    canisterId: undefined,
    methodName: undefined,
    methodConfiguration: () => ({}),
    alreadyConfiguredMethods: () => [],
    dialogMaxWidth: 800,
    title: undefined,
  },
);

const { alreadyConfiguredMethods, canisterId, method } = toRefs(props);

const emit = defineEmits<{
  (event: 'update:open', payload: boolean): void;
}>();

const initialModel = (): CanisterMethodCallConfigurationModel => {
  const model: CanisterMethodCallConfigurationModel = {
    canisterId: canisterId.value,
    methodName: method.value.methodName,
    requestPolicies: method.value.requestPolicies,
    permission: method.value.permission ?? {
      auth_scope: { Restricted: null },
      user_groups: [],
      users: [],
    },
    validationMethodName: variantIs(method.value.validationTarget, 'ValidationMethod')
      ? method.value.validationTarget.ValidationMethod.method_name
      : undefined,
    validationCanisterId: variantIs(method.value.validationTarget, 'ValidationMethod')
      ? method.value.validationTarget.ValidationMethod.canister_id
      : undefined,
    alreadyConfiguredMethods: alreadyConfiguredMethods.value,
  };

  return model;
};

const i18n = useI18n();
const canClose = ref(true);
const dialogTitle = computed(
  () => props.title || i18n.t('external_canisters.call_configuration.config_dialog_title'),
);
const canisterConfigureMethodCallModel = ref(
  initialModel(),
) as Ref<CanisterMethodCallConfigurationModel>;

const open = computed({
  get: () => props.open,
  set: isOpen => {
    emit('update:open', isOpen);
  },
});

watch(
  open,
  isOpen => {
    if (isOpen) {
      canisterConfigureMethodCallModel.value = initialModel();
    }
  },
  { immediate: true },
);
</script>
