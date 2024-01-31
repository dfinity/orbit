<template>
  <VSelect
    v-model="activeLocale"
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
import { useAppStore } from '~/ui/stores/app';

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
