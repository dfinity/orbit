<template>
  <div class="d-flex flex-row flex-no-wrap justify-space-between">
    <VAvatar class="ma-3 align-self-center" size="80px" rounded="0">
      <slot name="icon">
        <VIcon size="100%" :icon="props.icon" />
      </slot>
    </VAvatar>
    <div class="flex-grow-1 my-4">
      <VCardTitle class="text-h4 text-wrap">
        <slot name="title">
          {{ title }}
        </slot>
      </VCardTitle>
      <VCardSubtitle class="text-wrap">
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
            color="primary-variant mx-2"
            variant="tonal"
            size="small"
            :prepend-icon="mdiHome"
            :to="{
              name: defaultHomeRoute,
            }"
          >
            {{ $t('app.btn_home_back') }}
          </VBtn>
          <VBtn
            v-if="props.showDisasterRecovery"
            color="warning mx-2"
            variant="tonal"
            size="small"
            :prepend-icon="mdiLifebuoy"
            :to="{
              name: Routes.DisasterRecovery,
            }"
          >
            {{ $t('app.disaster_recovery') }}
          </VBtn>
        </slot>
      </VCardActions>
    </div>
  </div>
</template>

<script setup lang="ts">
import { mdiAlertCircle, mdiHome, mdiLifebuoy } from '@mdi/js';
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
import { Routes, defaultHomeRoute } from '~/configs/routes.config';

const props = withDefaults(
  defineProps<{
    title?: string;
    subtitle?: string;
    icon?: string;
    showBackToHome?: boolean;
    showDisasterRecovery?: boolean;
  }>(),
  {
    title: '',
    subtitle: '',
    icon: mdiAlertCircle,
    showBackToHome: true,
    showDisasterRecovery: false,
  },
);

const i18n = useI18n();
const title = computed(() => props.title || i18n.t('pages.error.title'));
const subtitle = computed(() => props.subtitle || i18n.t('pages.error.subtitle'));
</script>
