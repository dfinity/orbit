<template>
  <AuthCheck :privileges="[Privilege.AddAccessPolicy]">
    <ActionBtn
      v-model="modelValue"
      :title="$t('pages.permissions.update_dialog_title')"
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
          :disabled="!elem.value.valid"
          color="primary"
          variant="flat"
          @click="submit"
        >
          {{ $t('terms.edit') }}
        </VBtn>
      </template>
    </ActionBtn>
    <ShortValues :values="specifier.users.specificUsers.users.map(u => u.name)" />

    <template #unauthorized>
      <ShortValues :values="specifier.users.specificUsers.users.map(u => u.name)" empty="-" />
    </template>
  </AuthCheck>
</template>

<script lang="ts" setup>
import { mdiPencil } from '@mdi/js';
import { computed, toRefs } from 'vue';
import { ResourcePermissionsSpecifier } from '~/configs/permissions.config';
import { Proposal } from '~/generated/wallet/wallet.did';
import { Privilege } from '~/types';
import AuthCheck from '~/ui/components/AuthCheck.vue';
import ShortValues from '~/ui/components/ShortValues.vue';
import ActionBtn from '~/ui/components/buttons/ActionBtn.vue';
import SpecificUsersForm, {
  SpecificUsersFormProps,
} from '~/ui/components/permissions/SpecificUsersForm.vue';
import {
  useOnFailedOperation,
  useOnSuccessfulOperation,
} from '~/ui/composables/notifications.composable';

const props = defineProps<{
  specifier: ResourcePermissionsSpecifier;
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
</script>
