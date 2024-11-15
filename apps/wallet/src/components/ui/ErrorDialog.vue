<template>
  <VDialog v-model="open" width="600">
    <ErrorCard :title="title" :error="error" :error-details="props.errorDetails" />
  </VDialog>
</template>
<script setup lang="ts">
import { computed } from 'vue';
import { VDialog } from 'vuetify/components';
import ErrorCard from './ErrorCard.vue';
import { useI18n } from 'vue-i18n';

const props = withDefaults(
  defineProps<{
    modelValue?: boolean;
    title?: string;
    error: string;
    errorDetails?: string;
  }>(),
  {
    modelValue: true,
    title: undefined,
    error: '',
    errorDetails: undefined,
  },
);

const emit = defineEmits<{
  (event: 'update:modelValue', payload: boolean): void;
}>();

const open = computed({
  get: () => props.modelValue,
  set: value => emit('update:modelValue', value),
});

const i18n = useI18n();

const title = computed(() => props.title || i18n.t('app.error_dialog_title'));
const error = computed(() => props.error || i18n.t('app.error_dialog_message'));
</script>
