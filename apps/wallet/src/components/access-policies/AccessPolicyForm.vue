<template>
  <VForm ref="form" @submit.prevent="submit">
    <ResourceSpecifierField
      v-if="model.resource"
      :mode="props.mode.value"
      :model-value="model.resource"
    />

    <template v-if="model.allow">
      <SpecificUsersForm
        :mode="props.mode.value"
        :model-value="{
          userIds: model.allow.users,
        }"
      />
      <MembersOfGroupForm
        :mode="props.mode.value"
        :model-value="{
          groupIds: model.allow.user_groups,
        }"
      />
      <EveryoneForm
        :mode="props.mode.value"
        :model-value="toAuthScopeEnum(model.allow.auth_scope)"
      />
    </template>
  </VForm>
</template>

<script lang="ts" setup>
import { computed, ref, toRefs } from 'vue';
import MembersOfGroupForm from '~/components/access-policies/MembersOfGroupForm.vue';
import ResourceSpecifierField from '~/components/access-policies/ResourceSpecifierField.vue';
import SpecificUsersForm from '~/components/access-policies/SpecificUsersForm.vue';
import { AccessPolicy } from '~/generated/station/station.did';
import { toAuthScopeEnum } from '~/mappers/access-policies.mapper';
import { VFormValidation } from '~/types/helper.types';
import EveryoneForm from './EveryoneForm.vue';

export type AccessPolicyFormProps = {
  modelValue: Partial<AccessPolicy>;
  valid?: boolean;
  mode?: 'view' | 'edit';
};

const form = ref<VFormValidation | null>(null);

const input = withDefaults(defineProps<AccessPolicyFormProps>(), {
  valid: true,
  mode: 'edit',
});
const props = toRefs(input);

const emit = defineEmits<{
  (event: 'update:modelValue', payload: AccessPolicyFormProps['modelValue']): void;
  (event: 'valid', payload: boolean): void;
  (event: 'submit', payload: AccessPolicyFormProps['modelValue']): void;
}>();

const model = computed({
  get: () => props.modelValue.value,
  set: value => emit('update:modelValue', value),
});

const submit = async () => {
  const { valid } = form.value ? await form.value.validate() : { valid: false };

  if (valid) {
    emit('submit', model.value);
  }
};
</script>
