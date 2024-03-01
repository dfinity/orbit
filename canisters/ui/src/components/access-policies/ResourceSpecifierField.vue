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
      :model-value="toCommonActionSpecfierText(model.AccessPolicy)"
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
      :model-value="toCommonActionSpecfierText(model.Account)"
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
      :model-value="toCommonActionSpecfierText(model.AddressBook)"
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
      :model-value="toCommonActionSpecfierText(model.ProposalPolicy)"
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
      :model-value="toCommonActionSpecfierText(model.User)"
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
      :model-value="toCommonActionSpecfierText(model.UserGroup)"
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
      :model-value="toCommonActionSpecfierText(model.Proposal)"
      :label="$t('terms.action')"
      variant="plain"
      density="compact"
      disabled
    />
  </template>
  <template v-else-if="variantIs(model, 'CanisterSettings')">
    <VTextField
      :model-value="$t('access_policies.resources.canistersettings')"
      :label="$t('terms.resource')"
      variant="plain"
      density="compact"
      disabled
    />

    <VTextField
      :model-value="toCanisterSettingsSpecifierText(model.CanisterSettings)"
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
      :model-value="toCommonActionSpecfierText(model.ChangeCanister)"
      :label="$t('terms.action')"
      variant="plain"
      density="compact"
      disabled
    />
  </template>
  <template v-else-if="variantIs(model, 'Transfer')">
    <VTextField
      :model-value="$t('access_policies.resources.transfer')"
      :label="$t('terms.resource')"
      variant="plain"
      density="compact"
      disabled
    />

    <VTextField
      :model-value="toTransferActionSpecifierText(model.Transfer)"
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
  CanisterSettingsActionSpecifier,
  CommonActionSpecifier,
  CommonSpecifier,
  ResourceSpecifier,
  TransferActionSpecifier,
} from '~/generated/wallet/wallet.did';
import { unreachable, variantIs } from '~/utils/helper.utils';

export type ResourceSpecifierFieldProps = {
  modelValue: ResourceSpecifier;
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

const toCommonSpecifierText = (specifier: CommonSpecifier): string => {
  if (variantIs(specifier, 'Any')) {
    return i18n.t('terms.any');
  }

  if (variantIs(specifier, 'Id')) {
    return specifier.Id.join(', ');
  }

  if (variantIs(specifier, 'Group')) {
    return specifier.Group.join(', ');
  }

  return unreachable(specifier);
};

const toCommonActionSpecfierText = (action: CommonActionSpecifier): string => {
  if (variantIs(action, 'List')) {
    return i18n.t('access_policies.actions.list');
  }

  if (variantIs(action, 'Create')) {
    return i18n.t('access_policies.actions.create');
  }

  if (variantIs(action, 'Read')) {
    return i18n.t('access_policies.actions.read') + ` (${toCommonSpecifierText(action.Read)})`;
  }

  if (variantIs(action, 'Update')) {
    return i18n.t('access_policies.actions.update') + ` (${toCommonSpecifierText(action.Update)})`;
  }

  if (variantIs(action, 'Delete')) {
    return i18n.t('access_policies.actions.delete') + ` (${toCommonSpecifierText(action.Delete)})`;
  }

  return unreachable(action);
};

const toTransferActionSpecifierText = (action: TransferActionSpecifier): string => {
  if (variantIs(action, 'Create')) {
    return (
      i18n.t('access_policies.actions.create') +
      ` (${toCommonSpecifierText(action.Create.account)})`
    );
  }

  if (variantIs(action, 'Read')) {
    return (
      i18n.t('access_policies.actions.read') + ` (${toCommonSpecifierText(action.Read.account)})`
    );
  }

  if (variantIs(action, 'Delete')) {
    return (
      i18n.t('access_policies.actions.update') +
      ` (${toCommonSpecifierText(action.Delete.account)})`
    );
  }

  return unreachable(action);
};

const toCanisterSettingsSpecifierText = (specifier: CanisterSettingsActionSpecifier): string => {
  if (variantIs(specifier, 'Read')) {
    return i18n.t('access_policies.actions.readpublicconfig');
  }

  if (variantIs(specifier, 'ReadConfig')) {
    return i18n.t('access_policies.actions.readsensitiveconfig');
  }

  return unreachable(specifier);
};
</script>
