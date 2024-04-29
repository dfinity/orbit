<template>
  <div class="d-flex">
    <VProgressCircular indeterminate color="primary" class="mr-2" :size="20" :width="2" />
    <span class="text-medium-emphasis">{{ loadingMessage }}</span>
  </div>
</template>
<script setup lang="ts">
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import { VProgressCircular } from 'vuetify/components';

const props = withDefaults(
  defineProps<{
    message?: string;
  }>(),
  {
    message: undefined,
  },
);

const i18n = useI18n();

const loadingMessage = computed(() => props.message || i18n.t('app.loading_details'));
</script>
