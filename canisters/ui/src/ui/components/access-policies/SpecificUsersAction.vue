<template>
  <ShortValues
    v-if="!showActionBtn"
    :values="specifier.users.specificUsers.users.map(u => u.name)"
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
          :disabled="shouldDisableSubmitBtn(elem.value)"
          color="primary"
          variant="flat"
          @click="submit"
        >
          {{ $t('terms.edit') }}
        </VBtn>
      </template>
    </ActionBtn>
    <ShortValues :values="specifier.users.specificUsers.users.map(u => u.name)" />
  </template>
</template>

<script lang="ts" setup>
import { mdiPencil } from '@mdi/js';
import { computed, toRefs } from 'vue';
import { Proposal } from '~/generated/wallet/wallet.did';
import { Privilege } from '~/types';
import { ResourceAccessPolicySpecifier } from '~/types/access-policies.types';
import ShortValues from '~/ui/components/ShortValues.vue';
import ActionBtn from '~/ui/components/buttons/ActionBtn.vue';
import {
  useOnFailedOperation,
  useOnSuccessfulOperation,
} from '~/ui/composables/notifications.composable';
import { hasRequiredPrivilege } from '~/ui/utils/auth';
import SpecificUsersForm, { SpecificUsersFormProps } from './SpecificUsersForm.vue';

const props = defineProps<{
  specifier: ResourceAccessPolicySpecifier;
  modelValue: SpecificUsersFormProps;
  submitCb: (form: SpecificUsersFormProps) => Promise<Proposal>;
}>();

const { specifier, submitCb } = toRefs(props);

const modelValue = computed<SpecificUsersFormProps>({
  get: () => props.modelValue,
  set: value => emit('update:modelValue', value),
});

const emit = defineEmits<{
  (event: 'editing', payload: boolean): void;
  (event: 'update:modelValue', payload: SpecificUsersFormProps): void;
}>();

const canAdd = computed(
  () =>
    hasRequiredPrivilege({
      anyOf: [Privilege.AddAccessPolicy],
    }) && !specifier.value.users.specificUsers.policy.id,
);
const canEdit = computed(
  () =>
    !!specifier.value.users.specificUsers.policy.id &&
    specifier.value.users.specificUsers.policy.canEdit,
);
const canRemove = computed(
  () =>
    !!specifier.value.users.specificUsers.policy.id &&
    specifier.value.users.specificUsers.policy.canRemove,
);
const showActionBtn = computed(() => canAdd.value || canEdit.value || canRemove.value);

const shouldDisableSubmitBtn = (elem: SpecificUsersFormProps) => {
  if (!elem.modelValue.userIds?.length && !canRemove.value) {
    return true;
  }

  if (!!elem.modelValue.policyId && elem.modelValue.userIds?.length && !canEdit.value) {
    return true;
  }

  return !elem.valid;
};
</script>
~/configs/access-policies.config
