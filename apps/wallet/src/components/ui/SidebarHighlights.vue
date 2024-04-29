<template>
  <div class="d-flex flex-grow-1 d-flex-column justify-start align-start">
    <p class="px-4 text-h4 mt-16 text-primary font-weight-bold">
      <i18n-t :keypath="translationKey" scope="global">
        <template v-for="line in lines" :key="line.key" #[line.key]>
          <br />
          <span class="font-weight-regular text-white">
            {{ line.description }}
          </span>
        </template>
      </i18n-t>
    </p>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';

const translationKey = 'sidebar.highlights.main';
const totalLines = 3;
const i18n = useI18n();
const lines = computed<{ key: string; description: string }[]>(() =>
  Array.from({ length: totalLines }, (_, index) => ({
    key: `line${index + 1}`,
    description: i18n.t(`sidebar.highlights.line${index + 1}`),
  })),
);
</script>
