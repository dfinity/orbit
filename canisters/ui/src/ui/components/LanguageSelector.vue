<template>
  <VSelect
    v-model="activeLocale"
    :items="app.supportedLocales"
    variant="solo-filled"
    hide-details
  />
</template>
<script lang="ts" setup>
import { computed } from 'vue';
import { router } from '~/ui/modules';
import { useAppStore } from '~/ui/stores/app';

const app = useAppStore();

const activeLocale = computed({
  get: () => app.locale,
  set: value => {
    router.push({ params: { locale: value } });
    app.useLocale(value, true);
  },
});
</script>
