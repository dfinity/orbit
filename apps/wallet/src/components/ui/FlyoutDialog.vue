<template>
  <VDialog
    v-model="open"
    fullscreen
    :persistent="!props.closable"
    :max-width="mobile ? '100%' : props.maxWidth"
    transition="slide-x-reverse-transition"
    class="v-dialog--right"
  >
    <VCard>
      <VToolbar color="background">
        <VToolbarTitle class="px-2">
          <slot name="title">{{ props.title }}</slot>
        </VToolbarTitle>
        <VBtn :disabled="!props.closable" :icon="mdiClose" @click="open = false" />
      </VToolbar>
      <VDivider />
      <VCardText v-if="$slots.content">
        <slot name="content">{{ props.content }}</slot>
      </VCardText>
      <template v-if="$slots.actions">
        <VDivider />
        <VCardActions>
          <slot name="actions"></slot>
        </VCardActions>
      </template>

      <slot name="default"></slot>
    </VCard>
  </VDialog>
</template>
<script lang="ts" setup>
import { mdiClose } from '@mdi/js';
import { computed } from 'vue';
import { useDisplay } from 'vuetify';

const props = withDefaults(
  defineProps<{
    modelValue: boolean;
    title: string;
    content?: string;
    closable?: boolean;
    maxWidth?: string;
  }>(),
  {
    modelValue: false,
    content: '',
    closable: true,
    maxWidth: '400px',
  },
);

const emit = defineEmits<{
  (event: 'update:modelValue', payload: boolean): void;
}>();

const { mobile } = useDisplay();

const open = computed({
  get: () => props.modelValue,
  set: value => emit('update:modelValue', value),
});
</script>
