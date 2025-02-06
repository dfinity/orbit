<template>
  <span class="font-weight-bold">{{ name }}</span>
</template>

<script setup lang="ts">
defineProps<{
  id: string;
  name?: string;
}>();
</script>
