<template>
  <tr>
    <td colspan="4" class="bb-none font-weight-bold pt-4 pb-1">
      {{ $t(`permissions.resources.${props.resource.resourceType.toLowerCase()}`) }}
    </td>
  </tr>
  <tr v-for="(specifier, idx) in props.resource.specifiers" :key="idx">
    <td class="bb-none">
      {{ $t(`permissions.actions.${specifier.action.toLowerCase()}`) }}
    </td>
    <td class="bb-none cursor-pointer">
      <AuthCheck :privileges="[Privilege.AddAccessPolicy]">
        <ActionBtn
          :model-value="getMembersOfGroupForm(idx, specifier)"
          :title="$t('pages.permissions.update_dialog_title')"
          size="small"
          density="comfortable"
          :icon="mdiPencil"
          :submit="form => onMembersOfGroupFormSubmit(specifier.specifier, form)"
          @update:model-value="model => updateMembersOfGroupModel(idx, model)"
          @opened="emit('editing', true)"
          @closed="emit('editing', false)"
          @failed="useOnFailedOperation"
          @submitted="useOnSuccessfulOperation"
        >
          <template #default="{ model: elem, submit }">
            <MembersOfGroupForm
              v-model="elem.value.modelValue"
              @valid="isValid => (elem.value.valid = isValid)"
              @submit="submit"
            />
          </template>
          <template #actions="{ submit, loading: saving, model: elem }">
            <VSpacer />
            <VBtn
              :loading="saving"
              :disabled="!elem.value.valid"
              color="primary"
              variant="flat"
              @click="submit"
            >
              {{ $t('terms.edit') }}
            </VBtn>
          </template>
        </ActionBtn>
        <ShortValues
          :values="Object.values(specifier.users.membersOfGroup.groups).map(g => g.name)"
        />

        <template #unauthorized>
          <ShortValues
            :values="Object.values(specifier.users.membersOfGroup.groups).map(g => g.name)"
            empty="-"
          />
        </template>
      </AuthCheck>
    </td>
    <td class="bb-none cursor-pointer">
      <AuthCheck :privileges="[Privilege.AddAccessPolicy]">
        <ActionBtn
          :model-value="getSpecificUsersForm(idx, specifier)"
          :title="$t('pages.permissions.update_dialog_title')"
          size="small"
          density="comfortable"
          :icon="mdiPencil"
          :submit="form => onSpecificUsersFormSubmit(specifier.specifier, form)"
          @update:model-value="model => updateSpecificUsersModel(idx, model)"
          @opened="emit('editing', true)"
          @closed="emit('editing', false)"
          @failed="useOnFailedOperation"
          @submitted="useOnSuccessfulOperation"
        >
          <template #default="{ model: elem, submit }">
            <SpecificUsersForm
              v-model="elem.value.modelValue"
              @valid="isValid => (elem.value.valid = isValid)"
              @submit="submit"
            />
          </template>
          <template #actions="{ submit, loading: saving, model: elem }">
            <VSpacer />
            <VBtn
              :loading="saving"
              :disabled="!elem.value.valid"
              color="primary"
              variant="flat"
              @click="submit"
            >
              {{ $t('terms.edit') }}
            </VBtn>
          </template>
        </ActionBtn>
        <ShortValues
          :values="Object.values(specifier.users.specificUsers.users).map(u => u.name)"
        />

        <template #unauthorized>
          <ShortValues
            :values="Object.values(specifier.users.specificUsers.users).map(u => u.name)"
            empty="-"
          />
        </template>
      </AuthCheck>
    </td>
    <td class="bb-none cursor-pointer d-flex align-center">
      <AuthCheck :privileges="[Privilege.AddAccessPolicy]">
        <ActionBtn
          size="default"
          density="comfortable"
          :model-value="{
            specifier: specifier.specifier,
            everyone: { policyId: specifier.users.allUsers.policyId },
          }"
          :icon="
            specifier.users.allUsers.policyId ? mdiCheckboxMarkedOutline : mdiCheckboxBlankOutline
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
            :value="specifier.users.allUsers.policyId ? true : false"
          />
        </template>
      </AuthCheck>
    </td>
  </tr>
</template>

<script lang="ts" setup>
import { mdiCheckboxBlankOutline, mdiCheckboxMarkedOutline, mdiPencil } from '@mdi/js';
import { ResourcePermissions } from '~/configs/permissions.config';
import { Privilege } from '~/types';
import AuthCheck from '~/ui/components/AuthCheck.vue';
import ShortValues from '~/ui/components/ShortValues.vue';
import ActionBtn from '~/ui/components/buttons/ActionBtn.vue';
import MembersOfGroupForm, {
  MembersOfGroupFormProps,
} from '~/ui/components/permissions/MembersOfGroupForm.vue';
import {
  useOnFailedOperation,
  useOnSuccessfulOperation,
} from '~/ui/composables/notifications.composable';
import { useWalletStore } from '~/ui/stores/wallet';
import { ResourcePermissionsSpecifier } from '~/configs/permissions.config';
import { ref } from 'vue';
import { Proposal, ResourceSpecifier } from '~/generated/wallet/wallet.did';
import SpecificUsersForm, {
  SpecificUsersFormProps,
} from '~/ui/components/permissions/SpecificUsersForm.vue';

const wallet = useWalletStore();

const membersOfGroupModels = ref<Record<number, MembersOfGroupFormProps>>({});
const updateMembersOfGroupModel = (idx: number, model: MembersOfGroupFormProps) => {
  membersOfGroupModels.value[idx] = model;
};

const getMembersOfGroupForm = (
  idx: number,
  specifier: ResourcePermissionsSpecifier,
): MembersOfGroupFormProps => {
  if (membersOfGroupModels.value[idx]) {
    return membersOfGroupModels.value[idx];
  }

  const groups = Object.values(specifier.users.membersOfGroup.groups);
  return {
    valid: true,
    modelValue: {
      policyId: specifier.users.membersOfGroup.policyId,
      groupIds: groups.map(g => g.id),
      prefilledGroups: groups,
    },
  };
};

const onMembersOfGroupFormSubmit = (
  resource: ResourceSpecifier,
  form: MembersOfGroupFormProps,
): Promise<Proposal> => {
  if (form.modelValue.policyId === undefined) {
    return wallet.service.addAccessPolicy({
      user: { Group: form.modelValue.groupIds },
      resource,
    });
  }

  if (form.modelValue.groupIds.length === 0) {
    return wallet.service.removeAccessPolicy({ policy_id: form.modelValue.policyId });
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
  specifier: ResourcePermissionsSpecifier,
): SpecificUsersFormProps => {
  if (specificUsersModels.value[idx]) {
    return specificUsersModels.value[idx];
  }

  const users = Object.values(specifier.users.specificUsers.users);
  return {
    valid: true,
    modelValue: {
      policyId: specifier.users.specificUsers.policyId,
      userIds: users.map(g => g.id),
      prefilledUsers: users,
    },
  };
};

const onSpecificUsersFormSubmit = (
  resource: ResourceSpecifier,
  form: SpecificUsersFormProps,
): Promise<Proposal> => {
  if (form.modelValue.policyId === undefined) {
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

const props = defineProps<{
  resource: ResourcePermissions;
}>();

const emit = defineEmits<{
  (event: 'editing', payload: boolean): void;
}>();
</script>
