<template>
  <VAutocomplete
    v-if="!props.readonly.value"
    v-model="specifier"
    :items="availableSpecifiers"
    :label="$t('terms.specifier')"
    item-value="value"
    item-title="text"
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
import { useAvailableORequestSpecifiers } from '~/composables/request.composable';
import { RequestSpecifier } from '~/generated/station/station.did';
import { RequestSpecifierEnum } from '~/types/station.types';
import { KeysOfUnion, unreachable, variantIs } from '~/utils/helper.utils';
import AccountSpecifier from './AccountSpecifier.vue';
import AddressBookEntrySpecifier from './AddressBookEntrySpecifier.vue';
import TransferSpecifier from './TransferSpecifier.vue';
import UserGroupSpecifier from './UserGroupSpecifier.vue';
import UserSpecifier from './UserSpecifier.vue';
import UnsupportedSpecifier from './UnsupportedSpecifier.vue';
import { VAutocomplete } from 'vuetify/components';

const input = withDefaults(
  defineProps<{
    modelValue?: RequestSpecifier;
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
const availableSpecifiers = computed(useAvailableORequestSpecifiers);
const specifier = ref<RequestSpecifierEnum | null>(null);

const emit = defineEmits<{
  (event: 'update:modelValue', payload?: RequestSpecifier): void;
  (event: 'changedVariant', payload: void): void;
}>();

const model = computed({
  get: () => props.modelValue.value,
  set: value => emit('update:modelValue', value),
});

const componentsMap: {
  [key in KeysOfUnion<RequestSpecifier>]: Component | null;
} = {
  AddUser: null,
  AddUserGroup: null,
  AddAccount: null,
  AddRequestPolicy: null,
  AddAddressBookEntry: null,
  ChangeCanister: null,
  ManageSystemInfo: null,
  // below variants require more specific specifier model
  Transfer: TransferSpecifier,
  EditAccount: AccountSpecifier,
  EditUserGroup: UserGroupSpecifier,
  RemoveUserGroup: UserGroupSpecifier,
  EditUser: UserSpecifier,
  EditAddressBookEntry: AddressBookEntrySpecifier,
  RemoveAddressBookEntry: AddressBookEntrySpecifier,
  // below variants are not supported yet
  EditPermission: UnsupportedSpecifier,
  EditRequestPolicy: UnsupportedSpecifier,
  RemoveRequestPolicy: UnsupportedSpecifier,
  ChangeExternalCanister: UnsupportedSpecifier,
  CreateExternalCanister: UnsupportedSpecifier,
  CallExternalCanister: UnsupportedSpecifier,
  SetDisasterRecovery: UnsupportedSpecifier,
  FundExternalCanister: UnsupportedSpecifier,
};

function isKeyOfRequestSpecifier(key: string): key is keyof RequestSpecifier {
  return key in componentsMap;
}

const selectedSpecifier = computed<{
  component: Component;
  model: RequestSpecifier[keyof RequestSpecifier];
} | null>(() => {
  const keys = Object.keys(componentsMap) as Array<keyof RequestSpecifier>;
  if (!model.value) {
    return null;
  }

  for (const key of keys) {
    if (key in model.value && isKeyOfRequestSpecifier(key)) {
      return {
        component: componentsMap[key],
        model: model.value[key],
      };
    }
  }

  return null;
});

const updateSpecifierModelValue = (updated: RequestSpecifier[keyof RequestSpecifier]): void => {
  if (!updated) {
    model.value = undefined;
    return;
  }

  if (!model.value) {
    return;
  }

  for (const key of Object.values(RequestSpecifierEnum)) {
    if (isKeyOfRequestSpecifier(key) && key in model.value) {
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

    for (const key of Object.values(RequestSpecifierEnum)) {
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

    if (model.value && specifier.value && variantIs(model.value, specifier.value)) {
      return;
    }

    emit('changedVariant');

    switch (specifier.value) {
      case RequestSpecifierEnum.AddUserGroup:
        model.value = { [specifier.value]: null };
        break;
      case RequestSpecifierEnum.AddAccount:
        model.value = { [specifier.value]: null };
        break;
      case RequestSpecifierEnum.AddAddressBookEntry:
        model.value = { [specifier.value]: null };
        break;
      case RequestSpecifierEnum.AddRequestPolicy:
        model.value = { [specifier.value]: null };
        break;
      case RequestSpecifierEnum.EditUserGroup:
        model.value = { [specifier.value]: { Any: null } };
        break;
      case RequestSpecifierEnum.EditAccount:
        model.value = { [specifier.value]: { Any: null } };
        break;
      case RequestSpecifierEnum.EditAddressBookEntry:
        model.value = { [specifier.value]: { Any: null } };
        break;
      case RequestSpecifierEnum.EditPermission:
        model.value = { [specifier.value]: { Any: null } };
        break;
      case RequestSpecifierEnum.EditRequestPolicy:
        model.value = { [specifier.value]: { Any: null } };
        break;
      case RequestSpecifierEnum.RemoveUserGroup:
        model.value = { [specifier.value]: { Any: null } };
        break;
      case RequestSpecifierEnum.RemoveRequestPolicy:
        model.value = { [specifier.value]: { Any: null } };
        break;
      case RequestSpecifierEnum.RemoveAddressBookEntry:
        model.value = { [specifier.value]: { Any: null } };
        break;
      case RequestSpecifierEnum.Transfer:
        model.value = { [specifier.value]: { Any: null } };
        break;
      case RequestSpecifierEnum.ChangeCanister:
        model.value = { [specifier.value]: null };
        break;
      case RequestSpecifierEnum.EditUser:
        model.value = { [specifier.value]: { Any: null } };
        break;
      case RequestSpecifierEnum.AddUser:
        model.value = { [specifier.value]: null };
        break;
      case RequestSpecifierEnum.ManageSystemInfo:
        model.value = { [specifier.value]: null };
        break;
      case RequestSpecifierEnum.ChangeExternalCanister:
        model.value = { [specifier.value]: { Any: null } };
        break;
      case RequestSpecifierEnum.CreateExternalCanister:
        model.value = { [specifier.value]: null };
        break;
      case RequestSpecifierEnum.FundExternalCanister:
        model.value = { [specifier.value]: { Any: null } };
        break;
      case RequestSpecifierEnum.CallExternalCanister:
        model.value = {
          [specifier.value]: { validation_method: { No: null }, execution_method: { Any: null } },
        };
        break;
      case RequestSpecifierEnum.SetDisasterRecovery:
        model.value = { [specifier.value]: null };
        break;
      default:
        unreachable(specifier.value);
    }
  },
);
</script>
