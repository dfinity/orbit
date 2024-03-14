<template>
  <VForm ref="form" @submit.prevent="submit">
    <ResourceSpecifierField
      v-if="model.resource"
      :mode="props.mode.value"
      :model-value="model.resource"
    />

    <template v-if="model.allow">
      <SpecificUsersForm
        v-if="model.allow.users[0]"
        :mode="props.mode.value"
        :model-value="{
          userIds: model.allow.users[0],
        }"
      />
      <MembersOfGroupForm
        v-else-if="model.allow.user_groups[0]"
        :mode="props.mode.value"
        :model-value="{
          groupIds: model.allow.user_groups[0],
        }"
      />
      <!-- todo: add specific authenticated vs any rule -->
      <!-- <VCheckbox
        v-else-if="variantIs(model.allow, 'Any')"
        v-model="model.allow.Any"
        :label="$t('terms.everyone')"
        variant="plain"
        density="compact"
        disabled
      /> -->
    </template>
  </VForm>
</template>

<script lang="ts" setup>
import { computed, ref, toRefs } from 'vue';
import MembersOfGroupForm from '~/components/access-policies/MembersOfGroupForm.vue';
import ResourceSpecifierField from '~/components/access-policies/ResourceSpecifierField.vue';
import SpecificUsersForm from '~/components/access-policies/SpecificUsersForm.vue';
import { AccessPolicy } from '~/generated/wallet/wallet.did';
import { VFormValidation } from '~/types/helper.types';
import { variantIs } from '~/utils/helper.utils';

export type AccessPolicyFormProps = {
  modelValue: Partial<AccessPolicy>;
  valid?: boolean;
  mode?: 'view' | 'edit';
  display?: {
    id?: boolean;
  };
};

const form = ref<VFormValidation | null>(null);

const input = withDefaults(defineProps<AccessPolicyFormProps>(), {
  valid: true,
  display: () => ({
    id: true,
    specifier: true,
  }),
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
