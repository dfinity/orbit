<template>
  <VAutocomplete
    v-model="model"
    v-bind="$attrs"
    class="mt-2"
    name="account_id"
    :label="$t('terms.account')"
    :loading="autocomplete.loading.value"
    :items="accountList"
    chips
    clearable
    :rules="[requiredRule]"
    :variant="isViewMode ? 'plain' : 'filled'"
    :disabled="isViewMode"
    @update:search="autocomplete.searchItems"
  />
</template>

<script lang="ts" setup>
import { computed, onMounted } from 'vue';
import { useAccountsAutocomplete } from '~/composables/autocomplete.composable';
import { UUID } from '~/generated/station/station.did';
import { requiredRule } from '~/utils/form.utils';

const autocomplete = useAccountsAutocomplete();

const props = withDefaults(
  defineProps<{
    modelValue: UUID | null;
    valid?: boolean;
    triggerSubmit?: boolean;
    mode?: 'view' | 'edit';
  }>(),
  {
    valid: true,
    triggerSubmit: false,
    mode: 'edit',
  },
);

const emit = defineEmits<{
  (event: 'update:modelValue', payload: UUID | null): void;
}>();

const model = computed({
  get: () => props.modelValue,
  set: value => emit('update:modelValue', value),
});

const accountList = computed(() => {
  const accounts = autocomplete.results.value.map(group => ({
    title: group.name,
    value: group.id,
  }));

  return accounts;
});

const isViewMode = computed(() => props.mode === 'view');

onMounted(() => {
  autocomplete.searchItems();
});
</script>
