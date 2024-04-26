<template>
  <tr v-if="app.isMobile" data-test-id="mobile-list-view">
    <div class="text-body-2 font-weight-bold pl-4 pt-2">
      {{ $t(`access_policies.resources.${resource.resourceType.toLowerCase()}`) }}
    </div>
    <VCard v-for="(specifier, idx) in resource.resources" :key="idx" variant="text" class="mb-1">
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
                :submit-cb="form => onMembersOfGroupFormSubmit(specifier.resource, form)"
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
                :submit-cb="form => onSpecificUsersFormSubmit(specifier.resource, form)"
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
    <tr v-for="(specifier, idx) in resource.resources" :key="idx">
      <td class="bb-none">
        {{ $t(`access_policies.actions.${specifier.action.toLowerCase()}`) }}
      </td>
      <td class="bb-none cursor-pointer">
        <MembersOfGroupAction
          :specifier="specifier"
          :model-value="getMembersOfGroupForm(idx, specifier)"
          :submit-cb="form => onMembersOfGroupFormSubmit(specifier.resource, form)"
          @update:model-value="model => updateMembersOfGroupModel(idx, model)"
          @editing="emit('editing', $event)"
        />
      </td>
      <td class="bb-none cursor-pointer">
        <SpecificUsersAction
          :specifier="specifier"
          :model-value="getSpecificUsersForm(idx, specifier)"
          :submit-cb="form => onSpecificUsersFormSubmit(specifier.resource, form)"
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
import { Proposal, Resource } from '~/generated/station/station.did';
import {
  AggregatedResouceAccessPolicies,
  ResourceAccessPolicySpecifier,
} from '~/types/access-policies.types';
import { useAppStore } from '~/stores/app.store';
import { useStationStore } from '~/stores/station.store';
import EveryoneAction from './EveryoneAction.vue';
import MembersOfGroupAction from './MembersOfGroupAction.vue';
import { MembersOfGroupFormProps } from './MembersOfGroupForm.vue';
import SpecificUsersAction from './SpecificUsersAction.vue';
import { SpecificUsersFormProps } from './SpecificUsersForm.vue';

const station = useStationStore();
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

  const groups = [...specifier.allow.membersOfGroup];
  return {
    valid: true,
    modelValue: {
      groupIds: groups.map(g => g.id),
      prefilledGroups: groups,
    },
  };
};

const onMembersOfGroupFormSubmit = (
  resource: Resource,
  form: MembersOfGroupFormProps,
): Promise<Proposal> => {
  return station.service.editAccessPolicy({
    auth_scope: [],
    user_groups: [form.modelValue.groupIds],
    users: [],
    resource,
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

  const users = [...specifier.allow.specificUsers];
  return {
    valid: true,
    modelValue: {
      userIds: users.map(g => g.id),
      prefilledUsers: users,
    },
  };
};

const onSpecificUsersFormSubmit = (
  resource: Resource,
  form: SpecificUsersFormProps,
): Promise<Proposal> => {
  return station.service.editAccessPolicy({
    auth_scope: [],
    user_groups: [],
    users: [form.modelValue.userIds],
    resource,
  });
};
</script>
