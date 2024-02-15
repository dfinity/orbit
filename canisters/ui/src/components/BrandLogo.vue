<template>
  <div class="pb-0">
    <RouterLink to="/">
      <img :src="brandLogoImage" draggable="false" :height="props.height" />
    </RouterLink>
  </div>
</template>

<script lang="ts" setup>
const props = withDefaults(
  defineProps<{
    variation?: 'themed' | 'dark';
    height?: string;
  }>(),
  {
    variation: 'themed',
    height: '36px',
  },
);

const brandLogoImage = `/images/logo.svg`;
</script>
