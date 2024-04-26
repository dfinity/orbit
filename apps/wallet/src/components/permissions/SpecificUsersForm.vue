<template>
  <VForm ref="form" @submit.prevent="submit">
    <VAutocomplete
      v-model="model.userIds"
      name="user_ids"
      :label="$t('terms.users')"
      :loading="usersAutocomplete.loading.value"
      variant="underlined"
      :items="userList"
      chips
      multiple
      :disabled="isViewMode"
      clearable
      @update:search="usersAutocomplete.searchItems"
    />
  </VForm>
</template>

<script lang="ts" setup>
import { computed, onMounted, ref, toRefs, watch } from 'vue';
import { useUsersAutocomplete } from '~/composables/autocomplete.composable';
import { BasicUser, UUID } from '~/generated/station/station.did';
import { VFormValidation } from '~/types/helper.types';

export type SpecificUsersFormProps = {
  modelValue: { userIds: UUID[]; prefilledUsers?: BasicUser[] };
  valid?: boolean;
  mode?: 'view' | 'edit';
};

const props = withDefaults(defineProps<SpecificUsersFormProps>(), {
  valid: true,
  mode: 'edit',
});

const reactiveProps = toRefs(props);

const form = ref<VFormValidation | null>(null);
const usersAutocomplete = useUsersAutocomplete();

onMounted(() => {
  usersAutocomplete.searchItems();
});

const isFormValid = computed(() => (form.value ? form.value.isValid : false));

const isViewMode = computed(() => reactiveProps.mode.value === 'view');

const emit = defineEmits<{
  (event: 'update:modelValue', payload: SpecificUsersFormProps['modelValue']): void;
  (event: 'valid', payload: boolean): void;
  (event: 'submit', payload: SpecificUsersFormProps['modelValue']): void;
}>();

watch(
  () => isFormValid.value,
  isValid => emit('valid', isValid ?? false),
);

const model = computed({
  get: () => reactiveProps.modelValue.value,
  set: value => emit('update:modelValue', value),
});

const userList = computed(() => {
  const users = usersAutocomplete.results.value.map(user => ({
    title: user.name?.[0] ? user.name[0] : user.id,
    value: user.id,
  }));

  (model.value.prefilledUsers ?? []).forEach(user => {
    if (!users.find(g => g.value === user.id)) {
      users.push({
        title: user.name?.[0] ? user.name[0] : user.id,
        value: user.id,
      });
    }
  });

  model.value.userIds?.forEach(userId => {
    if (!users.find(g => g.value === userId)) {
      users.push({
        title: userId,
        value: userId,
      });
    }
  });

  return users;
});

const submit = async () => {
  const { valid } = form.value ? await form.value.validate() : { valid: false };

  if (valid) {
    emit('submit', model.value);
  }
};
</script>
