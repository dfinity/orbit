<template>
  <VForm ref="form" @submit.prevent="submit">
    <VTextField
      v-if="modelValue.policyId"
      v-model="modelValue.policyId"
      name="id"
      :label="$t('terms.id')"
      variant="plain"
      density="compact"
      readonly
    />
    <VAutocomplete
      v-model="modelValue.groupIds"
      name="groups"
      :label="$t('terms.user_groups')"
      :loading="userGroupsAutocomplete.loading.value"
      variant="underlined"
      :items="userGroups"
      chips
      multiple
      clearable
      @update:search="userGroupsAutocomplete.searchItems"
    />
  </VForm>
</template>

<script lang="ts" setup>
import { computed, ref, watch } from 'vue';
import { VFormValidation } from '~/ui/types';
import { reactive } from 'vue';
import { useUserGroupsAutocomplete } from '~/ui/composables/autocomplete.composable';
import { onMounted } from 'vue';
import { UserGroup, UUID } from '~/generated/wallet/wallet.did';

const form = ref<VFormValidation | null>(null);
const userGroupsAutocomplete = useUserGroupsAutocomplete();

onMounted(() => {
  userGroupsAutocomplete.searchItems();
});

const isFormValid = computed(() => (form.value ? form.value.isValid : false));

export type MembersOfGroupFormProps = {
  modelValue: { policyId?: UUID; groupIds: UUID[]; prefilledGroups?: UserGroup[] };
  valid?: boolean;
};

const props = withDefaults(defineProps<MembersOfGroupFormProps>(), {
  valid: true,
});

const emit = defineEmits<{
  (event: 'update:modelValue', payload: MembersOfGroupFormProps['modelValue']): void;
  (event: 'valid', payload: boolean): void;
  (event: 'submit', payload: MembersOfGroupFormProps['modelValue']): void;
}>();

watch(
  () => isFormValid.value,
  isValid => emit('valid', isValid ?? false),
);

const modelValue = reactive({ ...props.modelValue });

watch(
  () => modelValue,
  value => emit('update:modelValue', value),
  { deep: true },
);

const userGroups = computed(() => {
  const groups = userGroupsAutocomplete.results.value.map(group => ({
    title: group.name,
    value: group.id,
  }));

  (modelValue.prefilledGroups ?? []).forEach(group => {
    if (!groups.find(g => g.value === group.id)) {
      groups.push({
        title: group.name,
        value: group.id,
      });
    }
  });

  props.modelValue.groupIds?.forEach(group => {
    if (!groups.find(g => g.value === group)) {
      groups.push({
        title: group,
        value: group,
      });
    }
  });

  return groups;
});

const submit = async () => {
  const { valid } = form.value ? await form.value.validate() : { valid: false };

  if (valid) {
    emit('submit', modelValue);
  }
};
</script>
