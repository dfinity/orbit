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
      v-model="modelValue.userIds"
      name="groups"
      :label="$t('terms.users')"
      :loading="usersAutocomplete.loading.value"
      variant="underlined"
      :items="userGroups"
      chips
      multiple
      clearable
      @update:search="usersAutocomplete.searchItems"
    />
  </VForm>
</template>

<script lang="ts" setup>
import { computed, ref, watch } from 'vue';
import { VFormValidation } from '~/ui/types';
import { reactive } from 'vue';
import { useUsersAutocomplete } from '~/ui/composables/autocomplete.composable';
import { onMounted } from 'vue';
import { BasicUser, UUID } from '~/generated/wallet/wallet.did';

const form = ref<VFormValidation | null>(null);
const usersAutocomplete = useUsersAutocomplete();

onMounted(() => {
  usersAutocomplete.searchItems();
});

const isFormValid = computed(() => (form.value ? form.value.isValid : false));

export type SpecificUsersFormProps = {
  modelValue: { policyId?: UUID; userIds: UUID[]; prefilledUsers?: BasicUser[] };
  valid?: boolean;
};

const props = withDefaults(defineProps<SpecificUsersFormProps>(), {
  valid: true,
});

const emit = defineEmits<{
  (event: 'update:modelValue', payload: SpecificUsersFormProps['modelValue']): void;
  (event: 'valid', payload: boolean): void;
  (event: 'submit', payload: SpecificUsersFormProps['modelValue']): void;
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
  const users = usersAutocomplete.results.value.map(user => ({
    title: user.name?.[0] ? user.name[0] : user.id,
    value: user.id,
  }));

  (modelValue.prefilledUsers ?? []).forEach(user => {
    if (!users.find(g => g.value === user.id)) {
      users.push({
        title: user.name?.[0] ? user.name[0] : user.id,
        value: user.id,
      });
    }
  });

  props.modelValue.userIds?.forEach(userId => {
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
    emit('submit', modelValue);
  }
};
</script>
