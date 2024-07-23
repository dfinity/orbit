<template>This component updates the canister from the registry.</template>

<script lang="ts" setup>
import { computed } from 'vue';
import { ChangeCanisterFormValue } from './change-canister.types';

const props = defineProps<{
  modelValue: ChangeCanisterFormValue;
}>();

const emit = defineEmits<{
  (event: 'update:modelValue', payload: ChangeCanisterFormValue): void;
}>();

const _modelValue = computed({
  get: () => props.modelValue,
  set: value => emit('update:modelValue', value),
});
</script>
