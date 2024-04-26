<template>
  <template v-if="variantIs(model, 'Permission')">
    <VTextField
      :model-value="$t('permissions.resources.permission')"
      :label="$t('terms.resource')"
      variant="plain"
      density="compact"
      disabled
    />

    <VTextField
      :model-value="toPermissionResourceActionTest(model.Permission)"
      :label="$t('terms.action')"
      variant="plain"
      density="compact"
      disabled
    />
  </template>
  <template v-else-if="variantIs(model, 'Account')">
    <VTextField
      :model-value="$t('permissions.resources.account')"
      :label="$t('terms.resource')"
      variant="plain"
      density="compact"
      disabled
    />

    <VTextField
      :model-value="toAccountResourceActionText(model.Account)"
      :label="$t('terms.action')"
      variant="plain"
      density="compact"
      disabled
    />
  </template>
  <template v-else-if="variantIs(model, 'AddressBook')">
    <VTextField
      :model-value="$t('permissions.resources.addressbook')"
      :label="$t('terms.resource')"
      variant="plain"
      density="compact"
      disabled
    />

    <VTextField
      :model-value="toResourceActionText(model.AddressBook)"
      :label="$t('terms.action')"
      variant="plain"
      density="compact"
      disabled
    />
  </template>
  <template v-else-if="variantIs(model, 'ProposalPolicy')">
    <VTextField
      :model-value="$t('permissions.resources.proposalpolicy')"
      :label="$t('terms.resource')"
      variant="plain"
      density="compact"
      disabled
    />

    <VTextField
      :model-value="toResourceActionText(model.ProposalPolicy)"
      :label="$t('terms.action')"
      variant="plain"
      density="compact"
      disabled
    />
  </template>
  <template v-else-if="variantIs(model, 'User')">
    <VTextField
      :model-value="$t('permissions.resources.user')"
      :label="$t('terms.resource')"
      variant="plain"
      density="compact"
      disabled
    />

    <VTextField
      :model-value="toResourceActionText(model.User)"
      :label="$t('terms.action')"
      variant="plain"
      density="compact"
      disabled
    />
  </template>
  <template v-else-if="variantIs(model, 'UserGroup')">
    <VTextField
      :model-value="$t('permissions.resources.usergroup')"
      :label="$t('terms.resource')"
      variant="plain"
      density="compact"
      disabled
    />

    <VTextField
      :model-value="toResourceActionText(model.UserGroup)"
      :label="$t('terms.action')"
      variant="plain"
      density="compact"
      disabled
    />
  </template>
  <template v-else-if="variantIs(model, 'Proposal')">
    <VTextField
      :model-value="$t('permissions.resources.proposal')"
      :label="$t('terms.resource')"
      variant="plain"
      density="compact"
      disabled
    />

    <VTextField
      :model-value="toResourceActionText(model.Proposal)"
      :label="$t('terms.action')"
      variant="plain"
      density="compact"
      disabled
    />
  </template>
  <template v-else-if="variantIs(model, 'System')">
    <VTextField
      :model-value="$t('permissions.resources.system')"
      :label="$t('terms.resource')"
      variant="plain"
      density="compact"
      disabled
    />

    <VTextField
      :model-value="toSystemResourceActionText(model.System)"
      :label="$t('terms.action')"
      variant="plain"
      density="compact"
      disabled
    />
  </template>
  <template v-else-if="variantIs(model, 'ChangeCanister')">
    <VTextField
      :model-value="$t('permissions.resources.changecanister')"
      :label="$t('terms.resource')"
      variant="plain"
      density="compact"
      disabled
    />

    <VTextField
      :model-value="toResourceActionText(model.ChangeCanister)"
      :label="$t('terms.action')"
      variant="plain"
      density="compact"
      disabled
    />
  </template>
</template>

<script lang="ts" setup>
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import {
  PermissionResourceAction,
  AccountResourceAction,
  Resource,
  ResourceAction,
  ResourceId,
  SystemResourceAction,
} from '~/generated/station/station.did';
import { unreachable, variantIs } from '~/utils/helper.utils';

export type ResourceSpecifierFieldProps = {
  modelValue: Resource;
  mode?: 'view' | 'edit';
};

const props = withDefaults(defineProps<ResourceSpecifierFieldProps>(), {
  mode: 'edit',
});

const emit = defineEmits<{
  (event: 'update:modelValue', payload: ResourceSpecifierFieldProps['modelValue']): void;
}>();

const model = computed({
  get: () => props.modelValue,
  set: value => emit('update:modelValue', value),
});

const i18n = useI18n();

const toResourceIdText = (specifier: ResourceId): string => {
  if (variantIs(specifier, 'Any')) {
    return i18n.t('terms.any');
  }

  if (variantIs(specifier, 'Id')) {
    return specifier.Id;
  }

  return unreachable(specifier);
};

const toResourceActionText = (action: ResourceAction): string => {
  if (variantIs(action, 'List')) {
    return i18n.t('permissions.actions.list');
  }

  if (variantIs(action, 'Create')) {
    return i18n.t('permissions.actions.create');
  }

  if (variantIs(action, 'Read')) {
    return i18n.t('permissions.actions.read') + ` (${toResourceIdText(action.Read)})`;
  }

  if (variantIs(action, 'Update')) {
    return i18n.t('permissions.actions.update') + ` (${toResourceIdText(action.Update)})`;
  }

  if (variantIs(action, 'Delete')) {
    return i18n.t('permissions.actions.delete') + ` (${toResourceIdText(action.Delete)})`;
  }

  return unreachable(action);
};

const toAccountResourceActionText = (action: AccountResourceAction): string => {
  if (variantIs(action, 'Create')) {
    return i18n.t('permissions.actions.create');
  }

  if (variantIs(action, 'List')) {
    return i18n.t('permissions.actions.list');
  }

  if (variantIs(action, 'Read')) {
    return i18n.t('permissions.actions.read') + ` (${toResourceIdText(action.Read)})`;
  }

  if (variantIs(action, 'Update')) {
    return i18n.t('permissions.actions.update') + ` (${toResourceIdText(action.Update)})`;
  }

  if (variantIs(action, 'Transfer')) {
    return i18n.t('permissions.actions.update') + ` (${toResourceIdText(action.Transfer)})`;
  }

  return unreachable(action);
};

const toSystemResourceActionText = (specifier: SystemResourceAction): string => {
  if (variantIs(specifier, 'Capabilities')) {
    return i18n.t('permissions.actions.capabilities');
  }

  if (variantIs(specifier, 'SystemInfo')) {
    return i18n.t('permissions.actions.systeminfo');
  }

  return unreachable(specifier);
};

const toPermissionResourceActionTest = (specifier: PermissionResourceAction): string => {
  if (variantIs(specifier, 'Update')) {
    return i18n.t('permissions.actions.update');
  }

  if (variantIs(specifier, 'Read')) {
    return i18n.t('permissions.actions.read');
  }

  return unreachable(specifier);
};
</script>
