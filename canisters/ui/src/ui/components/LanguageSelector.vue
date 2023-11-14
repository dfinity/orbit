<template>
  <VSelect
    v-model="activeLocale"
    :items="settings.supportedLocales"
    variant="solo-filled"
    hide-details
  />
</template>
<script lang="ts" setup>
import { computed } from 'vue';
import { router } from '~/ui/modules/Router';
import { useSettingsStore } from '~/ui/stores';

const settings = useSettingsStore();

const activeLocale = computed({
  get: () => settings.locale,
  set: value => {
    router.push({ params: { locale: value } });
    settings.useLocale(value, true);
  },
});
</script>
