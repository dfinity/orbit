<template>
  <div class="d-flex flex-no-wrap justify-space-between">
    <VAvatar class="ma-3" size="180" rounded="0">
      <slot name="icon">
        <VIcon size="100%" color="primary" :icon="props.icon" />
      </slot>
    </VAvatar>
    <div class="flex-grow-1 my-8">
      <VCardTitle class="text-h4">
        <slot name="title">
          {{ title }}
        </slot>
      </VCardTitle>
      <VCardSubtitle>
        <slot name="subtitle">
          {{ subtitle }}
        </slot>
      </VCardSubtitle>
      <VCardText>
        <slot></slot>
      </VCardText>

      <VCardActions>
        <slot name="actions">
          <VBtn
            v-if="props.showBackToHome"
            color="primary-variant mt-8 mx-2"
            variant="tonal"
            size="small"
            :prepend-icon="mdiHome"
            :to="{
              name: defaultHomeRoute,
            }"
          >
            {{ $t('app.btn_home_back') }}
          </VBtn>
        </slot>
      </VCardActions>
    </div>
  </div>
</template>

<script setup lang="ts">
import { mdiAlertCircle, mdiHome } from '@mdi/js';
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import {
  VAvatar,
  VBtn,
  VCardActions,
  VCardSubtitle,
  VCardText,
  VCardTitle,
  VIcon,
} from 'vuetify/components';
import { defaultHomeRoute } from '~/configs/routes.config';

const props = withDefaults(
  defineProps<{
    title?: string;
    subtitle?: string;
    icon?: string;
    showBackToHome?: boolean;
  }>(),
  {
    title: '',
    subtitle: '',
    icon: mdiAlertCircle,
    showBackToHome: true,
  },
);

const i18n = useI18n();
const title = computed(() => props.title || i18n.t('pages.error.title'));
const subtitle = computed(() => props.subtitle || i18n.t('pages.error.subtitle'));
</script>
