<template>
  <VDialog v-model="open" width="600">
    <ErrorCard
      :title="title"
      :error="error"
      :error-details="props.errorDetails"
      @close="open = false"
    />
  </VDialog>
</template>
<script setup lang="ts">
import { computed, ref } from 'vue';
import { useI18n } from 'vue-i18n';
import { VDialog } from 'vuetify/components';
import ErrorCard from './ErrorCard.vue';

const props = withDefaults(
  defineProps<{
    modelValue?: boolean;
    title?: string;
    error: string;
    errorDetails?: string;
  }>(),
  {
    modelValue: undefined,
    title: undefined,
    error: '',
    errorDetails: undefined,
  },
);

const emit = defineEmits<{
  (event: 'update:modelValue', payload: boolean): void;
}>();

// Used to control the dialog visibility when the model value is not provided.
const defaultOpenedHandler = ref(true);

const open = computed({
  get: () => props.modelValue ?? defaultOpenedHandler.value,
  set: value => {
    if (props.modelValue !== undefined) {
      emit('update:modelValue', value);
    } else {
      defaultOpenedHandler.value = value;
    }
  },
});

const i18n = useI18n();

const title = computed(() => props.title || i18n.t('app.error_dialog_title'));
const error = computed(() => props.error || i18n.t('app.error_dialog_message'));
</script>
