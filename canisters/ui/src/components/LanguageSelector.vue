<template>
  <VSelect
    v-model="activeLocale"
    :items="app.supportedLocales"
    variant="solo-filled"
    density="compact"
    rounded
    :bg-color="props.bgColor"
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
    bgColor?: string;
  }>(),
  {
    bgColor: 'surface',
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
