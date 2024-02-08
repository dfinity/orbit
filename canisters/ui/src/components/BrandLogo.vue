<template>
  <div class="brand">
    <RouterLink to="/">
      <img :src="brandLogoImage" draggable="false" />
      <span>{{ app.appName }}</span>
    </RouterLink>
  </div>
</template>

<script lang="ts" setup>
import { computed } from 'vue';
import { useAppStore } from '~/stores/app.store';

const app = useAppStore();

const props = withDefaults(
  defineProps<{
    variation?: 'themed' | 'dark';
  }>(),
  {
    variation: 'themed',
  },
);

const brandLogoImage = computed(() => {
  if (props.variation === 'dark') {
    return `/images/app-logo-dark.png`;
  }

  return app.isDarkTheme ? `/images/app-logo-dark.png` : `/images/app-logo-light.png`;
});
</script>

<style scoped lang="scss">
.brand {
  font-size: var(--ds-font-size-xl);
  height: var(--ds-toolbar-height);
  line-height: var(--ds-toolbar-height);

  a {
    display: inline-block;
    text-decoration: none;
    height: 100%;
    color: inherit;
    vertical-align: middle;
  }

  img {
    min-width: var(--ds-toolbar-height);
    height: 100%;
    display: inline-block;
    vertical-align: middle;
    padding: calc(var(--ds-bdu) / 2);
  }

  span {
    height: 100%;
    display: inline-block;
    vertical-align: middle;
  }
}
</style>
