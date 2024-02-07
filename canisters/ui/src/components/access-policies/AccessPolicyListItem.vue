<template>
  <tr v-if="app.isMobile" data-test-id="mobile-list-view">
    <div class="text-body-2 font-weight-bold pl-4 pt-2">
      {{ $t(`access_policies.resources.${resource.resourceType.toLowerCase()}`) }}
    </div>
    <VCard v-for="(specifier, idx) in resource.specifiers" :key="idx" variant="text" class="mb-1">
      <VCardTitle class="text-body-1 py-0">
        {{ $t(`access_policies.actions.${specifier.action.toLowerCase()}`) }}
      </VCardTitle>
      <VCardText>
        <VList>
          <VListItem class="px-0 pt-0">
            <VListItemTitle class="text-body-2">
              {{ $t(`access_policies.group_members_title`) }}
            </VListItemTitle>
            <VListItemSubtitle>
              <MembersOfGroupAction
                :specifier="specifier"
                :model-value="getMembersOfGroupForm(idx, specifier)"
                :submit-cb="form => onMembersOfGroupFormSubmit(specifier.specifier, form)"
                @update:model-value="model => updateMembersOfGroupModel(idx, model)"
                @editing="emit('editing', $event)"
              />
            </VListItemSubtitle>
          </VListItem>
          <VListItem class="px-0">
            <VListItemTitle class="text-body-2">
              {{ $t(`access_policies.specific_users_title`) }}
            </VListItemTitle>
            <VListItemSubtitle>
              <SpecificUsersAction
                :specifier="specifier"
                :model-value="getSpecificUsersForm(idx, specifier)"
                :submit-cb="form => onSpecificUsersFormSubmit(specifier.specifier, form)"
                @update:model-value="model => updateSpecificUsersModel(idx, model)"
                @editing="emit('editing', $event)"
              />
            </VListItemSubtitle>
          </VListItem>
          <VListItem class="px-0">
            <VListItemTitle class="text-body-2">
              {{ $t(`access_policies.everyone_title`) }}
            </VListItemTitle>
            <VListItemSubtitle>
              <EveryoneAction :specifier="specifier" @editing="emit('editing', $event)" />
            </VListItemSubtitle>
          </VListItem>
        </VList>
        <VDivider />
      </VCardText>
    </VCard>
  </tr>
  <template v-else>
    <tr v-bind="$attrs">
      <td colspan="4" class="bb-none font-weight-bold pt-4 pb-1">
        {{ $t(`access_policies.resources.${resource.resourceType.toLowerCase()}`) }}
      </td>
    </tr>
    <tr v-for="(specifier, idx) in resource.specifiers" :key="idx">
      <td class="bb-none">
        {{ $t(`access_policies.actions.${specifier.action.toLowerCase()}`) }}
      </td>
      <td class="bb-none cursor-pointer">
        <MembersOfGroupAction
          :specifier="specifier"
          :model-value="getMembersOfGroupForm(idx, specifier)"
          :submit-cb="form => onMembersOfGroupFormSubmit(specifier.specifier, form)"
          @update:model-value="model => updateMembersOfGroupModel(idx, model)"
          @editing="emit('editing', $event)"
        />
      </td>
      <td class="bb-none cursor-pointer">
        <SpecificUsersAction
          :specifier="specifier"
          :model-value="getSpecificUsersForm(idx, specifier)"
          :submit-cb="form => onSpecificUsersFormSubmit(specifier.specifier, form)"
          @update:model-value="model => updateSpecificUsersModel(idx, model)"
          @editing="emit('editing', $event)"
        />
      </td>
      <td class="bb-none cursor-pointer d-flex align-center">
        <EveryoneAction :specifier="specifier" @editing="emit('editing', $event)" />
      </td>
    </tr>
  </template>
</template>

<script lang="ts" setup>
import { ref, toRefs, watch } from 'vue';
import { Proposal, ResourceSpecifier } from '~/generated/wallet/wallet.did';
import {
  AggregatedResouceAccessPolicies,
  ResourceAccessPolicySpecifier,
} from '~/types/access-policies.types';
import { useAppStore } from '~/stores/app.store';
import { useWalletStore } from '~/stores/wallet.store';
import EveryoneAction from './EveryoneAction.vue';
import MembersOfGroupAction from './MembersOfGroupAction.vue';
import { MembersOfGroupFormProps } from './MembersOfGroupForm.vue';
import SpecificUsersAction from './SpecificUsersAction.vue';
import { SpecificUsersFormProps } from './SpecificUsersForm.vue';

const wallet = useWalletStore();
const app = useAppStore();

const props = defineProps<{
  resource: AggregatedResouceAccessPolicies;
}>();

const { resource } = toRefs(props);

const emit = defineEmits<{
  (event: 'editing', payload: boolean): void;
}>();

watch(
  () => resource.value,
  () => {
    membersOfGroupModels.value = {};
    specificUsersModels.value = {};
  },
  {
    deep: true,
  },
);

const membersOfGroupModels = ref<Record<number, MembersOfGroupFormProps>>({});
const updateMembersOfGroupModel = (idx: number, model: MembersOfGroupFormProps) => {
  membersOfGroupModels.value[idx] = model;
};

const getMembersOfGroupForm = (
  idx: number,
  specifier: ResourceAccessPolicySpecifier,
): MembersOfGroupFormProps => {
  if (membersOfGroupModels.value[idx]) {
    return membersOfGroupModels.value[idx];
  }

  const groups = [...specifier.users.membersOfGroup.groups];
  return {
    valid: true,
    modelValue: {
      policyId: specifier.users.membersOfGroup.policy.id,
      groupIds: groups.map(g => g.id),
      prefilledGroups: groups,
    },
  };
};

const onMembersOfGroupFormSubmit = (
  resource: ResourceSpecifier,
  form: MembersOfGroupFormProps,
): Promise<Proposal> => {
  if (form.modelValue.policyId === null) {
    return wallet.service.addAccessPolicy({
      user: { Group: form.modelValue.groupIds },
      resource,
    });
  }

  if (form.modelValue.groupIds.length === 0) {
    return wallet.service.removeAccessPolicy({
      policy_id: form.modelValue.policyId,
    });
  }

  return wallet.service.editAccessPolicy({
    policy_id: form.modelValue.policyId,
    user: [{ Group: form.modelValue.groupIds }],
    resource: [],
  });
};

const specificUsersModels = ref<Record<number, SpecificUsersFormProps>>({});
const updateSpecificUsersModel = (idx: number, model: SpecificUsersFormProps) => {
  specificUsersModels.value[idx] = model;
};

const getSpecificUsersForm = (
  idx: number,
  specifier: ResourceAccessPolicySpecifier,
): SpecificUsersFormProps => {
  if (specificUsersModels.value[idx]) {
    return specificUsersModels.value[idx];
  }

  const users = [...specifier.users.specificUsers.users];
  return {
    valid: true,
    modelValue: {
      policyId: specifier.users.specificUsers.policy.id,
      userIds: users.map(g => g.id),
      prefilledUsers: users,
    },
  };
};

const onSpecificUsersFormSubmit = (
  resource: ResourceSpecifier,
  form: SpecificUsersFormProps,
): Promise<Proposal> => {
  if (form.modelValue.policyId === null) {
    return wallet.service.addAccessPolicy({
      user: { Id: form.modelValue.userIds },
      resource,
    });
  }

  if (form.modelValue.userIds.length === 0) {
    return wallet.service.removeAccessPolicy({ policy_id: form.modelValue.policyId });
  }

  return wallet.service.editAccessPolicy({
    policy_id: form.modelValue.policyId,
    user: [{ Id: form.modelValue.userIds }],
    resource: [],
  });
};
</script>
