<template>
  <tr>
    <td colspan="4" class="bb-none font-weight-bold pt-4 pb-1">
      {{ $t(`permissions.resources.${resource.resourceType.toLowerCase()}`) }}
    </td>
  </tr>
  <tr v-for="(permission, idx) in permissions" :key="idx">
    <td class="bb-none">
      {{ $t(`permissions.actions.${permission.action.toLowerCase()}`) }}
    </td>
    <td class="bb-none cursor-pointer">
      <AuthCheck :privileges="[Privilege.AddAccessPolicy]">
        <ShortValues :values="permission.user.groups.ids.map(id => wallet.userGroup(id))" />

        <template #unauthorized>
          <ShortValues :values="permission.user.groups.ids.map(id => wallet.userGroup(id))" />
        </template>
      </AuthCheck>
    </td>
    <td class="bb-none cursor-pointer">
      <AuthCheck :privileges="[Privilege.AddAccessPolicy]">
        <ShortValues :values="permission.user.users.ids" />

        <template #unauthorized>
          <ShortValues :values="permission.user.users.ids" />
        </template>
      </AuthCheck>
    </td>
    <td class="bb-none cursor-pointer d-flex align-center">
      <AuthCheck :privileges="[Privilege.AddAccessPolicy]">
        <ActionBtn
          size="default"
          density="comfortable"
          :model-value="{ specifier: permission.specifier, everyone: permission.user.everyone }"
          :icon="
            permission.user.everyone.value ? mdiCheckboxMarkedOutline : mdiCheckboxBlankOutline
          "
          :submit="
            ({ specifier, everyone }) => {
              if (everyone.policyId) {
                return wallet.service.removeAccessPolicy({ policy_id: everyone.policyId });
              }

              return wallet.service.addAccessPolicy({ user: { Any: null }, resource: specifier });
            }
          "
          @opened="emit('editing', true)"
          @closed="emit('editing', false)"
          @failed="useOnFailedOperation"
          @submitted="useOnSuccessfulOperation"
        />

        <template #unauthorized>
          <VCheckbox
            hide-details
            density="comfortable"
            disabled
            :value="permission.user.everyone.value"
          />
        </template>
      </AuthCheck>
    </td>
  </tr>
</template>

<script lang="ts" setup>
import { mdiCheckboxBlankOutline, mdiCheckboxMarkedOutline } from '@mdi/js';
import { computed } from 'vue';
import { ResourcePermissions } from '~/configs/permissions.config';
import { variantIs } from '~/core';
import { AccessPolicy, ResourceSpecifier, UUID } from '~/generated/wallet/wallet.did';
import { Privilege } from '~/types';
import { ResourceActionEnum } from '~/types/permissions.types';
import AuthCheck from '~/ui/components/AuthCheck.vue';
import ShortValues from '~/ui/components/ShortValues.vue';
import ActionBtn from '~/ui/components/buttons/ActionBtn.vue';
import {
  useOnFailedOperation,
  useOnSuccessfulOperation,
} from '~/ui/composables/notifications.composable';
import { useWalletStore } from '~/ui/stores/wallet';

const wallet = useWalletStore();

const props = withDefaults(
  defineProps<{
    resource: ResourcePermissions;
    policies: AccessPolicy[];
  }>(),
  {
    policies: () => [],
  },
);

const emit = defineEmits<{
  (event: 'editing', payload: boolean): void;
}>();

type ResourcePermissionsView = {
  action: ResourceActionEnum;
  specifier: ResourceSpecifier;
  user: UserAccessInfo;
};

type UserAccessInfo = {
  groups: { policyId?: UUID; ids: UUID[] };
  users: { policyId?: UUID; ids: UUID[] };
  everyone: { policyId?: UUID; value: boolean };
};

const permissions = computed<ResourcePermissionsView[]>(() => {
  return props.resource.specifiers.map(specifier => {
    const user: UserAccessInfo = {
      groups: { ids: [] },
      users: { ids: [] },
      everyone: { value: false },
    };

    const policies = props.policies.filter(policy =>
      props.resource.match(specifier.specifier, policy),
    );

    for (const policy of policies) {
      if (variantIs(policy.user, 'Any')) {
        user.everyone.policyId = policy.id;
        user.everyone.value = true;
      } else if (variantIs(policy.user, 'Id')) {
        user.users.policyId = policy.id;
        user.users.ids = user.users.ids.concat(policy.user.Id);
      } else if (variantIs(policy.user, 'Group')) {
        user.groups.policyId = policy.id;
        user.groups.ids = user.groups.ids.concat(policy.user.Group);
      }
    }

    return {
      action: specifier.action,
      specifier: specifier.specifier,
      user,
    };
  });
});
</script>
