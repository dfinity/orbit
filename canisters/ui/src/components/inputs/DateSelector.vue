<template>
  <VCard>
    <VCardText class="py-0 px-2 d-flex justify-center">
      <VDatePicker
        v-model="modelValue"
        :max="props.max?.toDateString()"
        @update:model-value="$emit('close')"
      />
    </VCardText>
    <VCardActions class="px-4 py-4">
      <VSpacer />
      <VBtn variant="outlined" density="comfortable" @click="$emit('update:modelValue', undefined)">
        {{ $t('terms.clear') }}
      </VBtn>
      <VBtn color="primary" variant="flat" density="comfortable" @click="$emit('close')">
        {{ $t('terms.close') }}
      </VBtn>
    </VCardActions>
  </VCard>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { VBtn, VCard, VCardActions, VCardText, VDatePicker, VSpacer } from 'vuetify/components';

const props = defineProps<{
  modelValue?: Date;
  max?: Date;
}>();

const emit = defineEmits<{
  (event: 'update:modelValue', payload?: Date): void;
  (event: 'close'): void;
}>();

const modelValue = computed({
  get: () => props.modelValue,
  set: value => emit('update:modelValue', value),
});
</script>
