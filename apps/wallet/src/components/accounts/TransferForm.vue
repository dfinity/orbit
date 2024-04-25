<template>
  <VForm ref="form" @submit.prevent="submit">
    <VTextField
      v-if="model.id && props.display.value.id"
      v-model="model.id"
      name="id"
      :label="$t('terms.id')"
      variant="plain"
      density="comfortable"
      :readonly="isViewMode"
      data-test-id="transfer-form-transfer-id"
    />
    <VTextField
      v-model="model.to"
      :label="$t('terms.destination_address')"
      :variant="isViewMode ? 'plain' : 'filled'"
      density="comfortable"
      class="mb-2"
      name="to"
      :readonly="isViewMode"
      type="text"
      :prepend-icon="mdiSend"
      :rules="[requiredRule]"
      data-test-id="transfer-form-destination-address"
    />
    <VTextField
      ref="amountInput"
      v-model="amount"
      :label="$t('terms.amount')"
      :variant="isViewMode ? 'plain' : 'filled'"
      density="comfortable"
      name="amount"
      class="mb-2"
      type="number"
      :readonly="isViewMode"
      :prepend-icon="mdiNumeric"
      :rules="[requiredRule, (v: unknown) => validTokenAmount(v, account.decimals)]"
      data-test-id="transfer-form-amount"
    />
  </VForm>
</template>

<script lang="ts" setup>
import { mdiNumeric, mdiSend } from '@mdi/js';
import { onUnmounted } from 'vue';
import { onMounted } from 'vue';
import { computed, ref, toRefs, watch } from 'vue';
import { VForm, VTextField } from 'vuetify/components';
import { Account, Transfer } from '~/generated/station/station.did';
import { VFormValidation } from '~/types/helper.types';
import { requiredRule, validTokenAmount } from '~/utils/form.utils';
import { amountToBigInt, formatBalance } from '~/utils/helper.utils';

export type TransferFormProps = {
  account: Account;
  modelValue: Partial<Transfer>;
  triggerSubmit?: boolean;
  valid?: boolean;
  mode?: 'view' | 'edit';
  display?: {
    id?: boolean;
  };
};

const form = ref<VFormValidation | null>(null);

const input = withDefaults(defineProps<TransferFormProps>(), {
  valid: true,
  display: () => ({
    id: true,
  }),
  mode: 'edit',
  triggerSubmit: false,
});
const props = toRefs(input);

const isViewMode = computed(() => props.mode.value === 'view');

const emit = defineEmits<{
  (event: 'update:modelValue', payload: TransferFormProps['modelValue']): void;
  (event: 'update:triggerSubmit', payload: boolean): void;
  (event: 'valid', payload: boolean): void;
  (event: 'submit', payload: TransferFormProps['modelValue']): void;
}>();

const model = computed(() => props.modelValue.value);
watch(model.value, newValue => emit('update:modelValue', newValue), { deep: true });

const amountInput = ref<HTMLInputElement | null>(null);
const amount = ref<string | undefined>(undefined);
watch(
  () => model.value.amount,
  newValue => {
    amount.value =
      newValue !== undefined ? formatBalance(newValue, props.account.value.decimals) : undefined;
  },
  { immediate: true },
);

const syncAmountInput = (): void => {
  if (
    amount.value !== undefined &&
    validTokenAmount(amount.value, props.account.value.decimals) === true
  ) {
    model.value.amount = amountToBigInt(amount.value, props.account.value.decimals);
  } else {
    model.value.amount = undefined;
  }
};

onMounted(() => {
  if (amountInput.value) {
    amountInput.value.addEventListener('blur', syncAmountInput);
  }
});

onUnmounted(() => {
  if (amountInput.value) {
    amountInput.value.removeEventListener('blur', syncAmountInput);
  }
});

const isFormValid = computed(() => (form.value ? form.value.isValid : false));
watch(
  () => isFormValid.value,
  isValid => emit('valid', isValid ?? false),
);

watch(
  () => props.triggerSubmit.value,
  () => {
    if (props.triggerSubmit.value) {
      emit('update:triggerSubmit', false);
      submit();
    }
  },
);

const submit = async () => {
  const { valid } = form.value ? await form.value.validate() : { valid: false };

  syncAmountInput();
  if (valid) {
    emit('submit', model.value);
  }
};
</script>
