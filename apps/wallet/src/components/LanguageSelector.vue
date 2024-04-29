<template>
  <VSelect
    v-model="activeLocale"
    bg-color="transparent"
    :style="{
      ['max-width']: props.maxWidth,
    }"
    :items="app.supportedLocales"
    variant="solo-filled"
    density="compact"
    rounded
    hide-details
    flat
  />
</template>
<script lang="ts" setup>
import { computed } from 'vue';
import { useRouter } from 'vue-router';
import { VSelect } from 'vuetify/components';
import { useAppStore } from '~/stores/app.store';

const props = withDefaults(
  defineProps<{
    maxWidth?: string;
  }>(),
  {
    maxWidth: '90px',
  },
);

const app = useAppStore();
const router = useRouter();

const activeLocale = computed({
  get: () => app.locale,
  set: value => {
    router.push({ params: { locale: value } });
    app.useLocale(value, true);
  },
});
</script>
