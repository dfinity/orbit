<template>
  <VForm ref="form" @submit.prevent="submit">
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
      :disabled="isViewMode"
      @update:search="userGroupsAutocomplete.searchItems"
    />
  </VForm>
</template>

<script lang="ts" setup>
import { computed, onMounted, ref, toRefs, watch } from 'vue';
import { useUserGroupsAutocomplete } from '~/composables/autocomplete.composable';
import { UUID, UserGroup } from '~/generated/station/station.did';
import { VFormValidation } from '~/types/helper.types';

export type MembersOfGroupFormProps = {
  modelValue: { groupIds: UUID[]; prefilledGroups?: UserGroup[] };
  valid?: boolean;
  mode?: 'view' | 'edit';
};

const props = withDefaults(defineProps<MembersOfGroupFormProps>(), {
  valid: true,
  mode: 'edit',
});

const emit = defineEmits<{
  (event: 'update:modelValue', payload: MembersOfGroupFormProps['modelValue']): void;
  (event: 'valid', payload: boolean): void;
  (event: 'submit', payload: MembersOfGroupFormProps['modelValue']): void;
}>();

const reactiveProps = toRefs(props);

const form = ref<VFormValidation | null>(null);
const userGroupsAutocomplete = useUserGroupsAutocomplete();

onMounted(() => {
  userGroupsAutocomplete.searchItems();
});

const isFormValid = computed(() => (form.value ? form.value.isValid : false));
const isViewMode = computed(() => props.mode === 'view');

watch(
  () => isFormValid.value,
  isValid => emit('valid', isValid ?? false),
);

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
