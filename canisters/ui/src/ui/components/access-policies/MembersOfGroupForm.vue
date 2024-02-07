<template>
  <VForm ref="form" @submit.prevent="submit">
    <VTextField
      v-if="model.policyId"
      v-model="model.policyId"
      name="id"
      :label="$t('terms.id')"
      variant="plain"
      density="compact"
      readonly
    />
    <VAutocomplete
      v-model="model.groupIds"
      name="group_ids"
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
import { computed, onMounted, ref, toRefs, watch } from 'vue';
import { UUID, UserGroup } from '~/generated/wallet/wallet.did';
import { useUserGroupsAutocomplete } from '~/ui/composables/autocomplete.composable';
import { VFormValidation } from '~/ui/types';

const form = ref<VFormValidation | null>(null);
const userGroupsAutocomplete = useUserGroupsAutocomplete();

onMounted(() => {
  userGroupsAutocomplete.searchItems();
});

const isFormValid = computed(() => (form.value ? form.value.isValid : false));

export type MembersOfGroupFormProps = {
  modelValue: { policyId: UUID | null; groupIds: UUID[]; prefilledGroups?: UserGroup[] };
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

const reactiveProps = toRefs(props);

const model = computed({
  get: () => reactiveProps.modelValue.value,
  set: value => emit('update:modelValue', value),
});

const userGroups = computed(() => {
  const groups = userGroupsAutocomplete.results.value.map(group => ({
    title: group.name,
    value: group.id,
  }));

  (model.value.prefilledGroups ?? []).forEach(group => {
    if (!groups.find(g => g.value === group.id)) {
      groups.push({
        title: group.name,
        value: group.id,
      });
    }
  });

  model.value.groupIds?.forEach(group => {
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
    emit('submit', model.value);
  }
};
</script>
