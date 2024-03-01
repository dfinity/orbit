<template>
  <VForm ref="form" @submit.prevent="submit">
    <VTextField
      v-if="model.id && props.display.value.id"
      v-model="model.id"
      name="id"
      :label="$t('terms.id')"
      variant="plain"
      density="compact"
      :disabled="isViewMode"
    />
    <VTextField
      v-model="model.name"
      name="name"
      :label="$t('terms.name')"
      :rules="[requiredRule]"
      variant="underlined"
      class="mb-2"
      density="comfortable"
      :prepend-icon="mdiWallet"
      :disabled="isViewMode"
    />
    <TokenAutocomplete
      v-if="props.display.value.asset"
      v-model="model.symbol"
      class="mb-2"
      :label="$t('terms.asset')"
      :prepend-icon="mdiKeyChainVariant"
      :rules="[requiredRule]"
      variant="underlined"
      density="comfortable"
      :disabled="isViewMode"
      @selected-asset="onSelectedAsset"
    />
    <UserAutocomplete
      v-model="model.owners"
      :label="$t('terms.owners')"
      variant="underlined"
      class="mb-2"
      density="comfortable"
      :disabled="isViewMode"
      :rules="[requiredRule]"
      multiple
      :prepend-icon="mdiAccountGroup"
    />
    <VTabs v-model="tab" grow bg-color="background" density="comfortable">
      <VTab value="edit">{{ $t('terms.edit') }}</VTab>
      <VTab value="transfers">{{ $t('terms.transfers') }}</VTab>
    </VTabs>
    <VWindow v-model="tab">
      <VWindowItem value="edit">
        <div class="d-flex flex-column ga-2">
          <small class="pt-2">{{ $t('app.account_dialog_edit_criteria_hint') }}</small>
          <CriteriaBuilder
            v-model="editPolicy"
            :specifier="{ EditAccount: { Any: null } }"
            :disabled="isViewMode"
            @remove="editPolicy = undefined"
          />
        </div>
      </VWindowItem>
      <VWindowItem value="transfers">
        <div class="d-flex flex-column ga-2">
          <small class="pt-2">{{ $t('app.account_dialog_transfers_criteria_hint') }}</small>
          <CriteriaBuilder
            v-model="transferPolicy"
            :specifier="{ Transfer: { account: { Any: null } } }"
            :disabled="isViewMode"
            @remove="transferPolicy = undefined"
          />
        </div>
      </VWindowItem>
    </VWindow>
  </VForm>
</template>

<script lang="ts" setup>
import { mdiAccountGroup, mdiKeyChainVariant, mdiWallet } from '@mdi/js';
import { computed, onMounted, ref, toRefs, watch } from 'vue';
import TokenAutocomplete from '~/components/inputs/TokenAutocomplete.vue';
import UserAutocomplete from '~/components/inputs/UserAutocomplete.vue';
import CriteriaBuilder from '~/components/proposal-policies/criteria/CriteriaBuilder.vue';
import { Account, WalletAsset } from '~/generated/wallet/wallet.did';
import { useWalletStore } from '~/stores/wallet.store';
import { VFormValidation } from '~/types/helper.types';
import { requiredRule } from '~/utils/form.utils';

export type AccountConfigFormProps = {
  modelValue: Partial<Account>;
  triggerSubmit?: boolean;
  valid?: boolean;
  mode: 'view' | 'edit';
  display?: {
    id?: boolean;
    asset?: boolean;
  };
};

const form = ref<VFormValidation | null>(null);

const input = withDefaults(defineProps<AccountConfigFormProps>(), {
  valid: true,
  display: () => ({
    id: true,
    asset: true,
  }),
  mode: 'edit',
  triggerSubmit: false,
});
const props = toRefs(input);

const emit = defineEmits<{
  (event: 'update:modelValue', payload: AccountConfigFormProps['modelValue']): void;
  (event: 'update:triggerSubmit', payload: boolean): void;
  (event: 'valid', payload: boolean): void;
  (event: 'submit', payload: AccountConfigFormProps['modelValue']): void;
}>();

const isViewMode = computed(() => props.mode.value === 'view');

const model = computed(() => props.modelValue.value);
watch(model.value, newValue => emit('update:modelValue', newValue), { deep: true });

const editPolicy = computed({
  get: () => props.modelValue.value.policies?.edit?.[0],
  set: value => {
    emit('update:modelValue', {
      ...props.modelValue.value,
      policies: {
        edit: value ? [value] : [],
        transfer: props.modelValue.value.policies?.transfer?.[0]
          ? [props.modelValue.value.policies?.transfer?.[0]]
          : [],
      },
    });
  },
});

const transferPolicy = computed({
  get: () => props.modelValue.value.policies?.transfer?.[0],
  set: value => {
    emit('update:modelValue', {
      ...props.modelValue.value,
      policies: {
        transfer: value ? [value] : [],
        edit: props.modelValue.value.policies?.edit?.[0]
          ? [props.modelValue.value.policies?.edit?.[0]]
          : [],
      },
    });
  },
});

const wallet = useWalletStore();
const tab = ref<'edit' | 'transfers'>('edit');

const onSelectedAsset = (asset?: WalletAsset): void => {
  if (asset) {
    model.value.symbol = asset.symbol;
    model.value.blockchain = asset.blockchain;
    model.value.standard = asset.standard;
  } else {
    model.value.symbol = undefined;
    model.value.blockchain = undefined;
    model.value.standard = undefined;
  }
};

onMounted(() => {
  if (wallet.configuration.details.supported_assets.length === 1 && !model.value.symbol) {
    onSelectedAsset(wallet.configuration.details.supported_assets[0]);
  }

  if (!model.value.metadata) {
    model.value.metadata = [];
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

  if (valid) {
    emit('submit', model.value);
  }
};
</script>
