<template>
  <ShortValues v-if="!canEdit" :values="specifier.allow.specificUsers.map(u => u.name)" empty="-" />
  <template v-else>
    <ActionBtn
      v-model="model"
      :title="$t('pages.permissions.update_dialog_title')"
      size="small"
      density="comfortable"
      :icon="mdiPencil"
      :submit="submitCb"
      data-test-id="specific-users-action-btn"
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
        <VBtn :loading="saving" :disabled="!elem.value.valid" @click="submit">
          {{ $t('terms.edit') }}
        </VBtn>
      </template>
    </ActionBtn>
    <ShortValues :values="specifier.allow.specificUsers.map(u => u.name)" />
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
import { Request } from '~/generated/station/station.did';
import { ResourcePermissionSpecifier } from '~/types/permissions.types';
import SpecificUsersForm, { SpecificUsersFormProps } from './SpecificUsersForm.vue';

const props = defineProps<{
  specifier: ResourcePermissionSpecifier;
  modelValue: SpecificUsersFormProps;
  submitCb: (form: SpecificUsersFormProps) => Promise<Request>;
}>();

const { specifier, submitCb, modelValue: reactivePropModel } = toRefs(props);

const model = computed<SpecificUsersFormProps>({
  get: () => reactivePropModel.value,
  set: value => emit('update:modelValue', value),
});

const emit = defineEmits<{
  (event: 'editing', payload: boolean): void;
  (event: 'update:modelValue', payload: SpecificUsersFormProps): void;
}>();

const canEdit = computed(() => specifier.value.canEdit);
</script>
