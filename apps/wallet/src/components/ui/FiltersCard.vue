<template>
  <VCard
    v-if="!app.isMobile"
    :min-height="props.minHeight"
    :min-width="props.minWidth"
    :max-width="props.maxWidth"
    class="d-flex flex-column"
    v-bind="$attrs"
  >
    <VToolbar color="transparent" class="pr-4">
      <VToolbarTitle>{{ props.title }}</VToolbarTitle>
      <VIcon v-if="props.icon" :icon="props.icon" />
    </VToolbar>
    <VCardText class="pt-0 d-flex flex-column flex-grow-1">
      <slot></slot>
    </VCardText>
  </VCard>

  <template v-else>
    <VBtn class="text-caption" :append-icon="props.icon" v-bind="$attrs" @click.stop="open = true">
      <span class="text-h6 font-weight-regular">{{ props.title }}</span>
    </VBtn>

    <VDialog v-model="open" fullscreen persistent transition="dialog-bottom-transition" scrollable>
      <VCard @click.stop="">
        <VToolbar color="transparent">
          <VToolbarTitle>{{ props.title }} <VIcon :icon="props.icon" /></VToolbarTitle>
        </VToolbar>
        <VCardText class="py-0">
          <slot></slot>
        </VCardText>
        <VCardActions>
          <VSpacer />
          <VBtn
            color="primary"
            variant="flat"
            density="comfortable"
            class="ma-4"
            @click="open = false"
          >
            {{ $t('terms.close') }}
          </VBtn>
        </VCardActions>
      </VCard>
    </VDialog>
  </template>
</template>
<script lang="ts" setup>
import { ref } from 'vue';
import {
  VBtn,
  VCard,
  VCardActions,
  VCardText,
  VDialog,
  VIcon,
  VSpacer,
  VToolbar,
  VToolbarTitle,
} from 'vuetify/components';
import { useAppStore } from '~/stores/app.store';

const props = withDefaults(
  defineProps<{
    title: string;
    icon?: string;
    minHeight?: string;
    minWidth?: string;
    maxWidth?: string;
  }>(),
  {
    icon: undefined,
    minHeight: '172px',
    minWidth: '272px',
    maxWidth: '272px',
  },
);

const app = useAppStore();
const open = ref(false);
</script>
