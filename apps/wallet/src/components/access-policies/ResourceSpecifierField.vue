<template>
  <template v-if="variantIs(model, 'AccessPolicy')">
    <VTextField
      :model-value="$t('access_policies.resources.accesspolicy')"
      :label="$t('terms.resource')"
      variant="plain"
      density="compact"
      disabled
    />

    <VTextField
      :model-value="toAccessPolicyResourceActionTest(model.AccessPolicy)"
      :label="$t('terms.action')"
      variant="plain"
      density="compact"
      disabled
    />
  </template>
  <template v-else-if="variantIs(model, 'Account')">
    <VTextField
      :model-value="$t('access_policies.resources.account')"
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
      :model-value="$t('access_policies.resources.addressbook')"
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
      :model-value="$t('access_policies.resources.proposalpolicy')"
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
      :model-value="$t('access_policies.resources.user')"
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
      :model-value="$t('access_policies.resources.usergroup')"
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
      :model-value="$t('access_policies.resources.proposal')"
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
      :model-value="$t('access_policies.resources.system')"
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
      :model-value="$t('access_policies.resources.changecanister')"
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
  AccessPolicyResourceAction,
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
    return i18n.t('access_policies.actions.list');
  }

  if (variantIs(action, 'Create')) {
    return i18n.t('access_policies.actions.create');
  }

  if (variantIs(action, 'Read')) {
    return i18n.t('access_policies.actions.read') + ` (${toResourceIdText(action.Read)})`;
  }

  if (variantIs(action, 'Update')) {
    return i18n.t('access_policies.actions.update') + ` (${toResourceIdText(action.Update)})`;
  }

  if (variantIs(action, 'Delete')) {
    return i18n.t('access_policies.actions.delete') + ` (${toResourceIdText(action.Delete)})`;
  }

  return unreachable(action);
};

const toAccountResourceActionText = (action: AccountResourceAction): string => {
  if (variantIs(action, 'Create')) {
    return i18n.t('access_policies.actions.create');
  }

  if (variantIs(action, 'List')) {
    return i18n.t('access_policies.actions.list');
  }

  if (variantIs(action, 'Read')) {
    return i18n.t('access_policies.actions.read') + ` (${toResourceIdText(action.Read)})`;
  }

  if (variantIs(action, 'Update')) {
    return i18n.t('access_policies.actions.update') + ` (${toResourceIdText(action.Update)})`;
  }

  if (variantIs(action, 'Transfer')) {
    return i18n.t('access_policies.actions.update') + ` (${toResourceIdText(action.Transfer)})`;
  }

  return unreachable(action);
};

const toSystemResourceActionText = (specifier: SystemResourceAction): string => {
  if (variantIs(specifier, 'Capabilities')) {
    return i18n.t('access_policies.actions.capabilities');
  }

  if (variantIs(specifier, 'SystemInfo')) {
    return i18n.t('access_policies.actions.systeminfo');
  }

  return unreachable(specifier);
};

const toAccessPolicyResourceActionTest = (specifier: AccessPolicyResourceAction): string => {
  if (variantIs(specifier, 'Update')) {
    return i18n.t('access_policies.actions.update');
  }

  if (variantIs(specifier, 'Read')) {
    return i18n.t('access_policies.actions.read');
  }

  return unreachable(specifier);
};
</script>
