<template>
  <div class="operation-item__code__title">{{ operation.code }}</div>
  <div class="operation-item__code__time">
    <VChip size="x-small" :title="operation.created_at">
      <VIcon :icon="mdiClockOutline" size="x-small" />&nbsp;
      {{ new Date(operation.created_at).toLocaleDateString() }}
    </VChip>
  </div>
</template>
<script lang="ts" setup>
import { mdiClockOutline } from '@mdi/js';
import { computed } from 'vue';
import { Operation } from '~/generated/bank/bank.did';

const props = defineProps<{
  modelValue: Operation;
}>();

const emit = defineEmits<{
  (event: 'update:modelValue', payload: Operation): void;
  (event: 'read', payload: boolean): void;
}>();

const operation = computed({
  get: () => props.modelValue,
  set: value => emit('update:modelValue', value),
});
</script>
