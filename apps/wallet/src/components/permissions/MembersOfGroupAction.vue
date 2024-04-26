<template>
  <ShortValues
    v-if="!canEdit"
    :values="specifier.allow.membersOfGroup.map(g => g.name)"
    empty="-"
  />
  <template v-else>
    <ActionBtn
      v-model="modelValue"
      :title="$t('pages.permissions.update_dialog_title')"
      size="small"
      density="comfortable"
      :icon="mdiPencil"
      :submit="submitCb"
      data-test-id="members-of-group-action-btn"
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
        <VBtn :loading="saving" :disabled="!elem.value.valid" @click="submit">
          {{ $t('terms.edit') }}
        </VBtn>
      </template>
    </ActionBtn>
    <ShortValues :values="specifier.allow.membersOfGroup.map(g => g.name)" />
  </template>
</template>

<script lang="ts" setup>
import { mdiPencil } from '@mdi/js';
import { computed, toRefs } from 'vue';
import ShortValues from '~/components/ShortValues.vue';
import ActionBtn from '~/components/buttons/ActionBtn.vue';
import {
  useOnFailedOperation,
  useOnSuccessfulOperation,
} from '~/composables/notifications.composable';
import { Proposal } from '~/generated/station/station.did';
import { ResourcePermissionSpecifier } from '~/types/permissions.types';
import MembersOfGroupForm, { MembersOfGroupFormProps } from './MembersOfGroupForm.vue';

const props = defineProps<{
  specifier: ResourcePermissionSpecifier;
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

const canEdit = computed(() => specifier.value.canEdit);
</script>
