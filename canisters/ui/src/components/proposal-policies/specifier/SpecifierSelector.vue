<template>
  <VAutocomplete
    v-model="specifier"
    :items="availableSpecifiers"
    :label="$t('terms.specifier')"
    item-value="id"
    item-title="name"
    variant="underlined"
    density="comfortable"
  />

  <component :is="specifierSelector" v-model="model" />
</template>
<script setup lang="ts">
import { computed, ref, toRefs, watch } from 'vue';
import { useAvailableOProposalSpecifiers } from '~/composables/proposal.composable';
import { ProposalSpecifier } from '~/generated/wallet/wallet.did';
import { ProposalSpecifierEnum } from '~/types/wallet.types';
import { KeysOfUnion, unreachable, variantIs } from '~/utils/helper.utils';
import EditAddressBookEntrySpecifier from './EditAddressBookEntrySpecifier.vue';
import RemoveAddressBookEntrySpecifier from './RemoveAddressBookEntrySpecifier.vue';
import type { Component } from 'vue';

const input = withDefaults(
  defineProps<{
    modelValue?: ProposalSpecifier;
  }>(),
  {
    modelValue: undefined,
  },
);

const props = toRefs(input);
const availableSpecifiers = computed(useAvailableOProposalSpecifiers);
const specifier = ref<ProposalSpecifierEnum | null>(null);

const emit = defineEmits<{
  (event: 'update:modelValue', payload?: ProposalSpecifier): void;
}>();

const model = computed({
  get: () => props.modelValue.value,
  set: value => emit('update:modelValue', value),
});

const componentsMap: {
  [key in KeysOfUnion<ProposalSpecifier>]: Component | null;
} = {
  AddUserGroup: null,
  RemoveUserGroup: null,
  EditUserGroup: null,
  AddUser: null,
  EditUser: null,
  AddAccount: null,
  EditAccount: null,
  AddAccessPolicy: null,
  RemoveAccessPolicy: null,
  EditAccessPolicy: null,
  AddProposalPolicy: null,
  EditProposalPolicy: null,
  RemoveProposalPolicy: null,
  Transfer: null,
  ChangeCanister: null,
  AddAddressBookEntry: null,
  EditAddressBookEntry: EditAddressBookEntrySpecifier,
  RemoveAddressBookEntry: RemoveAddressBookEntrySpecifier,
};

const specifierSelector = computed(() => {
  const keys = Object.keys(componentsMap) as KeysOfUnion<ProposalSpecifier>[];
  if (!model.value) {
    return null;
  }

  for (const key of keys) {
    if (key in model.value && key in componentsMap) {
      return componentsMap[key];
    }
  }

  return null;
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
      case ProposalSpecifierEnum.AddAccessPolicy:
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
      case ProposalSpecifierEnum.RemoveAccessPolicy:
        model.value = { [specifier.value]: { Any: null } };
        break;
      case ProposalSpecifierEnum.RemoveProposalPolicy:
        model.value = { [specifier.value]: { Any: null } };
        break;
      case ProposalSpecifierEnum.RemoveAddressBookEntry:
        model.value = { [specifier.value]: { Any: null } };
        break;
      case ProposalSpecifierEnum.Transfer:
        model.value = { [specifier.value]: { account: { Any: null } } };
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
