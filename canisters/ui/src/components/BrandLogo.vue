<template>
  <div class="pb-0" :style="props.color ? { color: props.color } : undefined">
    <RouterLink to="/">
      <Logo draggable="false" :height="props.height" />
    </RouterLink>
  </div>
</template>

<script lang="ts" setup>
import Logo from '~/components/LogoImage.vue';

const props = withDefaults(
  defineProps<{
    variation?: 'themed' | 'dark';
    height?: string;
    color?: string;
  }>(),
  {
    variation: 'themed',
    height: '36px',
    color: undefined,
  },
);
</script>
