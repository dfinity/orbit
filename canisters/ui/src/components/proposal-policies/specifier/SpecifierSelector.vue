<template>
  <VAutocomplete
    v-if="!props.readonly.value"
    v-model="specifier"
    :items="availableSpecifiers"
    :label="$t('terms.specifier')"
    item-value="value"
    item-title="text"
    variant="underlined"
    density="comfortable"
    :disabled="props.disabled.value"
  />

  <div v-else class="py-2">
    {{ specifierText }}
  </div>

  <component
    :is="selectedSpecifier?.component"
    v-if="selectedSpecifier"
    :model-value="selectedSpecifier.model"
    :disabled="props.disabled.value"
    :readonly="props.readonly.value"
    @update:model-value="updateSpecifierModelValue"
  />
</template>
<script setup lang="ts">
import type { Component } from 'vue';
import { computed, ref, toRefs, watch } from 'vue';
import { useAvailableOProposalSpecifiers } from '~/composables/proposal.composable';
import { ProposalSpecifier } from '~/generated/wallet/wallet.did';
import { ProposalSpecifierEnum } from '~/types/wallet.types';
import { KeysOfUnion, unreachable, variantIs } from '~/utils/helper.utils';
import AccountSpecifier from './AccountSpecifier.vue';
import AddressBookEntrySpecifier from './AddressBookEntrySpecifier.vue';
import TransferSpecifier from './TransferSpecifier.vue';
import UserGroupSpecifier from './UserGroupSpecifier.vue';
import UserSpecifier from './UserSpecifier.vue';
import UnsupportedSpecifier from './UnsupportedSpecifier.vue';

const input = withDefaults(
  defineProps<{
    modelValue?: ProposalSpecifier;
    disabled?: boolean;
    readonly?: boolean;
  }>(),
  {
    modelValue: undefined,
    disabled: false,
    readonly: false,
  },
);

const props = toRefs(input);
const availableSpecifiers = computed(useAvailableOProposalSpecifiers);
const specifier = ref<ProposalSpecifierEnum | null>(null);

const emit = defineEmits<{
  (event: 'update:modelValue', payload?: ProposalSpecifier): void;
  (event: 'changedVariant', payload: void): void;
}>();

const model = computed({
  get: () => props.modelValue.value,
  set: value => emit('update:modelValue', value),
});

const componentsMap: {
  [key in KeysOfUnion<ProposalSpecifier>]: Component | null;
} = {
  AddUser: null,
  AddUserGroup: null,
  AddAccount: null,
  AddProposalPolicy: null,
  AddAddressBookEntry: null,
  ChangeCanister: null,
  // below variants require more specific specifier model
  EditAccessPolicy: UnsupportedSpecifier,
  EditProposalPolicy: UnsupportedSpecifier,
  RemoveProposalPolicy: UnsupportedSpecifier,
  Transfer: TransferSpecifier,
  EditAccount: AccountSpecifier,
  EditUserGroup: UserGroupSpecifier,
  RemoveUserGroup: UserGroupSpecifier,
  EditUser: UserSpecifier,
  EditAddressBookEntry: AddressBookEntrySpecifier,
  RemoveAddressBookEntry: AddressBookEntrySpecifier,
};

function isKeyOfProposalSpecifier(key: string): key is keyof ProposalSpecifier {
  return key in componentsMap;
}

const selectedSpecifier = computed<{
  component: Component;
  model: ProposalSpecifier[keyof ProposalSpecifier];
} | null>(() => {
  const keys = Object.keys(componentsMap) as Array<keyof ProposalSpecifier>;
  if (!model.value) {
    return null;
  }

  for (const key of keys) {
    if (key in model.value && isKeyOfProposalSpecifier(key)) {
      return {
        component: componentsMap[key],
        model: model.value[key],
      };
    }
  }

  return null;
});

const updateSpecifierModelValue = (updated: ProposalSpecifier[keyof ProposalSpecifier]): void => {
  if (!updated) {
    model.value = undefined;
    return;
  }

  if (!model.value) {
    return;
  }

  for (const key of Object.values(ProposalSpecifierEnum)) {
    if (isKeyOfProposalSpecifier(key) && key in model.value) {
      model.value[key] = updated;
      return;
    }
  }
};

const specifierText = computed(() => {
  if (!specifier.value) {
    return '-';
  }

  return availableSpecifiers.value.find(s => s.value === specifier.value)?.text || '-';
});

watch(
  () => model.value,
  () => {
    if (!model.value) {
      specifier.value = null;
      return;
    }

    for (const key of Object.values(ProposalSpecifierEnum)) {
      if (key in model.value) {
        specifier.value = key;
        return;
      }
    }

    specifier.value = null;
  },
  { immediate: true },
);

watch(
  () => specifier.value,
  () => {
    if (!specifier.value) {
      return;
    }

    if (model.value && variantIs(model.value, specifier.value)) {
      return;
    }

    emit('changedVariant');

    switch (specifier.value) {
      case ProposalSpecifierEnum.AddUserGroup:
        model.value = { [specifier.value]: null };
        break;
      case ProposalSpecifierEnum.AddAccount:
        model.value = { [specifier.value]: null };
        break;
      case ProposalSpecifierEnum.AddAddressBookEntry:
        model.value = { [specifier.value]: null };
        break;
      case ProposalSpecifierEnum.AddProposalPolicy:
        model.value = { [specifier.value]: null };
        break;
      case ProposalSpecifierEnum.EditUserGroup:
        model.value = { [specifier.value]: { Any: null } };
        break;
      case ProposalSpecifierEnum.EditAccount:
        model.value = { [specifier.value]: { Any: null } };
        break;
      case ProposalSpecifierEnum.EditAddressBookEntry:
        model.value = { [specifier.value]: { Any: null } };
        break;
      case ProposalSpecifierEnum.EditAccessPolicy:
        model.value = { [specifier.value]: { Any: null } };
        break;
      case ProposalSpecifierEnum.EditProposalPolicy:
        model.value = { [specifier.value]: { Any: null } };
        break;
      case ProposalSpecifierEnum.RemoveUserGroup:
        model.value = { [specifier.value]: { Any: null } };
        break;
      case ProposalSpecifierEnum.RemoveProposalPolicy:
        model.value = { [specifier.value]: { Any: null } };
        break;
      case ProposalSpecifierEnum.RemoveAddressBookEntry:
        model.value = { [specifier.value]: { Any: null } };
        break;
      case ProposalSpecifierEnum.Transfer:
        model.value = { [specifier.value]: { Any: null } };
        break;
      case ProposalSpecifierEnum.ChangeCanister:
        model.value = { [specifier.value]: null };
        break;
      case ProposalSpecifierEnum.EditUser:
        model.value = { [specifier.value]: { Any: null } };
        break;
      case ProposalSpecifierEnum.AddUser:
        model.value = { [specifier.value]: null };
        break;
      default:
        unreachable(specifier.value);
    }
  },
);
</script>
