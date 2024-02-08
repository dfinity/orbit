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
      v-model="model.userIds"
      name="user_ids"
      :label="$t('terms.users')"
      :loading="usersAutocomplete.loading.value"
      variant="underlined"
      :items="userList"
      chips
      multiple
      clearable
      @update:search="usersAutocomplete.searchItems"
    />
  </VForm>
</template>

<script lang="ts" setup>
import { computed, onMounted, ref, toRefs, watch } from 'vue';
import { useUsersAutocomplete } from '~/composables/autocomplete.composable';
import { BasicUser, UUID } from '~/generated/wallet/wallet.did';
import { VFormValidation } from '~/types/helper.types';

const form = ref<VFormValidation | null>(null);
const usersAutocomplete = useUsersAutocomplete();

onMounted(() => {
  usersAutocomplete.searchItems();
});

const isFormValid = computed(() => (form.value ? form.value.isValid : false));

export type SpecificUsersFormProps = {
  modelValue: { policyId: UUID | null; userIds: UUID[]; prefilledUsers?: BasicUser[] };
  valid?: boolean;
};

const props = withDefaults(defineProps<SpecificUsersFormProps>(), {
  valid: true,
});

const reactiveProps = toRefs(props);

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
