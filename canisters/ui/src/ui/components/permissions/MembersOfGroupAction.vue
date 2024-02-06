<template>
  <ShortValues
    v-if="!showActionBtn"
    :values="specifier.users.membersOfGroup.groups.map(g => g.name)"
    empty="-"
  />
  <template v-else>
    <ActionBtn
      v-model="modelValue"
      :title="$t('pages.access_policies.update_dialog_title')"
      size="small"
      density="comfortable"
      :icon="mdiPencil"
      :submit="submitCb"
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
          :disabled="shouldDisableSubmitBtn(elem.value)"
          color="primary"
          variant="flat"
          @click="submit"
        >
          {{ $t('terms.edit') }}
        </VBtn>
      </template>
    </ActionBtn>
    <ShortValues :values="specifier.users.membersOfGroup.groups.map(g => g.name)" />
  </template>
</template>

<script lang="ts" setup>
import { mdiPencil } from '@mdi/js';
import { computed, toRefs } from 'vue';
import { ResourcePermissionsSpecifier } from '~/configs/permissions.config';
import { Proposal } from '~/generated/wallet/wallet.did';
import { Privilege } from '~/types';
import ShortValues from '~/ui/components/ShortValues.vue';
import ActionBtn from '~/ui/components/buttons/ActionBtn.vue';
import MembersOfGroupForm, {
  MembersOfGroupFormProps,
} from '~/ui/components/permissions/MembersOfGroupForm.vue';
import {
  useOnFailedOperation,
  useOnSuccessfulOperation,
} from '~/ui/composables/notifications.composable';
import { hasRequiredPrivilege } from '~/ui/utils/auth';

const props = defineProps<{
  specifier: ResourcePermissionsSpecifier;
  modelValue: MembersOfGroupFormProps;
  submitCb: (form: MembersOfGroupFormProps) => Promise<Proposal>;
}>();

const { specifier, submitCb } = toRefs(props);

const modelValue = computed<MembersOfGroupFormProps>({
  get: () => props.modelValue,
  set: value => emit('update:modelValue', value),
});

const emit = defineEmits<{
  (event: 'editing', payload: boolean): void;
  (event: 'update:modelValue', payload: MembersOfGroupFormProps): void;
}>();

const canAdd = computed(
  () =>
    hasRequiredPrivilege({
      anyOf: [Privilege.AddAccessPolicy],
    }) && !specifier.value.users.membersOfGroup.policy.id,
);
const canEdit = computed(
  () =>
    !!specifier.value.users.membersOfGroup.policy.id &&
    specifier.value.users.membersOfGroup.policy.canEdit,
);
const canRemove = computed(
  () =>
    !!specifier.value.users.membersOfGroup.policy.id &&
    specifier.value.users.membersOfGroup.policy.canRemove,
);
const showActionBtn = computed(() => canAdd.value || canEdit.value || canRemove.value);

const shouldDisableSubmitBtn = (elem: MembersOfGroupFormProps) => {
  if (!elem.modelValue.groupIds?.length && !canRemove.value) {
    return true;
  }

  if (!!elem.modelValue.policyId && elem.modelValue.groupIds?.length && !canEdit.value) {
    return true;
  }

  return !elem.valid;
};
</script>
